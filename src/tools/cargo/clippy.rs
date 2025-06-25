use std::process::Command;

use crate::{
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-clippy",
    description = "Checks a Rust package to catch common mistakes and improve code quality using Clippy",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
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

    /// Run Clippy only on the given crate, without linting the dependencies
    #[serde(default)]
    no_deps: bool,

    /// Allow dirty working directory (unstaged changes)
    #[serde(default)]
    allow_dirty: bool,

    /// Automatically apply lint suggestions (implies --no-deps and --all-targets)
    #[serde(default)]
    fix: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default)]
    release: bool,

    /// Check for the specified target triple
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default)]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default)]
    lib: bool,

    /// Check all binaries
    #[serde(default)]
    bins: bool,

    /// Check all examples
    #[serde(default)]
    examples: bool,

    /// Check all tests
    #[serde(default)]
    tests: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default)]
    no_default_features: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default)]
    quiet: bool,

    // temporary disabled because AI agents often pass arguments that are not valid
    // /// Additional clippy arguments (e.g., lint warnings/denials)
    // #[serde(default, deserialize_with = "deserialize_string_vec")]
    // clippy_args: Option<Vec<String>>,
    /// Treat warnings as errors
    #[serde(default)]
    warnings_as_errors: bool,
}

impl CargoClippyTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("clippy");
        cmd.arg("--locked");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if self.no_deps {
            cmd.arg("--no-deps");
        }

        if self.fix {
            cmd.arg("--fix");
        }

        if self.allow_dirty {
            cmd.arg("--allow-dirty");
        }

        if self.release {
            cmd.arg("--release");
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if self.all_targets {
            cmd.arg("--all-targets");
        }

        if self.lib {
            cmd.arg("--lib");
        }

        if self.bins {
            cmd.arg("--bins");
        }

        if self.examples {
            cmd.arg("--examples");
        }

        if self.tests {
            cmd.arg("--tests");
        }

        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        // // Add clippy-specific arguments after --
        // if let Some(clippy_args) = &self.clippy_args {
        //     if !clippy_args.is_empty() {
        //         cmd.arg("--");
        //         for arg in clippy_args {
        //             cmd.arg(arg);
        //         }
        //     }
        // }

        if self.warnings_as_errors {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        execute_command(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;use serde_json::json;

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
        let tool =
            tool.expect("Deserialization should succeed even if `package` is missing (it's Option)");            
        
        assert_eq!(tool.package, None);
        assert_eq!(tool.workspace, true);
        assert_eq!(tool.all_features, true);
        assert_eq!(tool.allow_dirty, true);
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
        assert_eq!(tool.workspace, false);
        assert_eq!(tool.all_features, false);
        assert_eq!(tool.allow_dirty, false);
    }
}
