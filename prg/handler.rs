use relay::RelayBoard;
use rmcp::{ServerHandler, handler::server::{router::tool::ToolRouter, wrapper::Parameters}, model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, schemars, tool, tool_handler, tool_router, ErrorData as McpError};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RelayOnRequest { pub relay: u8 }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RelayOffRequest { pub relay: u8 }

#[derive(Clone)]
pub struct RelayHandler {
    board: Arc<Mutex<Option<RelayBoard>>>,
    tool_router: ToolRouter<Self>
}

#[tool_router]
impl RelayHandler {
    pub fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(None)),
            tool_router: Self::tool_router()
        }
    }

    async fn ensure_board(&self) -> Result<(), McpError> {
        let mut b = self.board.lock().await;
        if b.is_none() {
            *b = Some(RelayBoard::new().map_err(|e| McpError::internal_error(format!("{}", e), None))?);
        }
        Ok(())
    }

    #[tool(description = "Turn relay ON")]
    async fn relay_on(&self, Parameters(RelayOnRequest { relay }): Parameters<RelayOnRequest>) -> Result<CallToolResult, McpError> {
        self.ensure_board().await?;
        self.board.lock().await.as_mut().unwrap().relay_on(relay).map_err(|e| McpError::internal_error(format!("{}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(format!("J{} ON", relay))]))
    }

    #[tool(description = "Turn relay OFF")]
    async fn relay_off(&self, Parameters(RelayOffRequest { relay }): Parameters<RelayOffRequest>) -> Result<CallToolResult, McpError> {
        self.ensure_board().await?;
        self.board.lock().await.as_mut().unwrap().relay_off(relay).map_err(|e| McpError::internal_error(format!("{}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(format!("J{} OFF", relay))]))
    }

    #[tool(description = "All OFF")]
    async fn relay_all_off(&self) -> Result<CallToolResult, McpError> {
        self.ensure_board().await?;
        self.board.lock().await.as_mut().unwrap().all_off().map_err(|e| McpError::internal_error(format!("{}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text("All OFF")]))
    }
}

#[tool_handler]
impl ServerHandler for RelayHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Relay control".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
