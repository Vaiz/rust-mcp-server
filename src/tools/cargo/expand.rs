use std::process::Command;

use rmcp::ErrorData;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags},
};

/// Request parameters for cargo-expand command.
/// Shows the result of macro expansion for Rust code.
#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoExpandRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Local path to module or other named item to expand, e.g. os::unix::ffi
    #[serde(default, deserialize_with = "deserialize_string")]
    item: Option<String>,

    /// Package to expand
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

    /// Expand only this package's library
    #[serde(default)]
    lib: Option<bool>,

    /// Expand only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Expand only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Expand only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Include tests when expanding the lib or bin
    #[serde(default)]
    tests: Option<bool>,

    /// Expand only the specified bench target
    #[serde(default, deserialize_with = "deserialize_string")]
    bench: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Build artifacts in release mode, with optimizations
    #[serde(default)]
    release: Option<bool>,

    /// Build artifacts with the specified profile
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Target triple which compiles will be for
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Directory for all generated artifacts
    #[serde(default, deserialize_with = "deserialize_string")]
    target_dir: Option<String>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Do not attempt to run rustfmt on expanded code
    #[serde(default)]
    ugly: Option<bool>,

    /// Print command lines as they are executed
    #[serde(default)]
    verbose: Option<bool>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    locking_mode: Option<String>,
}

impl CargoExpandRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");

        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }

        cmd.arg("expand");

        // Package selection
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        // Target selection
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.tests.unwrap_or(false) {
            cmd.arg("--tests");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
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

        // Locking mode
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        cmd.args(locking_flags);

        // Output options
        if self.ugly.unwrap_or(false) {
            cmd.arg("--ugly");
        }

        if self.verbose.unwrap_or(false) {
            cmd.arg("--verbose");
        }

        // Item to expand (positional argument, must be last)
        if let Some(item) = &self.item {
            cmd.arg(item);
        }

        Ok(cmd)
    }
}

pub struct CargoExpandRmcpTool;

impl Tool for CargoExpandRmcpTool {
    const NAME: &'static str = "cargo-expand";
    const TITLE: &'static str = "Cargo Expand";
    const DESCRIPTION: &'static str = "Show the result of macro expansion for Rust code. Requires cargo-expand to be installed (cargo install cargo-expand). Useful for debugging procedural macros, derive macros, and understanding what code macros generate.";
    type RequestArgs = CargoExpandRequest;

    fn call_rmcp_tool(&self, req: Self::RequestArgs) -> Result<crate::Response, rmcp::ErrorData> {
        execute_command(req.build_cmd()?, Self::NAME).map(Into::into)
    }
}
