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
    name = "cargo-update",
    description = "Update dependencies as recorded in the local lock file. Updates the dependencies in Cargo.lock to their latest compatible versions.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoUpdateTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to update. If not specified, updates all dependencies.
    /// Examples: ["serde"], ["tokio", "clap"], [] (update all)
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    spec: Option<Vec<String>>,

    /// Don't actually write the lockfile
    #[serde(default)]
    dry_run: bool,

    /// Force updating all dependencies of [SPEC]... as well
    #[serde(default)]
    recursive: bool,

    /// Update [SPEC] to exactly PRECISE
    #[serde(default, deserialize_with = "deserialize_string")]
    precise: Option<String>,

    /// Update [SPEC] to latest SemVer-breaking version (unstable)
    #[serde(default)]
    breaking: bool,

    /// Only update the workspace packages
    #[serde(default)]
    workspace: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

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

    /// Coloring [possible values: auto, always, never]
    #[serde(default, deserialize_with = "deserialize_string")]
    color: Option<String>,

    /// Override a configuration value
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,

    /// Unstable (nightly-only) flags to Cargo
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    unstable_flags: Option<Vec<String>>,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoUpdateTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("update");

        // Package selection
        if let Some(specs) = &self.spec {
            for spec in specs {
                cmd.arg(spec);
            }
        }

        // Update options
        if self.dry_run {
            cmd.arg("--dry-run");
        }

        if self.recursive {
            cmd.arg("--recursive");
        }

        if let Some(precise) = &self.precise {
            cmd.arg("--precise").arg(precise);
        }

        if self.breaking {
            cmd.arg("--breaking");
        }

        if self.workspace {
            cmd.arg("--workspace");
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

        // Configuration options
        if let Some(color) = &self.color {
            cmd.arg("--color").arg(color);
        }

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(unstable_flags) = &self.unstable_flags {
            for flag in unstable_flags {
                cmd.arg("-Z").arg(flag);
            }
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
