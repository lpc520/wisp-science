//! Minimal stdio JSON-RPC MCP client + `McpTool` adapter.
//!
//! Launch a server with [`McpClient::launch`], enumerate its tools with
//! [`McpClient::tools_list`], and wrap each as an [`McpTool`] to register with
//! the agent's tool registry. Example (bio-tools):
//!
//! ```ignore
//! let client = Arc::new(McpClient::launch("python",
//!     &["../mcp-servers/bio-tools/run_server.py", "mcp_pubmed"]).await?);
//! for t in client.tools_list().await? {
//!     registry.add(Box::new(McpTool::new(t, client.clone())));
//! }
//! ```

pub mod client;
pub mod tool;

pub use client::{bundled_bio_tools_dir, McpClient, RemoteTool};
pub use tool::McpTool;
