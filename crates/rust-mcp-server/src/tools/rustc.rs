use std::process::Command;

use crate::{Tool, execute_command, serde_utils::deserialize_string};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct RustcExplainRequest {
    /// The Rust compiler error code to explain (e.g., "E0001", "E0308", "E0432")
    pub error_code: String,

    /// The toolchain to use for rustc (e.g., "stable", "nightly", "1.70.0")
    #[serde(default, deserialize_with = "deserialize_string")]
    pub toolchain: Option<String>,
}

impl RustcExplainRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("rustc");

        if let Some(toolchain) = &self.toolchain {
            cmd = Command::new("rustup");
            cmd.arg("run").arg(toolchain).arg("rustc");
        }

        cmd.arg("--explain").arg(&self.error_code);

        Ok(cmd)
    }
}

pub struct RustcExplainRmcpTool;

impl Tool for RustcExplainRmcpTool {
    const NAME: &'static str = "rustc-explain";
    const TITLE: &'static str = "Explain Rust error";
    const DESCRIPTION: &'static str = "Provide a detailed explanation of a Rust compiler error code. This tool allows AI agents to request more information about compilation errors by providing the error code (e.g., E0001, E0308, etc.). Very useful for understanding and resolving Rust compilation errors.";
    type RequestArgs = RustcExplainRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
