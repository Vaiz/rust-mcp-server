use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::deserialize_string;
use crate::tools::execute_command;

#[mcp_tool(
    name = "rustc-explain",
    description = "Provide a detailed explanation of a Rust compiler error code. This tool allows AI agents to request more information about compilation errors by providing the error code (e.g., E0001, E0308, etc.). Very useful for understanding and resolving Rust compilation errors.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct RustcExplainTool {
    /// The Rust compiler error code to explain (e.g., "E0001", "E0308", "E0432")
    pub error_code: String,

    /// The toolchain to use for rustc (e.g., "stable", "nightly", "1.70.0")
    #[serde(default, deserialize_with = "deserialize_string")]
    pub toolchain: Option<String>,
}

impl RustcExplainTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("rustc");

        if let Some(toolchain) = &self.toolchain {
            cmd = Command::new("rustup");
            cmd.arg("run").arg(toolchain).arg("rustc");
        }

        cmd.arg("--explain").arg(&self.error_code);

        execute_command(cmd, &Self::tool_name())
    }
}
