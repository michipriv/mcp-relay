// Filename: prg/handler.rs
// V 1.2 2024-12-28 Removed J prefix from output messages
// V 1.1 2024-12-28 Relay numbering changed from 2,3,4,5 to 1,2,3,4
// V 1.0 Initial version

use relay::RelayBoard;
use rmcp::{ServerHandler, handler::server::{router::tool::ToolRouter, wrapper::Parameters}, model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, schemars, tool, tool_handler, tool_router, ErrorData as McpError};
use std::sync::Arc;
use tokio::sync::Mutex;

//*********************************
//  Request structures
//*********************************
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RelayOnRequest { pub relay: u8 }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RelayOffRequest { pub relay: u8 }

//*********************************
//  RelayHandler structure
//*********************************
#[derive(Clone)]
pub struct RelayHandler {
    board: Arc<Mutex<Option<RelayBoard>>>,
    tool_router: ToolRouter<Self>
}

#[tool_router]
impl RelayHandler {
    //*********************************
    //  Create new handler
    //*********************************
    pub fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(None)),
            tool_router: Self::tool_router()
        }
    }

    //*********************************
    //  Ensure relay board is initialized
    //*********************************
    async fn ensure_board(&self) -> Result<(), McpError> {
        let mut b = self.board.lock().await;
        if b.is_none() {
            *b = Some(RelayBoard::new().map_err(|e| McpError::internal_error(format!("{}", e), None))?);
        }
        Ok(())
    }

    //*********************************
    //  Turn relay ON (relay 1-4)
    //*********************************
    #[tool(description = "Turn relay ON")]
    async fn relay_on(&self, Parameters(RelayOnRequest { relay }): Parameters<RelayOnRequest>) -> Result<CallToolResult, McpError> {
        self.ensure_board().await?;
        self.board.lock().await.as_mut().unwrap().relay_on(relay).map_err(|e| McpError::internal_error(format!("{}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(format!("Relay {} ON", relay))]))
    }

    //*********************************
    //  Turn relay OFF (relay 1-4)
    //*********************************
    #[tool(description = "Turn relay OFF")]
    async fn relay_off(&self, Parameters(RelayOffRequest { relay }): Parameters<RelayOffRequest>) -> Result<CallToolResult, McpError> {
        self.ensure_board().await?;
        self.board.lock().await.as_mut().unwrap().relay_off(relay).map_err(|e| McpError::internal_error(format!("{}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(format!("Relay {} OFF", relay))]))
    }

    //*********************************
    //  Turn all relays OFF
    //*********************************
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

// EOF
