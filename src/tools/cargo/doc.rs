use std::process::Command;

use crate::{
    serde_utils::{
        default_true, deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{Annotations, CallToolResult, Role, TextContent, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-doc",
    description = "Build documentation for a Rust package using Cargo. Recommended to use with --no-deps and specific --package for faster builds. Returns path to generated documentation index.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, JsonSchema)]
pub struct CargoDocTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to document. If not specified, documents the current package/workspace.
    /// Recommended to specify specific packages for faster builds.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Document all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Exclude packages from documentation build
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Don't build documentation for dependencies (recommended for faster builds)
    #[serde(default = "default_true")]
    no_deps: bool,

    /// Document private items
    #[serde(default)]
    document_private_items: bool,

    /// Enable docs.rs configuration for additional features (sets RUSTDOCFLAGS="--cfg docsrs")
    #[serde(default)]
    docsrs_config: bool,

    /// Document only this package's library
    #[serde(default)]
    lib: bool,

    /// Document all binaries
    #[serde(default)]
    bins: bool,

    /// Document only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Document all examples
    #[serde(default)]
    examples: bool,

    /// Document only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: bool,

    /// Build artifacts in release mode, with optimizations
    #[serde(default)]
    release: bool,

    /// Build artifacts with the specified profile
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,

    /// Do not abort the build as soon as there is an error
    #[serde(default)]
    keep_going: bool,

    /// Build for the target triple
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
    ignore_rust_version: bool,

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

    /// Error format
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,
}

impl CargoDocTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("doc");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        // Documentation options
        if self.no_deps {
            cmd.arg("--no-deps");
        }

        if self.document_private_items {
            cmd.arg("--document-private-items");
        }

        // Set RUSTDOCFLAGS for docs.rs configuration if enabled
        if self.docsrs_config {
            cmd.env("RUSTDOCFLAGS", "--cfg docsrs");
        }

        // Target selection
        if self.lib {
            cmd.arg("--lib");
        }

        if self.bins {
            cmd.arg("--bins");
        }

        if let Some(bin) = &self.bin {
            cmd.arg("--bin").arg(bin);
        }

        if self.examples {
            cmd.arg("--examples");
        }

        if let Some(example) = &self.example {
            cmd.arg("--example").arg(example);
        }

        // Feature selection
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        // Compilation options
        if self.release {
            cmd.arg("--release");
        }

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going {
            cmd.arg("--keep-going");
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

        if self.ignore_rust_version {
            cmd.arg("--ignore-rust-version");
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref())?;
        for flag in locking_flags {
            cmd.arg(flag);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        // Execute the command and get the result
        let mut result = execute_command(cmd)?;

        // Add documentation path information
        let doc_path = self.get_doc_path();
        let doc_info = format!(
            "\n📚 Documentation generated successfully!\n📁 Documentation index: {doc_path}"
        );

        let annotations = Some(Annotations {
            audience: vec![Role::User, Role::Assistant],
            last_modified: None,
            priority: Some(0.5),
        });

        result
            .content
            .push(TextContent::new(doc_info, annotations, None).into());

        Ok(result)
    }

    fn get_doc_path(&self) -> String {
        let base_dir = self.target_dir.as_deref().unwrap_or("target");
        let profile = if self.release { "release" } else { "debug" };

        // If a specific target is set, include it in the path
        if let Some(target) = &self.target {
            format!("{base_dir}/{target}/doc/index.html")
        } else {
            format!("{base_dir}/{profile}/doc/index.html")
        }
    }
}
