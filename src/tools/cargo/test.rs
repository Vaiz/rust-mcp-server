use std::process::Command;

use crate::{
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-test",
    description = "Run `cargo test` to execute Rust tests in the current project.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoTestTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// If specified, only run tests containing this string in their names
    #[serde(default, deserialize_with = "deserialize_string")]
    testname: Option<String>,

    /// Arguments for the test binary (after --)
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    test_args: Option<Vec<String>>,

    /// Compile, but don't run tests
    #[serde(default)]
    no_run: Option<bool>,

    /// Run all tests regardless of failure
    #[serde(default)]
    no_fail_fast: Option<bool>,

    /// Package(s) to run tests for
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Test all packages in the workspace
    #[serde(default)]
    workspace: Option<bool>,

    /// Exclude packages from the test
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Test only this package's library
    #[serde(default)]
    lib: Option<bool>,

    /// Test all binaries
    #[serde(default)]
    bins: Option<bool>,

    /// Test only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Test all examples
    #[serde(default)]
    examples: Option<bool>,

    /// Test only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Test all targets that have `test = true` set
    #[serde(default)]
    tests: Option<bool>,

    /// Test only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Test all targets that have `bench = true` set
    #[serde(default)]
    benches: Option<bool>,

    /// Test only the specified bench target
    #[serde(default, deserialize_with = "deserialize_string")]
    bench: Option<String>,

    /// Test all targets (does not include doctests)
    #[serde(default)]
    all_targets: Option<bool>,

    /// Test only this library's documentation
    #[serde(default)]
    doc: Option<bool>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,

    /// Build artifacts in release mode, with optimizations
    #[serde(default)]
    release: Option<bool>,

    /// Build artifacts with the specified profile
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Build for the target triple
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Directory for all generated artifacts
    #[serde(default, deserialize_with = "deserialize_string")]
    target_dir: Option<String>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[serde(default)]
    ignore_rust_version: Option<bool>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    locking_mode: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}

impl CargoTestTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("test");

        // Add testname argument if provided
        if let Some(testname) = &self.testname {
            cmd.arg(testname);
        }

        // Test compilation options
        if self.no_run.unwrap_or(false) {
            cmd.arg("--no-run");
        }

        if self.no_fail_fast.unwrap_or(false) {
            cmd.arg("--no-fail-fast");
        }

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        // Target selection
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        if self.bins.unwrap_or(false) {
            cmd.arg("--bins");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if self.examples.unwrap_or(false) {
            cmd.arg("--examples");
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        if self.tests.unwrap_or(false) {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.benches.unwrap_or(false) {
            cmd.arg("--benches");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
        }

        if self.all_targets.unwrap_or(false) {
            cmd.arg("--all-targets");
        }

        if self.doc.unwrap_or(false) {
            cmd.arg("--doc");
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

        // Compilation options
        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if let Some(target_dir) = &self.target_dir {
            cmd.arg("--target-dir").arg(target_dir);
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }

        if self.ignore_rust_version.unwrap_or(false) {
            cmd.arg("--ignore-rust-version");
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        for flag in locking_flags {
            cmd.arg(flag);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        // Pass test binary args after --
        if let Some(test_args) = &self.test_args {
            cmd.arg("--");
            for arg in test_args {
                cmd.arg(arg);
            }
        }

        execute_command(cmd, &Self::tool_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_missing_package_field() {
        let input = json!({
            "toolchain": null,
            "workspace": true,
            "all_features": true,
            "no_default_features": false,
            "release": false,
            "all_targets": true
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed even if `package` is missing (it's Option)");

        assert_eq!(tool.package, None);
        assert_eq!(tool.workspace, Some(true));
        assert_eq!(tool.all_features, Some(true));
        assert_eq!(tool.all_targets, Some(true));
    }

    #[test]
    fn test_deserialize_with_package_field_array() {
        let input = json!({
            "package": ["my_package", "another_package"],
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with package array");

        assert_eq!(
            tool.package.unwrap(),
            ["my_package".to_owned(), "another_package".to_owned()]
        );
        assert_eq!(tool.workspace, None);
        assert_eq!(tool.all_features, None);
    }

    #[test]
    fn test_deserialize_with_single_package_array() {
        let input = json!({
            "package": ["single_package"],
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with single-item package array");

        assert_eq!(tool.package.unwrap(), ["single_package".to_owned()]);
    }

    #[test]
    fn test_deserialize_with_single_package() {
        let input = json!({
            "package": "single_package",
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with single-item package array");

        assert_eq!(tool.package.unwrap(), ["single_package".to_owned()]);
    }

    #[test]
    fn test_deserialize_with_features_array() {
        let input = json!({
            "features": ["serde", "tokio"],
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
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

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed with single feature string");

        assert_eq!(tool.features.unwrap(), ["serde".to_owned()]);
    }

    #[test]
    fn test_deserialize_with_features_string_array() {
        let input = json!({
            "features": "[\"serde\",\"tokio\"]",
        });

        let tool: Result<CargoTestTool, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed with features string that looks like array");

        assert_eq!(tool.features.unwrap(), ["[\"serde\",\"tokio\"]".to_owned()]);
    }
}
