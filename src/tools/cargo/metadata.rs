use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
};
use rmcp::ErrorData;
#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoMetadataRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Only include resolve dependencies matching the given target-triple
    #[serde(default, deserialize_with = "deserialize_string")]
    filter_platform: Option<String>,

    /// Output information only about the workspace members and don't fetch dependencies
    #[serde(default)]
    no_deps: Option<bool>,

    /// Use verbose output (-vv very verbose/build.rs output)
    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,

    /// Override a configuration value
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    locking_mode: Option<String>,
}
impl CargoMetadataRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("metadata");
        cmd.arg("--format-version").arg("1");

        // Package/dependency filtering
        if let Some(triple) = &self.filter_platform {
            cmd.arg("--filter-platform").arg(triple);
        }

        if self.no_deps.unwrap_or(false) {
            cmd.arg("--no-deps");
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        // Feature selection
        if let Some(features) = &self.features
            && !features.is_empty()
        {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }

        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        cmd.args(locking_flags);

        Ok(cmd)
    }
}

pub struct CargoMetadataRmcpTool;

impl Tool for CargoMetadataRmcpTool {
    const NAME: &'static str = "cargo-metadata";
    const TITLE: &'static str = "cargo metadata";
    const DESCRIPTION: &'static str = "Outputs a listing of a project's resolved dependencies and metadata in machine-readable format (JSON).";
    type RequestArgs = CargoMetadataRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_features_array() {
        let input = json!({
            "features": ["serde", "tokio"],
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with features array");

        assert_eq!(
            tool.features.unwrap(),
            ["serde".to_owned(), "tokio".to_owned()]
        );
    }

    #[test]
    fn test_deserialize_with_single_feature_string() {
        let input = json!({
            "features": "serde",
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with single feature string");

        assert_eq!(tool.features.unwrap(), ["serde".to_owned()]);
    }

    #[test]
    fn test_deserialize_with_features_string_array() {
        let input = json!({
            "features": "[\"serde\",\"tokio\"]",
        });

        let tool: Result<CargoMetadataRequest, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed with features string that looks like array");

        assert_eq!(tool.features.unwrap(), ["[\"serde\",\"tokio\"]".to_owned()]);
    }
}
