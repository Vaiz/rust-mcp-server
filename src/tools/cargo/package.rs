use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{default_true, deserialize_string, deserialize_string_vec};
use crate::tools::execute_command;

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-package",
    description = "Assemble the local package into a distributable tarball. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoPackageTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to assemble
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Assemble all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Don't assemble specified packages
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Print files included in a package without making one
    #[serde(default)]
    list: bool,

    /// Don't verify the contents by building them
    #[serde(default)]
    no_verify: bool,

    /// Ignore warnings about a lack of human-usable metadata
    #[serde(default)]
    no_metadata: bool,

    /// Allow dirty working directories to be packaged
    #[serde(default)]
    allow_dirty: bool,

    /// Don't include the lock file when packaging
    #[serde(default)]
    exclude_lockfile: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: bool,

    /// Build for the target triple
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Directory for all generated artifacts
    #[serde(default, deserialize_with = "deserialize_string")]
    target_dir: Option<String>,

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,

    /// Do not abort the build as soon as there is an error
    #[serde(default)]
    keep_going: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,

    /// Registry index URL to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    index: Option<String>,

    /// Registry to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    registry: Option<String>,

    /// Output representation (unstable) [possible values: human, json]
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoPackageTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");

        // Add toolchain if specified
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }

        cmd.arg("package");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if let Some(exclude) = &self.exclude {
            for excluded in exclude {
                cmd.arg("--exclude").arg(excluded);
            }
        }

        // Operation modes
        if self.list {
            cmd.arg("--list");
        }

        if self.no_verify {
            cmd.arg("--no-verify");
        }

        if self.no_metadata {
            cmd.arg("--no-metadata");
        }

        if self.allow_dirty {
            cmd.arg("--allow-dirty");
        }

        if self.exclude_lockfile {
            cmd.arg("--exclude-lockfile");
        }

        // Feature selection
        if let Some(features) = &self.features {
            if !features.is_empty() {
                cmd.arg("--features").arg(features.join(","));
            }
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        // Compilation options
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if let Some(target_dir) = &self.target_dir {
            cmd.arg("--target-dir").arg(target_dir);
        }

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going {
            cmd.arg("--keep-going");
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

        // Registry options
        if let Some(index) = &self.index {
            cmd.arg("--index").arg(index);
        }

        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        // Output options
        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.quiet && !self.verbose {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}
