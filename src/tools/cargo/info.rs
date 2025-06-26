use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{default_true, deserialize_string};
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-info",
    description = "Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoInfoTool {
    /// Package to inspect
    pub spec: String,

    /// Package version
    #[serde(default, deserialize_with = "deserialize_string")]
    pub version: Option<String>,

    /// Registry to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl CargoInfoTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("info");
        if let Some(version) = self.version.as_ref() {
            cmd.arg(format!("{}@{version}", self.spec));
        } else {
            cmd.arg(&self.spec);
        }

        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}
