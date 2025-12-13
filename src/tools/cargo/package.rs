use std::process::Command;

use crate::{
    ToolImpl, execute_rmcp_command,
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoPackageRequest {
    /// [Optional] The toolchain to use for packaging, e.g., "stable", "nightly", or "1.70.0".
    /// When specified, cargo will use this specific Rust toolchain version.
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// [Optional] Specific package(s) to assemble. Can specify multiple packages by name.
    /// If not specified, packages the current package or workspace root.
    /// Example: ["my-lib", "my-binary"]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// [Optional] Assemble all packages in the workspace into separate tarballs.
    /// Useful for workspaces with multiple publishable crates.
    #[serde(default)]
    workspace: Option<bool>,

    /// [Optional] Don't assemble specified packages when using --workspace.
    /// Allows selective packaging of workspace members.
    /// Example: ["internal-tools", "test-utils"]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// [Optional] Print files that would be included in the package without creating the tarball.
    /// Useful for reviewing package contents and debugging .gitignore rules.
    #[serde(default)]
    list: bool,

    /// [Optional] Don't verify the package contents by building them.
    /// Skips the compilation step, making packaging faster but less safe.
    /// Use when you're confident the package builds correctly.
    #[serde(default)]
    no_verify: bool,

    /// [Optional] Ignore warnings about missing package metadata (description, license, etc.).
    /// Allows packaging even when human-readable metadata fields are incomplete.
    #[serde(default)]
    no_metadata: bool,

    /// [Optional] Allow packaging even when the working directory has uncommitted changes.
    /// By default, cargo package requires a clean git working directory.
    #[serde(default)]
    allow_dirty: bool,

    /// [Optional] Don't include Cargo.lock in the generated package.
    /// Useful for libraries where you want users to resolve dependencies freshly.
    #[serde(default)]
    exclude_lockfile: bool,

    /// [Optional] Space or comma separated list of features to activate during verification build.
    /// Only affects the build verification step, not the package contents.
    /// Example: ["serde", "async-std"]
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// [Optional] Activate all available features during verification build.
    /// Ensures the package builds correctly with all feature combinations.
    #[serde(default)]
    all_features: Option<bool>,

    /// [Optional] Do not activate the `default` feature during verification build.
    /// Useful for testing minimal builds or when default features are problematic.
    #[serde(default)]
    no_default_features: Option<bool>,

    /// [Optional] Build for the specified target triple during verification.
    /// Useful for cross-compilation testing or platform-specific packages.
    /// Example: "x86_64-unknown-linux-musl"
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// [Optional] Directory for placing generated artifacts and build cache.
    /// Overrides the default target/ directory location.
    #[serde(default, deserialize_with = "deserialize_string")]
    target_dir: Option<String>,

    /// [Optional] Number of parallel jobs for the verification build.
    /// Defaults to the number of CPU cores. Set to 1 for sequential builds.
    #[serde(default)]
    jobs: Option<u32>,

    /// [Optional] Do not abort the verification build as soon as there is an error.
    /// Continues building other targets even if some fail, useful for debugging.
    #[serde(default)]
    keep_going: Option<bool>,

    /// [Optional] Path to the Cargo.toml file to package.
    /// Useful when running from a different directory or with non-standard layouts.
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// [Optional] Path to the Cargo.lock file (unstable feature).
    /// Allows using a different lock file location than the default.
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

    /// [Optional] Registry index URL to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    index: Option<String>,

    /// [Optional] Registry to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    registry: Option<String>,

    /// [Optional] Output representation (unstable) [possible values: human, json]
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// [Optional] Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}
impl CargoPackageRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        if self.workspace.unwrap_or(false) {
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
        if let Some(features) = &self.features
            && !features.is_empty()
        {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
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

        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        for flag in locking_flags {
            cmd.arg(flag);
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

        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        Ok(cmd)
    }
}

pub struct CargoPackageRmcpTool;

impl ToolImpl for CargoPackageRmcpTool {
    const NAME: &'static str = "cargo-package";
    const TITLE: &'static str = "cargo package";
    const DESCRIPTION: &'static str = "Assemble the local package into a distributable tarball for publishing or distribution. <br/>    <br/>    Common use cases:<br/>    - Create a .crate file for publishing to crates.io or a private registry<br/>    - Generate distribution packages for deployment or sharing<br/>    - Validate package contents before publishing (using --list)<br/>    - Test packaging process without verification (using --no-verify)<br/>    - Package workspace members selectively or all at once<br/>    <br/>    The generated tarball contains all files needed to build the package, excluding files listed in .gitignore or .cargo_vcs_info.json. <br/>    By default, the package is also built to verify it can be compiled successfully.<br/>    <br/>    Usually run without any additional arguments for single-package projects.";
    type RequestArgs = CargoPackageRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_rmcp_command(cmd, &Self::NAME)
    }
}
