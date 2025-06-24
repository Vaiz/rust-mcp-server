use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-generate_lockfile",
    description = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoGenerateLockfileTool {
    /// The name of the package to generate lockfile for. If not specified, generates for the current package/workspace.
    package: Option<String>,
}

impl CargoGenerateLockfileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-build",
    description = "Builds a Rust project using Cargo. Usually, run without any additional arguments.",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoBuildTool {
    /// The name of the package to build. If not specified, the current package/workspace is built.
    package: Option<String>,

    /// The profile to use for the build. Defaults to "dev".
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    profile: Option<String>,
}

impl CargoBuildTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        cmd.arg("--locked");

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCleanTool {
    /// The name of the package to clean. If not specified, cleans the entire workspace.
    package: Option<String>,

    /// The profile to use for the build. Defaults to "dev".
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    profile: Option<String>,
}

impl CargoCleanTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clean");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(name = "cargo-fmt", description = "Formats Rust code using rustfmt. Usually, run without any additional arguments.", openWorldHint = false)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoFmtTool {
    /// The name of the package(s) to format. If not specified, formats the current package.
    package: Option<Vec<String>>,

    /// Format all packages in the workspace and their dependencies
    #[serde(default = "default_false")]
    all: bool,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default = "default_false")]
    check: bool,

    /// No output printed to stdout
    #[serde(default = "default_false")]
    quiet: bool,

    /// Use verbose output
    #[serde(default = "default_false")]
    verbose: bool,
}

impl CargoFmtTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("fmt");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.all {
            cmd.arg("--all");
        }

        if self.check {
            cmd.arg("--check");
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-check",
    description = "Checks a Rust package and all of its dependencies for errors. Usually, run without any additional arguments.",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCheckTool {
    /// Package(s) to check
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default = "default_false")]
    workspace: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default = "default_false")]
    release: bool,

    /// Check for the specified target triple
    target: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default = "default_false")]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default = "default_false")]
    lib: bool,

    /// Check all binaries
    #[serde(default = "default_false")]
    bins: bool,

    /// Check all examples
    #[serde(default = "default_false")]
    examples: bool,

    /// Check all tests
    #[serde(default = "default_false")]
    tests: bool,

    /// Space or comma separated list of features to activate
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default = "default_false")]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default = "default_false")]
    no_default_features: bool,

    /// Use verbose output
    #[serde(default = "default_false")]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default = "default_false")]
    quiet: bool,
}

impl CargoCheckTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("check");
        cmd.arg("--locked");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
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

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-clippy",
    description = "Checks a Rust package to catch common mistakes and improve code quality using Clippy",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoClippyTool {
    /// Package(s) to check
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default = "default_false")]
    workspace: bool,

    /// Run Clippy only on the given crate, without linting the dependencies
    #[serde(default = "default_false")]
    no_deps: bool,

    /// Automatically apply lint suggestions (implies --no-deps and --all-targets)
    #[serde(default = "default_false")]
    fix: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default = "default_false")]
    release: bool,

    /// Check for the specified target triple
    target: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default = "default_false")]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default = "default_false")]
    lib: bool,

    /// Check all binaries
    #[serde(default = "default_false")]
    bins: bool,

    /// Check all examples
    #[serde(default = "default_false")]
    examples: bool,

    /// Check all tests
    #[serde(default = "default_false")]
    tests: bool,

    /// Space or comma separated list of features to activate
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default = "default_false")]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default = "default_false")]
    no_default_features: bool,

    /// Use verbose output
    #[serde(default = "default_false")]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default = "default_false")]
    quiet: bool,

    /// Additional clippy arguments (e.g., lint warnings/denials)
    clippy_args: Option<Vec<String>>,
}

impl CargoClippyTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
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

        // Add clippy-specific arguments after --
        if let Some(clippy_args) = &self.clippy_args {
            if !clippy_args.is_empty() {
                cmd.arg("--");
                for arg in clippy_args {
                    cmd.arg(arg);
                }
            }
        }

        execute_command(cmd)
    }
}

const fn default_false() -> bool {
    false
}
