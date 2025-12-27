// Filename: prg/main.rs
// V 3.0 2024-12-27 Config-File Support

use anyhow::Result;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use serde::Deserialize;
use std::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
use handler::RelayHandler;

#[derive(Debug, Deserialize)]
struct Config {
    auth_token: String,
    bind_address: String,
    log_level: String,
}

impl Config {
    fn load() -> Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "/data/relay/config.json".to_string());
        let config_str = fs::read_to_string(&config_path)?;
        Ok(serde_json::from_str(&config_str)?)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting MCP-Relay-Server HTTP/SSE");
    tracing::info!("Config loaded from: {}", 
        std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/data/relay/config.json".to_string()));
    tracing::info!("Listening on http://{}", config.bind_address);

    let ct = tokio_util::sync::CancellationToken::new();
    let auth_token = config.auth_token.clone();

    let service = StreamableHttpService::new(
        || Ok(RelayHandler::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig {
            cancellation_token: ct.child_token(),
            ..Default::default()
        },
    );

    let router = axum::Router::new()
        .nest_service("/mcp", service)
        .layer(axum::middleware::from_fn(move |req, next| {
            let token = auth_token.clone();
            async move { auth_middleware(req, next, &token).await }
        }));

    let tcp_listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await?;

    Ok(())
}

async fn auth_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
    token: &str,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    if let Some(auth) = auth_header {
        if auth == format!("Bearer {}", token) {
            return Ok(next.run(req).await);
        }
    }

    Err(axum::http::StatusCode::UNAUTHORIZED)
}
