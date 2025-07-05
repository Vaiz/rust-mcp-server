use std::process::Command;

use crate::{
    serde_utils::{default_true, deserialize_string, deserialize_string_vec},
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
use crate::serde_utils::Tool;

#[mcp_tool(
    name = "cargo-clippy",
    description = "Checks a Rust package to catch common mistakes and improve code quality using Clippy",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoClippyTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Exclude packages from the check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Run Clippy only on the given crate, without linting the dependencies
    #[serde(default)]
    no_deps: bool,

    /// Allow dirty working directory (unstaged changes). Works only with `fix`.
    #[serde(default)]
    allow_dirty: bool,

    /// Automatically apply lint suggestions (implies --no-deps and --all-targets)
    #[serde(default)]
    fix: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default)]
    release: bool,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default)]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default)]
    lib: bool,

    /// Check all binaries
    #[serde(default)]
    bins: bool,

    /// Check only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Check all examples
    #[serde(default)]
    examples: bool,

    /// Check only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Check all tests
    #[serde(default)]
    tests: bool,

    /// Check only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Check all targets that have `bench = true` set
    #[serde(default)]
    benches: bool,

    /// Check only the specified bench target
    #[serde(default, deserialize_with = "deserialize_string")]
    bench: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default)]
    no_default_features: bool,

    /// Check artifacts with the specified profile
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Check for the specified target triple
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Directory for all generated artifacts
    #[serde(default, deserialize_with = "deserialize_string")]
    target_dir: Option<String>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[serde(default)]
    ignore_rust_version: bool,

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,

    /// Treat warnings as errors
    #[serde(default)]
    warnings_as_errors: bool,
}

impl CargoClippyTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("clippy");

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

        // Clippy-specific options
        if self.no_deps {
            cmd.arg("--no-deps");
        }

        if self.fix {
            cmd.arg("--fix");
        }

        if self.allow_dirty && self.fix {
            cmd.arg("--allow-dirty");
        }

        // Compilation options
        if self.release {
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

        // Target selection
        if self.all_targets {
            cmd.arg("--all-targets");
        }

        if self.lib {
            cmd.arg("--lib");
        }

        if self.bins {
            cmd.arg("--bins");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if self.examples {
            cmd.arg("--examples");
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        if self.tests {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.benches {
            cmd.arg("--benches");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if self.ignore_rust_version {
            cmd.arg("--ignore-rust-version");
        }

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

        if self.warnings_as_errors {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        execute_command(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_with_missing_package_field() {
        // Simulate a JSON input missing the `package` field (should be Option)
        let input = json!({
            "toolchain": null,
            "workspace": true,
            "no_deps": false,
            "allow_dirty": true,
            "fix": true,
            "release": false,
            "target": null,
            "all_targets": true,
            "lib": true,
            "bins": true,
            "examples": true,
            "tests": true,
            "features": null,
            "all_features": true,
            "no_default_features": false,
            "verbose": true,
            "quiet": false,
            "warnings_as_errors": false
        });

        let tool: Result<CargoClippyTool, _> = serde_json::from_value(input);
        let tool = tool
            .expect("Deserialization should succeed even if `package` is missing (it's Option)");

        assert_eq!(tool.package, None);
        assert!(tool.workspace);
        assert!(tool.all_features);
        assert!(tool.allow_dirty);
    }

    #[test]
    fn test_deserialize_with_package_field() {
        // Simulate a JSON input missing the `package` field (should be Option)
        let input = json!({
            "package": ["my_package"],
        });

        let tool: Result<CargoClippyTool, _> = serde_json::from_value(input);
        let tool = tool.expect("Deserialization should succeed");

        assert_eq!(tool.package.unwrap(), ["my_package".to_owned()]);
        assert!(!tool.workspace);
        assert!(!tool.all_features);
        assert!(!tool.allow_dirty);
    }
}
