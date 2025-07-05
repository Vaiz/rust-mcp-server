use std::process::Command;

use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{PackageWithVersion, default_true, deserialize_string};
use crate::tools::execute_command;

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-info",
    description = "Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoInfoTool {
    /// Package with optional version (e.g., {"package": "serde", "version": "1.0.0"})
    #[serde(flatten)]
    pub package_spec: PackageWithVersion,

    /// Registry index URL to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub index: Option<String>,

    /// Registry to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,

    /// Use verbose output that includes crate dependencies.
    #[serde(default)]
    pub verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,

    /// Override a configuration value
    #[serde(default, deserialize_with = "deserialize_string")]
    pub config: Option<String>,

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    pub locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    pub frozen: bool,
}

impl CargoInfoTool {
    pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
        use schemars::schema_for;
        let schema = schema_for!(CargoInfoTool).to_value();
        if let serde_json::Value::Object(mut map) = schema {
            map.remove("$schema");
            map
        } else {
            panic!("Expected schema to be an object, got: {schema:?}");
        }
    }

    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("info");

        cmd.arg(self.package_spec.to_spec());

        if let Some(index) = &self.index {
            cmd.arg("--index").arg(index);
        }

        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        // Manifest options
        if self.locked {
            cmd.arg("--locked");
        }

        if self.offline {
            cmd.arg("--offline");
        }

        if self.frozen {
            cmd.arg("--frozen");
        }

        // Output options
        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.quiet && !self.verbose {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}
