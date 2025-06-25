use std::process::Command;

use crate::{serde_utils::deserialize_string, tools::execute_command};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-metadata",
    description = "Outputs a listing of a project's resolved dependencies and metadata in machine-readable format (JSON).",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoMetadataTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Only include resolve dependencies matching the given target-triple
    #[serde(deserialize_with = "deserialize_string")]
    filter_platform: Option<String>,

    /// Output information only about the workspace members and don't fetch dependencies
    #[serde(default)]
    no_deps: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default)]
    quiet: bool,

    /// Coloring [possible values: auto, always, never]
    #[serde(deserialize_with = "deserialize_string")]
    color: Option<String>,

    /// Override a configuration value
    #[serde(deserialize_with = "deserialize_string")]
    config: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(deserialize_with = "deserialize_string")]
    features: Option<String>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: bool,

    /// Path to Cargo.toml
    #[serde(deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,
}

impl CargoMetadataTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("metadata");
        cmd.arg("--locked");
        cmd.arg("--format-version").arg("1");

        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(ref triple) = self.filter_platform {
            cmd.arg("--filter-platform").arg(triple);
        }
        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if self.verbose {
            cmd.arg("--verbose");
        }
        if self.quiet {
            cmd.arg("--quiet");
        }
        if let Some(ref color) = self.color {
            cmd.arg("--color").arg(color);
        }
        if let Some(ref config) = self.config {
            cmd.arg("--config").arg(config);
        }
        if let Some(ref features) = self.features {
            cmd.arg("--features").arg(features);
        }
        if self.all_features {
            cmd.arg("--all-features");
        }
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }
        if let Some(ref lockfile_path) = self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }
        if self.offline {
            cmd.arg("--offline");
        }
        if self.frozen {
            cmd.arg("--frozen");
        }

        execute_command(cmd)
    }
}
