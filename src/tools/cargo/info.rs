use std::process::Command;

use crate::serde_utils::Tool;
use crate::{
    serde_utils::{
        PackageWithVersion, default_true, deserialize_string, locking_mode_to_cli_flags,
    },
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

/// Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.
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

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    pub locking_mode: Option<String>,
}

impl CargoInfoTool {
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
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref())?;
        cmd.args(locking_flags);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_info_schema() {
        const EXPECTED_SCHEMA: &str = r##"{
  "description": "Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.",
  "properties": {
    "config": {
      "default": null,
      "description": "Override a configuration value",
      "type": "string"
    },
    "index": {
      "default": null,
      "description": "Registry index URL to search packages in",
      "type": "string"
    },
    "locking_mode": {
      "default": null,
      "description": "Locking mode for dependency resolution.\n\nValid options:\n- \"locked\" (default): Assert that `Cargo.lock` will remain unchanged\n- \"unlocked\": Allow `Cargo.lock` to be updated\n- \"offline\": Run without accessing the network\n- \"frozen\": Equivalent to specifying both --locked and --offline",
      "type": "string"
    },
    "package": {
      "description": "The package name",
      "type": "string"
    },
    "quiet": {
      "default": true,
      "description": "Do not print cargo log messages. By default is `true`.",
      "type": "boolean"
    },
    "registry": {
      "default": null,
      "description": "Registry to search packages in",
      "type": "string"
    },
    "verbose": {
      "default": false,
      "description": "Use verbose output that includes crate dependencies.",
      "type": "boolean"
    },
    "version": {
      "default": null,
      "description": "Optional version specification",
      "type": "string"
    }
  },
  "required": [
    "package"
  ],
  "title": "CargoInfoTool",
  "type": "object"
}"##;
        let schema = serde_json::Value::from(CargoInfoTool::json_schema());
        println!(
            "CargoInfoTool schema: {}",
            serde_json::to_string_pretty(&schema).unwrap()
        );

        let expected_schema: serde_json::Value = serde_json::from_str(EXPECTED_SCHEMA).unwrap();
        assert_eq!(
            schema, expected_schema,
            "CargoInfoTool schema should match expected structure"
        );
    }
}
