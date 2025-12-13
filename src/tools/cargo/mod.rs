mod add_remove;
mod build;
mod check;
mod clippy;
mod doc;
mod info;
mod metadata;
mod package;
mod search;
mod test;
mod update;

pub use add_remove::{CargoAddTool, CargoRemoveTool};
pub use build::CargoBuildTool;
pub use check::{CargoCheckRmcpTool, CargoCheckTool};
pub use clippy::CargoClippyTool;
pub use doc::CargoDocTool;
pub use info::CargoInfoTool;
pub use metadata::CargoMetadataTool;
pub use package::CargoPackageTool;
pub use search::CargoSearchTool;
pub use test::CargoTestTool;
pub use update::CargoUpdateTool;

use std::process::Command;

use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{
    deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
    output_verbosity_to_cli_flags,
};
use crate::tools::execute_command;

use crate::serde_utils::Tool;

#[mcp_tool(
    name = "cargo-generate_lockfile",
    description = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoGenerateLockfileTool {
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

impl CargoGenerateLockfileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

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

        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        cmd.args(locking_flags);

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        execute_command(cmd, &Self::tool_name())
    }
}

#[mcp_tool(
    name = "cargo-clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoCleanTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to clean artifacts for. If not specified, cleans the entire workspace.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Clean artifacts of the specified profile. If not specified, cleans everything.
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Whether or not to clean just the documentation directory
    #[serde(default)]
    doc: Option<bool>,

    /// Display what would be deleted without deleting anything
    #[serde(default)]
    dry_run: Option<bool>,

    /// Whether or not to clean release artifacts
    #[serde(default)]
    release: Option<bool>,

    /// Target triple to clean output for
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

impl CargoCleanTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("clean");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        // Compilation options
        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if self.doc.unwrap_or(false) {
            cmd.arg("--doc");
        }

        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        if self.release.unwrap_or(false) {
            cmd.arg("--release");
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

        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        cmd.args(locking_flags);

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        execute_command(cmd, &Self::tool_name())
    }
}

#[mcp_tool(
    name = "cargo-fmt",
    description = "Formats Rust code using rustfmt. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoFmtTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package(s) to format. If not specified, formats the current package.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Format all packages, and also their local path-based dependencies
    #[serde(default)]
    all: bool,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default)]
    check: bool,

    /// Specify path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Specify message-format: short|json|human
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}

impl CargoFmtTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("fmt");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.all {
            cmd.arg("--all");
        }

        // Formatting options
        if self.check {
            cmd.arg("--check");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        execute_command(cmd, &Self::tool_name())
    }
}

#[mcp_tool(
    name = "cargo-new",
    description = "Create a new cargo package at <path>. Creates a new Rust project with the specified name and template.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoNewTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Path where the new cargo package will be created.
    /// Examples: "my-project", "path/to/my-lib", "../new-crate"
    pub path: String,

    /// Set the resulting package name, defaults to the directory name
    #[serde(default, deserialize_with = "deserialize_string")]
    pub name: Option<String>,

    /// Use a binary (application) template [default]
    #[serde(default)]
    pub bin: bool,

    /// Use a library template
    #[serde(default)]
    pub lib: Option<bool>,

    /// Edition to set for the crate generated. Possible values: 2015, 2018, 2021, 2024
    #[serde(default, deserialize_with = "deserialize_string")]
    pub edition: Option<String>,

    /// Initialize a new repository for the given version control system, overriding a global configuration.
    /// Possible values: git, hg, pijul, fossil, none
    #[serde(default, deserialize_with = "deserialize_string")]
    pub vcs: Option<String>,

    /// Registry to use
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    pub locking_mode: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    pub output_verbosity: Option<String>,
}

impl CargoNewTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("new");

        // Add the path argument (required)
        cmd.arg(&self.path);

        // Template options
        if self.bin {
            cmd.arg("--bin");
        }
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        // Package options
        if let Some(name) = &self.name {
            cmd.arg("--name").arg(name);
        }
        if let Some(edition) = &self.edition {
            cmd.arg("--edition").arg(edition);
        }
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        // VCS options
        if let Some(vcs) = &self.vcs {
            cmd.arg("--vcs").arg(vcs);
        }

        // Manifest options
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "unlocked")?;
        cmd.args(locking_flags);

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        execute_command(cmd, &Self::tool_name())
    }
}

#[mcp_tool(
    name = "cargo-list",
    description = "Lists installed cargo commands using 'cargo --list'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoListTool {}

impl CargoListTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("--list");
        execute_command(cmd, &Self::tool_name())
    }
}
