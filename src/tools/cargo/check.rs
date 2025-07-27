use std::process::Command;

use crate::{
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-check",
    description = "Checks a Rust package and all of its dependencies for errors. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, JsonSchema)]
pub struct CargoCheckTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default)]
    workspace: Option<bool>,

    /// Exclude packages from the check
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Check only this package's library
    #[serde(default)]
    lib: Option<bool>,

    /// Check all binaries
    #[serde(default)]
    bins: Option<bool>,

    /// Check only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Check all examples
    #[serde(default)]
    examples: Option<bool>,

    /// Check only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Check all tests
    #[serde(default)]
    tests: Option<bool>,

    /// Check only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Check all targets that have `bench = true` set
    #[serde(default)]
    benches: Option<bool>,

    /// Check only the specified bench target
    #[serde(default, deserialize_with = "deserialize_string")]
    bench: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default)]
    all_targets: Option<bool>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the default feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,

    /// Do not abort the build as soon as there is an error
    #[serde(default)]
    keep_going: Option<bool>,

    /// Check artifacts in release mode, with optimizations
    #[serde(default)]
    release: Option<bool>,

    /// Check artifacts with the specified profile
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Check for the specified target triple
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

    /// Treat warnings as errors
    #[serde(default)]
    warnings_as_errors: Option<bool>,
}

impl CargoCheckTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("check");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        // Target selection
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        if self.bins.unwrap_or(false) {
            cmd.arg("--bins");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if self.examples.unwrap_or(false) {
            cmd.arg("--examples");
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        if self.tests.unwrap_or(false) {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.benches.unwrap_or(false) {
            cmd.arg("--benches");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
        }

        if self.all_targets.unwrap_or(false) {
            cmd.arg("--all-targets");
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        // Compilation options
        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going.unwrap_or(false) {
            cmd.arg("--keep-going");
        }

        if self.release.unwrap_or(false) {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
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

        if self.ignore_rust_version.unwrap_or(false) {
            cmd.arg("--ignore-rust-version");
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        for flag in locking_flags {
            cmd.arg(flag);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        if self.warnings_as_errors.unwrap_or(false) {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        execute_command(cmd)
    }
}
