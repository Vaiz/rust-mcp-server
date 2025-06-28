use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{deserialize_string, deserialize_string_vec};
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-hack",
    description = "Cargo subcommand to provide various options useful for testing and continuous integration, including feature testing and multi-version compatibility.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoHackTool {
    /// The cargo command to run (e.g., check, test, build, clippy)
    #[serde(default, deserialize_with = "deserialize_string")]
    command: Option<String>,

    /// Package(s) to check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Perform command for all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Exclude packages from the check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Require Cargo.lock is up to date
    #[serde(default)]
    locked: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Perform for each feature of the package
    #[serde(default)]
    each_feature: bool,

    /// Perform for the feature powerset of the package
    #[serde(default)]
    feature_powerset: bool,

    /// Use optional dependencies as features
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    optional_deps: Option<Vec<String>>,

    /// Space or comma separated list of features to exclude
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude_features: Option<Vec<String>>,

    /// Exclude run of just --no-default-features flag
    #[serde(default)]
    exclude_no_default_features: bool,

    /// Exclude run of just --all-features flag
    #[serde(default)]
    exclude_all_features: bool,

    /// Specify a max number of simultaneous feature flags of --feature-powerset
    depth: Option<u32>,

    /// Space or comma separated list of features to group
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    group_features: Option<Vec<String>>,

    /// Build for specified target triple
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    target: Option<Vec<String>>,

    /// Space or comma separated list of features to not use together
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    mutually_exclusive_features: Option<Vec<String>>,

    /// Include only the specified features in the feature combinations
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    include_features: Option<Vec<String>>,

    /// Perform without dev-dependencies
    #[serde(default)]
    no_dev_deps: bool,

    /// Equivalent to --no-dev-deps flag except for does not restore the original Cargo.toml
    #[serde(default)]
    remove_dev_deps: bool,

    /// Perform without `publish = false` crates
    #[serde(default)]
    no_private: bool,

    /// Skip to perform on `publish = false` packages
    #[serde(default)]
    ignore_private: bool,

    /// Skip passing --features flag to cargo if that feature does not exist
    #[serde(default)]
    ignore_unknown_features: bool,

    /// Perform commands on `package.rust-version`
    #[serde(default)]
    rust_version: bool,

    /// Perform commands on a specified (inclusive) range of Rust versions
    #[serde(default, deserialize_with = "deserialize_string")]
    version_range: Option<String>,

    /// Specify the version interval of --version-range (default to 1)
    version_step: Option<u32>,

    /// Remove artifacts for that package before running the command
    #[serde(default)]
    clean_per_run: bool,

    /// Remove artifacts per Rust version
    #[serde(default)]
    clean_per_version: bool,

    /// Keep going on failure
    #[serde(default)]
    keep_going: bool,

    /// Partition runs and execute only its subset according to M/N
    #[serde(default, deserialize_with = "deserialize_string")]
    partition: Option<String>,

    /// Log grouping: none, github-actions
    #[serde(default, deserialize_with = "deserialize_string")]
    log_group: Option<String>,

    /// Print commands without run (Unstable)
    #[serde(default)]
    print_command_list: bool,

    /// Do not pass --manifest-path option to cargo (Unstable)
    #[serde(default)]
    no_manifest_path: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// Coloring: auto, always, never
    #[serde(default, deserialize_with = "deserialize_string")]
    color: Option<String>,
}

impl CargoHackTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("hack");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if self.locked {
            cmd.arg("--locked");
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.each_feature {
            cmd.arg("--each-feature");
        }

        if self.feature_powerset {
            cmd.arg("--feature-powerset");
        }

        if let Some(optional_deps) = &self.optional_deps {
            if optional_deps.is_empty() {
                cmd.arg("--optional-deps");
            } else {
                cmd.arg("--optional-deps").arg(optional_deps.join(","));
            }
        }

        if let Some(exclude_features) = &self.exclude_features {
            cmd.arg("--exclude-features")
                .arg(exclude_features.join(","));
        }

        if self.exclude_no_default_features {
            cmd.arg("--exclude-no-default-features");
        }

