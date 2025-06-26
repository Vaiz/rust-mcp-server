use std::process::Command;

use crate::{
    serde_utils::{default_true, deserialize_string, deserialize_string_vec},
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
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

    /// Disable the default features
    #[serde(default)]
    pub no_default_features: bool,

    /// Re-enable the default features
    #[serde(default)]
    pub default_features: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub features: Option<Vec<String>>,

    /// Rename the dependency
    #[serde(default, deserialize_with = "deserialize_string")]
    pub rename: Option<String>,

    /// Package to modify
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target_package: Option<String>,

    /// Filesystem path to local crate to add
    #[serde(default, deserialize_with = "deserialize_string")]
    pub path: Option<String>,

    /// Git repository location
    #[serde(default, deserialize_with = "deserialize_string")]
    pub git: Option<String>,

    /// Git branch to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub branch: Option<String>,

    /// Git tag to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub tag: Option<String>,

    /// Git reference to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub rev: Option<String>,

    /// Package registry for this dependency
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,

    /// Add as dependency to the given target platform
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target: Option<String>,

    /// Don't actually write the manifest
    #[serde(default)]
    pub dry_run: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[serde(default)]
    pub ignore_rust_version: bool,

    /// Assert that `Cargo.lock` will remain unchanged.
    #[serde(default)]
    pub locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    pub frozen: bool,

    /// Use verbose output
    #[serde(default)]
    pub verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl CargoAddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("add");

        if let Some(version) = &self.version {
            cmd.arg(format!("{}@{version}", self.package));
        } else {
            cmd.arg(&self.package);
        }

        // Dependency type
        if self.dev {
            cmd.arg("--dev");
        }
        if self.build {
            cmd.arg("--build");
        }
        if self.optional {
            cmd.arg("--optional");
        }

        // Feature selection
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }
        if self.default_features {
            cmd.arg("--default-features");
        }
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        // Package selection
        if let Some(target_package) = &self.target_package {
            cmd.arg("--package").arg(target_package);
        }

        // Source options
        if let Some(path) = &self.path {
            cmd.arg("--path").arg(path);
        }
        if let Some(git) = &self.git {
            cmd.arg("--git").arg(git);
        }
        if let Some(branch) = &self.branch {
            cmd.arg("--branch").arg(branch);
        }
        if let Some(tag) = &self.tag {
            cmd.arg("--tag").arg(tag);
        }
        if let Some(rev) = &self.rev {
            cmd.arg("--rev").arg(rev);
        }

        // Registry options
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        // Target platform
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Naming options
        if let Some(rename) = &self.rename {
            cmd.arg("--rename").arg(rename);
        }

        // Other options
        if self.dry_run {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
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

        execute_command(cmd)
    }
}

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-remove",
    description = "Remove dependencies from a Cargo.toml manifest file.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoRemoveTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Dependencies to be removed.
    /// Examples:
    /// - Single dependency: ["regex"]
    /// - Multiple dependencies: ["tokio", "clap", "serde"]
    /// - Can be simple crate names as they appear in Cargo.toml
    pub dep_id: Vec<String>,

    /// Remove from dev-dependencies
    #[serde(default)]
    pub dev: bool,

    /// Remove from build-dependencies
    #[serde(default)]
    pub build: bool,

    /// Remove from target-dependencies
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target: Option<String>,

    /// Package to remove from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub package: Option<String>,

    /// Don't actually write the manifest
    #[serde(default)]
    pub dry_run: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Assert that `Cargo.lock` will remain unchanged.
    #[serde(default)]
    pub locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    pub frozen: bool,

    /// Use verbose output
    #[serde(default)]
    pub verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl CargoRemoveTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("remove");

        // Add dependency names
        for dep in &self.dep_id {
            cmd.arg(dep);
        }

        // Section options
        if self.dev {
            cmd.arg("--dev");
        }
        if self.build {
            cmd.arg("--build");
        }
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Package selection
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        // Other options
        if self.dry_run {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
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

        execute_command(cmd)
    }
}
