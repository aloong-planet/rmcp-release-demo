//! A stdio MCP server with two tools, `echo` and `version`.
//!
//! stdout carries only MCP messages; everything else goes to stderr.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, ServiceExt, tool, tool_handler, tool_router, transport};
use schemars::JsonSchema;
use serde::Deserialize;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, JsonSchema)]
struct EchoParams {
    /// The text to return.
    text: String,
}

#[derive(Clone)]
struct Server {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Server {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Return the given text unchanged.")]
    fn echo(&self, Parameters(EchoParams { text }): Parameters<EchoParams>) -> String {
        text
    }

    #[tool(description = "Return the server's version.")]
    fn version(&self) -> String {
        VERSION.to_owned()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    // Written out because the generated one reports rmcp's own name and version.
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(NAME, VERSION))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("{NAME} {VERSION} serving MCP on stdio");
    let service = Server::new().serve(transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

// Probe, never merged: breaks the x86_64-unknown-linux-musl build only.
#[cfg(all(target_arch = "x86_64", target_env = "musl"))]
compile_error!("probe: break the x86_64-unknown-linux-musl build only");