        if self.exclude_all_features {
            cmd.arg("--exclude-all-features");
        }

        if let Some(depth) = self.depth {
            cmd.arg("--depth").arg(depth.to_string());
        }

        if let Some(group_features) = &self.group_features {
            cmd.arg("--group-features").arg(group_features.join(","));
        }

        // Target selection
        if let Some(targets) = &self.target {
            for target in targets {
                cmd.arg("--target").arg(target);
            }
        }

        // Feature constraints
        if let Some(mutually_exclusive) = &self.mutually_exclusive_features {
            cmd.arg("--mutually-exclusive-features")
                .arg(mutually_exclusive.join(","));
        }

        if let Some(include_features) = &self.include_features {
            cmd.arg("--include-features")
                .arg(include_features.join(","));
        }

        // Dependency options
        if self.no_dev_deps {
            cmd.arg("--no-dev-deps");
        }

        if self.remove_dev_deps {
            cmd.arg("--remove-dev-deps");
        }

        if self.no_private {
            cmd.arg("--no-private");
        }

        if self.ignore_private {
            cmd.arg("--ignore-private");
        }

        if self.ignore_unknown_features {
            cmd.arg("--ignore-unknown-features");
        }

        // Version options
        if self.rust_version {
            cmd.arg("--rust-version");
        }

        if let Some(version_range) = &self.version_range {
            cmd.arg("--version-range").arg(version_range);
        }

        if let Some(version_step) = self.version_step {
            cmd.arg("--version-step").arg(version_step.to_string());
        }

        // Cleanup options
        if self.clean_per_run {
            cmd.arg("--clean-per-run");
        }

        if self.clean_per_version {
            cmd.arg("--clean-per-version");
        }

        // Execution options
        if self.keep_going {
            cmd.arg("--keep-going");
        }

        if let Some(partition) = &self.partition {
            cmd.arg("--partition").arg(partition);
        }

        if let Some(log_group) = &self.log_group {
            cmd.arg("--log-group").arg(log_group);
        }

        if self.print_command_list {
            cmd.arg("--print-command-list");
        }

        if self.no_manifest_path {
            cmd.arg("--no-manifest-path");
        }

        // Output options
        if self.verbose {
            cmd.arg("--verbose");
        }

        if let Some(color) = &self.color {
            cmd.arg("--color").arg(color);
        }

        // Add the cargo command to run (e.g., check, test, build)
        if let Some(command) = &self.command {
            cmd.arg(command);
        } else {
            // Default to check if no command specified
            cmd.arg("check");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-hack-install",
    description = "Installs cargo-hack tool for feature testing and continuous integration",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoHackInstallTool {}

impl CargoHackInstallTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-hack");

        execute_command(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_basic_cargo_hack_tool() {
        let input = json!({
            "command": "check",
            "workspace": true,
            "each_feature": true,
            "verbose": false
        });

        let tool: Result<CargoHackTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed");

        assert_eq!(tool.command, Some("check".to_string()));
        assert!(tool.workspace);
        assert!(tool.each_feature);
        assert!(!tool.verbose);
        assert!(!tool.feature_powerset);
    }

    #[test]
    fn test_deserialize_feature_powerset_options() {
        let input = json!({
            "feature_powerset": true,
            "depth": 2,
            "exclude_features": ["dev", "test"],
            "group_features": ["group1", "group2"]
        });

        let tool: Result<CargoHackTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed");

        assert!(tool.feature_powerset);
        assert_eq!(tool.depth, Some(2));
        assert_eq!(tool.exclude_features, Some(vec!["dev".to_string(), "test".to_string()]));
        assert_eq!(tool.group_features, Some(vec!["group1".to_string(), "group2".to_string()]));
    }

    #[test]
    fn test_deserialize_minimal_cargo_hack_tool() {
        let input = json!({});

        let tool: Result<CargoHackTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed even with minimal input");

        assert_eq!(tool.command, None);
        assert!(!tool.workspace);
        assert!(!tool.each_feature);
        assert!(!tool.feature_powerset);
    }
}
