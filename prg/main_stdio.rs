// Filename: prg/main.rs
// V 4.0 2024-12-28 Stdio transport (no HTTP/SSE)

use anyhow::Result;
use rmcp::service::Service;
use rmcp::transport::stdio::StdioTransport;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
use handler::RelayHandler;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting MCP-Relay-Server (stdio)");

    let handler = RelayHandler::new();
    let transport = StdioTransport::new();
    let ct = tokio_util::sync::CancellationToken::new();

    let service = Service::new(handler, transport, ct.clone());
    
    service.serve().await?;

    Ok(())
}

// EOF
