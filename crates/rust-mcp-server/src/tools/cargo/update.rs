use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoUpdateRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to update. If not specified, updates all dependencies.
    /// Examples: ["serde"], ["tokio", "clap"], [] (update all)
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    spec: Option<Vec<String>>,

    /// Don't actually write the lockfile
    #[serde(default)]
    dry_run: Option<bool>,

    /// Force updating all dependencies of [SPEC]... as well
    #[serde(default)]
    recursive: Option<bool>,

    /// Update [SPEC] to exactly PRECISE
    #[serde(default, deserialize_with = "deserialize_string")]
    precise: Option<String>,

    /// Update [SPEC] to latest SemVer-breaking version (unstable)
    #[serde(default)]
    breaking: Option<bool>,

    /// Only update the workspace packages
    #[serde(default)]
    workspace: Option<bool>,

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

    /// Override a configuration value
    #[serde(default, deserialize_with = "deserialize_string")]
    config: Option<String>,

    /// Unstable (nightly-only) flags to Cargo
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    unstable_flags: Option<Vec<String>>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}
impl CargoUpdateRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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
        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        if self.recursive.unwrap_or(false) {
            cmd.arg("--recursive");
        }

        if let Some(precise) = &self.precise {
            cmd.arg("--precise").arg(precise);
        }

        if self.breaking.unwrap_or(false) {
            cmd.arg("--breaking");
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
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

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(unstable_flags) = &self.unstable_flags {
            for flag in unstable_flags {
                cmd.arg("-Z").arg(flag);
            }
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        Ok(cmd)
    }
}

pub struct CargoUpdateRmcpTool;

impl Tool for CargoUpdateRmcpTool {
    const NAME: &'static str = "cargo-update";
    const TITLE: &'static str = "cargo update";
    const DESCRIPTION: &'static str = "Update dependencies as recorded in the local lock file. Updates the dependencies in Cargo.lock to their latest compatible versions.";
    type RequestArgs = CargoUpdateRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
