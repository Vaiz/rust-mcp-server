mod check;
mod clippy;
mod info;
mod metadata;
mod search;
mod test;

pub use check::CargoCheckTool;
pub use clippy::CargoClippyTool;
pub use info::CargoInfoTool;
pub use metadata::CargoMetadataTool;
pub use search::CargoSearchTool;
pub use test::CargoTestTool;

use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{default_true, deserialize_string, deserialize_string_vec};
use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-generate_lockfile",
    description = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoGenerateLockfileTool {
    /// The name of the package to generate lockfile for. If not specified, generates for the current package/workspace.
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoGenerateLockfileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-build",
    description = "Builds a Rust project using Cargo. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoBuildTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package to build. If not specified, the current package/workspace is built.
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

    /// The profile to use for the build. Defaults to "dev".
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Treat warnings as errors
    #[serde(default)]
    warnings_as_errors: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoBuildTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("build");
        cmd.arg("--locked");

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        if self.warnings_as_errors {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCleanTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package to clean. If not specified, cleans the entire workspace.
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

    /// lean artifacts of the specified profile. If not specified, cleans everything.
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoCleanTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("clean");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-fmt",
    description = "Formats Rust code using rustfmt. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoFmtTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package(s) to format. If not specified, formats the current package.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Format all packages in the workspace and their dependencies
    #[serde(default)]
    all: bool,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default)]
    check: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,
}

impl CargoFmtTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
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
    name = "cargo-add",
    description = "Adds a dependency to a Rust project using cargo add.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoAddTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,
    /// The name of the dependency to add.
    pub package: String,
    /// Optional version requirement.
    #[serde(default, deserialize_with = "deserialize_string")]
    pub version: Option<String>,
    /// Add as a dev-dependency
    #[serde(default)]
    pub dev: bool,
    /// Add as a build-dependency
    #[serde(default)]
    pub build: bool,
    /// Add as an optional dependency
    #[serde(default)]
    pub optional: bool,
}

impl CargoAddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("add");
        if let Some(version) = &self.version {
            cmd.arg(format!("{}@{version}", self.package));
        } else {
            cmd.arg(&self.package);
        }
        if self.dev {
            cmd.arg("--dev");
        }
        if self.build {
            cmd.arg("--build");
        }
        if self.optional {
            cmd.arg("--optional");
        }
        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-list",
    description = "Lists installed cargo commands using 'cargo --list'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoListTool {}

impl CargoListTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("--list");
        execute_command(cmd)
    }
}
