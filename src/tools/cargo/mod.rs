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
mod workspace_info;

pub use add_remove::{CargoAddRmcpTool, CargoRemoveRmcpTool};
pub use build::CargoBuildRmcpTool;
pub use check::CargoCheckRmcpTool;
pub use clippy::CargoClippyRmcpTool;
pub use doc::CargoDocRmcpTool;
pub use info::CargoInfoRmcpTool;
pub use metadata::CargoMetadataRmcpTool;
pub use package::CargoPackageRmcpTool;
pub use search::CargoSearchRmcpTool;
pub use test::CargoTestRmcpTool;
pub use update::CargoUpdateRmcpTool;
pub use workspace_info::CargoWorkspaceInfoRmcpTool;

use std::process::Command;

use crate::{
    ResultExt, Tool, execute_command,
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
};
use rmcp::{ErrorData, model::CallToolResult};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoGenerateLockfileRequest {
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

impl CargoGenerateLockfileRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        Ok(cmd)
    }
}

pub struct CargoGenerateLockfileRmcpTool;

impl Tool for CargoGenerateLockfileRmcpTool {
    const NAME: &'static str = "cargo-generate_lockfile";
    const TITLE: &'static str = "Generate Cargo.lock";
    const DESCRIPTION: &'static str = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.";
    type RequestArgs = CargoGenerateLockfileRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoCleanRequest {
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

impl CargoCleanRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        Ok(cmd)
    }
}

pub struct CargoCleanRmcpTool;

impl Tool for CargoCleanRmcpTool {
    const NAME: &'static str = "cargo-clean";
    const TITLE: &'static str = "Clean Cargo artifacts";
    const DESCRIPTION: &'static str = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.";
    type RequestArgs = CargoCleanRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoFmtRequest {
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

impl CargoFmtRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        Ok(cmd)
    }
}

pub struct CargoFmtRmcpTool;

impl Tool for CargoFmtRmcpTool {
    const NAME: &'static str = "cargo-fmt";
    const TITLE: &'static str = "Format Rust code";
    const DESCRIPTION: &'static str =
        "Formats Rust code using rustfmt. Usually, run without any additional arguments.";
    type RequestArgs = CargoFmtRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        let output = execute_command(request.build_cmd()?, Self::NAME)?;
        let failed = !output.success();
        let mut call_tool_result: CallToolResult = output.into();

        if failed && request.check {
            call_tool_result.add_recommendation(format!(
                "Run #{} without `check: true` to automatically format the code",
                Self::NAME
            ));
        }

        Ok(call_tool_result)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoNewRequest {
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

impl CargoNewRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        Ok(cmd)
    }
}

pub struct CargoNewRmcpTool;

impl Tool for CargoNewRmcpTool {
    const NAME: &'static str = "cargo-new";
    const TITLE: &'static str = "Create new Rust project";
    const DESCRIPTION: &'static str = "Create a new cargo package at <path>. Creates a new Rust project with the specified name and template.";
    type RequestArgs = CargoNewRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoListRequest {}

impl CargoListRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("--list");
        Ok(cmd)
    }
}

pub struct CargoListRmcpTool;

impl Tool for CargoListRmcpTool {
    const NAME: &'static str = "cargo-list";
    const TITLE: &'static str = "List cargo commands";
    const DESCRIPTION: &'static str = "Lists installed cargo commands using 'cargo --list'.";
    type RequestArgs = CargoListRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
