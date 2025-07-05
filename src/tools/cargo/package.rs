use std::process::Command;

use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{default_true, deserialize_string, deserialize_string_vec};
use crate::tools::execute_command;

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
use crate::serde_utils::Tool;

#[mcp_tool(
    name = "cargo-package",
    description = "Assemble the local package into a distributable tarball for publishing or distribution. 
    
    Common use cases:
    - Create a .crate file for publishing to crates.io or a private registry
    - Generate distribution packages for deployment or sharing
    - Validate package contents before publishing (using --list)
    - Test packaging process without verification (using --no-verify)
    - Package workspace members selectively or all at once
    
    The generated tarball contains all files needed to build the package, excluding files listed in .gitignore or .cargo_vcs_info.json. 
    By default, the package is also built to verify it can be compiled successfully.
    
    Usually run without any additional arguments for single-package projects.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoPackageTool {
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
    workspace: bool,

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
    all_features: bool,

    /// [Optional] Do not activate the `default` feature during verification build.
    /// Useful for testing minimal builds or when default features are problematic.
    #[serde(default)]
    no_default_features: bool,

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
    keep_going: bool,

    /// [Optional] Path to the Cargo.toml file to package.
    /// Useful when running from a different directory or with non-standard layouts.
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// [Optional] Path to the Cargo.lock file (unstable feature).
    /// Allows using a different lock file location than the default.
    #[serde(default, deserialize_with = "deserialize_string")]
    lockfile_path: Option<String>,

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    locked: bool,

    /// [Optional] Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// [Optional] Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,

    /// [Optional] Registry index URL to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    index: Option<String>,

    /// [Optional] Registry to prepare the package for (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    registry: Option<String>,

    /// [Optional] Output representation (unstable) [possible values: human, json]
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// [Optional] Use verbose output
    #[serde(default)]
    verbose: bool,

    /// [Optional] Show only the essential command output. By default is `true`.
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
