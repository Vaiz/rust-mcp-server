use std::path::Path;
use std::process::Command;

use rmcp::{
    ErrorData,
    model::RawContent,
};

use crate::{
    Tool, execute_rmcp_command,
    serde_utils::{
        deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
    tools::WORKSPACE_ROOT,
};

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoDocRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package(s) to document. If not specified, documents the current package/workspace.
    /// Recommended to specify specific packages for faster builds.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Document all packages in the workspace
    #[serde(default)]
    workspace: Option<bool>,

    /// Exclude packages from documentation build
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Don't build documentation for dependencies (recommended for faster builds)
    #[serde(default)]
    no_deps: Option<bool>,

    /// Document private items
    #[serde(default)]
    document_private_items: Option<bool>,

    /// Enable docs.rs configuration for additional features (sets RUSTDOCFLAGS="--cfg docsrs")
    #[serde(default)]
    docsrs_config: Option<bool>,

    /// Document only this package's library
    #[serde(default)]
    lib: Option<bool>,

    /// Document all binaries
    #[serde(default)]
    bins: Option<bool>,

    /// Document only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Document all examples
    #[serde(default)]
    examples: Option<bool>,

    /// Document only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

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

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,

    /// Do not abort the build as soon as there is an error
    #[serde(default)]
    keep_going: Option<bool>,

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

    /// Error format
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,
}

impl CargoDocRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exclude in excludes {
                cmd.arg("--exclude").arg(exclude);
            }
        }

        // Documentation options
        if self.no_deps.unwrap_or(true) {
            cmd.arg("--no-deps");
        }

        if self.document_private_items.unwrap_or(false) {
            cmd.arg("--document-private-items");
        }

        // Set RUSTDOCFLAGS for docs.rs configuration if enabled
        if self.docsrs_config.unwrap_or(false) {
            cmd.env("RUSTDOCFLAGS", "--cfg docsrs");
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

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        if self.keep_going.unwrap_or(false) {
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

        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        Ok(cmd)
    }

    fn get_doc_path(&self) -> Option<String> {
        let base_dir = self.target_dir.as_deref().unwrap_or("target");

        // Get the base documentation directory
        let doc_dir = if let Some(target) = &self.target {
            format!("{base_dir}/{target}/doc")
        } else {
            format!("{base_dir}/doc")
        };

        // Get the absolute path using workspace root
        let absolute_doc_dir = if let Some(workspace_root) = WORKSPACE_ROOT.get() {
            Path::new(workspace_root).join(&doc_dir)
        } else {
            Path::new(&doc_dir).to_path_buf()
        };

        // Strategy 1: If package(s) specified, try to find documentation for the first package
        if let Some(packages) = &self.package
            && let Some(first_package) = packages.first()
        {
            // Convert package name to the format used in file paths (hyphens to underscores)
            let package_path_name = first_package.replace('-', "_");
            let package_index = absolute_doc_dir.join(&package_path_name).join("index.html");
            if package_index.exists() {
                return Some(Self::normalize_path(&package_index));
            }
        }

        // Strategy 2: Look for any index.html file under the doc directory
        if absolute_doc_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&absolute_doc_dir) {
                for entry in entries.flatten() {
                    if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                        let index_path = entry.path().join("index.html");
                        if index_path.exists() {
                            return Some(Self::normalize_path(&index_path));
                        }
                    }
                }
            }

            // Check for a top-level index.html
            let top_index = absolute_doc_dir.join("index.html");
            if top_index.exists() {
                return Some(Self::normalize_path(&top_index));
            }
        }

        None
    }

    fn normalize_path(path: &Path) -> String {
        let absolute_path = match path.canonicalize() {
            Ok(canonical) => canonical,
            Err(_) => path.to_path_buf(),
        };

        absolute_path.to_string_lossy().into_owned()
    }
}

pub struct CargoDocRmcpTool;

impl Tool for CargoDocRmcpTool {
    const NAME: &'static str = "cargo-doc";
    const TITLE: &'static str = "Build Rust documentation";
    const DESCRIPTION: &'static str = "Build documentation for a Rust package using Cargo. Recommended to use with no_deps and specific package for faster builds. Returns path to generated documentation index.";
    type RequestArgs = CargoDocRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let cmd = request.build_cmd()?;
        let mut result = execute_rmcp_command(cmd, Self::NAME)?;

        // Add documentation path information only if the command was successful
        if result.is_error != Some(true) {
            let doc_path = request.get_doc_path();
            let doc_info = if let Some(doc_path) = doc_path {
                format!(
                    "\n📚 Documentation generated successfully!\n📄 Documentation file: {doc_path}\n💡 Open this file in your browser to view the docs"
                )
            } else {
                "\n📚 Documentation generated successfully!".to_owned()
            };

            use rmcp::model::{AnnotateAble, Annotations, Role};
            let annotations = Annotations {
                audience: Some(vec![Role::User, Role::Assistant]),
                last_modified: None,
                priority: Some(0.5),
            };

            result
                .content
                .push(RawContent::text(doc_info).annotate(annotations));
        }

        Ok(result)
    }
}
