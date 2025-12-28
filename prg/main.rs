// Filename: prg/main.rs
// V 2.1 2024-12-28 Fixed path parameters for Axum 0.8
// V 2.0 2024-12-28 Complete REST API rewrite, removed MCP dependencies

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use relay::RelayBoard;

//*********************************
//  Configuration structure
//*********************************
#[derive(Debug, Deserialize)]
struct Config {
    auth_token: String,
    bind_address: String,
    log_level: String,
}

impl Config {
    fn load() -> anyhow::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "/data/relay/config.json".to_string());
        let config_str = std::fs::read_to_string(&config_path)?;
        Ok(serde_json::from_str(&config_str)?)
    }
}

//*********************************
//  Application state
//*********************************
#[derive(Clone)]
struct AppState {
    board: Arc<Mutex<Option<RelayBoard>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(None)),
        }
    }

    async fn ensure_board(&self) -> Result<(), String> {
        let mut board = self.board.lock().await;
        if board.is_none() {
            *board = Some(RelayBoard::new().map_err(|e| e.to_string())?);
        }
        Ok(())
    }
}

//*********************************
//  Response structures
//*********************************
#[derive(Serialize)]
struct RelayResponse {
    relay: u8,
    state: String,
}

#[derive(Serialize)]
struct AllOffResponse {
    message: String,
}

#[derive(Serialize)]
struct RelayStatusItem {
    id: u8,
    state: String,
}

#[derive(Serialize)]
struct StatusResponse {
    relays: Vec<RelayStatusItem>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

//*********************************
//  Handler: Turn relay ON
//*********************************
async fn relay_on(
    State(state): State<AppState>,
    Path(id): Path<u8>,
) -> Result<Json<RelayResponse>, (StatusCode, String)> {
    if !(1..=4).contains(&id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid relay ID".to_string()));
    }

    state.ensure_board().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let board = state.board.lock().await;
    board.as_ref().unwrap().relay_on(id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RelayResponse {
        relay: id,
        state: "on".to_string(),
    }))
}

//*********************************
//  Handler: Turn relay OFF
//*********************************
async fn relay_off(
    State(state): State<AppState>,
    Path(id): Path<u8>,
) -> Result<Json<RelayResponse>, (StatusCode, String)> {
    if !(1..=4).contains(&id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid relay ID".to_string()));
    }

    state.ensure_board().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let board = state.board.lock().await;
    board.as_ref().unwrap().relay_off(id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RelayResponse {
        relay: id,
        state: "off".to_string(),
    }))
}

//*********************************
//  Handler: Turn all relays OFF
//*********************************
async fn all_off(
    State(state): State<AppState>,
) -> Result<Json<AllOffResponse>, (StatusCode, String)> {
    state.ensure_board().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let board = state.board.lock().await;
    board.as_ref().unwrap().all_off()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AllOffResponse {
        message: "all relays off".to_string(),
    }))
}

//*********************************
//  Handler: Get status of all relays
//*********************************
async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<StatusResponse>, (StatusCode, String)> {
    state.ensure_board().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let board = state.board.lock().await;
    let mut relays = Vec::new();

    for id in 1..=4 {
        let value = board.as_ref().unwrap().get_state(id)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        relays.push(RelayStatusItem {
            id,
            state: if value == 1 { "on" } else { "off" }.to_string(),
        });
    }

    Ok(Json(StatusResponse { relays }))
}

//*********************************
//  Handler: Health check
//*********************************
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

//*********************************
//  Authentication middleware
//*********************************
async fn auth_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
    token: &str,
) -> Result<axum::response::Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    if let Some(auth) = auth_header {
        if auth == format!("Bearer {}", token) {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

//*********************************
//  Main entry point
//*********************************
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting Relay REST API Server");
    tracing::info!("Config loaded from: {}", 
        std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/data/relay/config.json".to_string()));
    tracing::info!("Listening on http://{}", config.bind_address);

    let state = AppState::new();
    let auth_token = config.auth_token.clone();

    let app = Router::new()
        .route("/relay/{id}/on", post(relay_on))
        .route("/relay/{id}/off", post(relay_off))
        .route("/relay/all/off", post(all_off))
        .route("/relay/status", get(get_status))
        .route("/health", get(health))
        .with_state(state)
        .layer(axum::middleware::from_fn(move |req, next| {
            let token = auth_token.clone();
            async move { auth_middleware(req, next, &token).await }
        }));

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        .await?;

    Ok(())
}

// EOF
