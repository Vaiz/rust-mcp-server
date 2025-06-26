mod check;
mod clippy;
mod info;
mod metadata;
mod search;
mod test;

pub use check::CargoCheckTool;
pub use clippy::CargoClippyTool;
pub use info::CargoInfoTool;
pub use metadata::CargoMetadataTool;
pub use search::CargoSearchTool;
pub use test::CargoTestTool;

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
    name = "cargo-generate_lockfile",
    description = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoGenerateLockfileTool {
    /// The name of the package to generate lockfile for. If not specified, generates for the current package/workspace.
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

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

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoGenerateLockfileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

        // Package selection
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
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

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-build",
    description = "Builds a Rust project using Cargo. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoBuildTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package to build. If not specified, the current package/workspace is built.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Build all packages in the workspace
    #[serde(default)]
    workspace: bool,  

    /// Exclude packages from the build
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Build only this package's library
    #[serde(default)]
    lib: bool,

    /// Build all binaries
    #[serde(default)]
    bins: bool,

    /// Build only the specified binary
    #[serde(default, deserialize_with = "deserialize_string")]
    bin: Option<String>,

    /// Build all examples
    #[serde(default)]
    examples: bool,

    /// Build only the specified example
    #[serde(default, deserialize_with = "deserialize_string")]
    example: Option<String>,

    /// Build all targets that have `test = true` set
    #[serde(default)]
    tests: bool,

    /// Build only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Build all targets that have `bench = true` set
    #[serde(default)]
    benches: bool,

    /// Build only the specified bench target
    #[serde(default, deserialize_with = "deserialize_string")]
    bench: Option<String>,

    /// Build all targets
    #[serde(default)]
    all_targets: bool,

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

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,

    /// Treat warnings as errors
    #[serde(default)]
    warnings_as_errors: bool,
}

impl CargoBuildTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("build");

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

        if self.tests {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.benches {
            cmd.arg("--benches");
        }

        if let Some(bench) = &self.bench {
            cmd.arg("--bench").arg(bench);
        }

        if self.all_targets {
            cmd.arg("--all-targets");
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

        if self.quiet {
            cmd.arg("--quiet");
        }

        if self.warnings_as_errors {
            cmd.env("RUSTFLAGS", "-D warnings");
        }

        execute_command(cmd)
    }
}

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCleanTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package to clean. If not specified, cleans the entire workspace.
    #[serde(default, deserialize_with = "deserialize_string")]
    package: Option<String>,

    /// Clean artifacts of the specified profile. If not specified, cleans everything.
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    #[serde(default, deserialize_with = "deserialize_string")]
    profile: Option<String>,

    /// Whether or not to clean just the documentation directory
    #[serde(default)]
    doc: bool,

    /// Display what would be deleted without deleting anything
    #[serde(default)]
    dry_run: bool,

    /// Whether or not to clean release artifacts
    #[serde(default)]
    release: bool,

    /// Target triple to clean output for
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

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
    locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoCleanTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("clean");

        // Package selection
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        // Compilation options
        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if self.doc {
            cmd.arg("--doc");
        }

        if self.dry_run {
            cmd.arg("--dry-run");
        }

        if self.release {
            cmd.arg("--release");
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

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

/// MCP defaults differ from cargo defaults: `quiet` is `true` by default
/// for better integration with automated tooling.
#[mcp_tool(
    name = "cargo-fmt",
    description = "Formats Rust code using rustfmt. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoFmtTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package(s) to format. If not specified, formats the current package.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Format all packages, and also their local path-based dependencies
    #[serde(default)]
    all: bool,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default)]
    check: bool,

    /// Specify path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Specify message-format: short|json|human
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// No output printed to stdout. By default is `true`.
    #[serde(default = "default_true")]
    quiet: bool,
}

impl CargoFmtTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("fmt");

        // Package selection
        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.all {
            cmd.arg("--all");
        }

        // Formatting options
        if self.check {
            cmd.arg("--check");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }

        // Output options
        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

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

    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
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
            cmd.arg(format!("+{}", toolchain));
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
        if self.quiet {
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
    
    /// Assert that `Cargo.lock` will remain unchanged. By default is `true`.
    #[serde(default = "default_true")]
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
            cmd.arg(format!("+{}", toolchain));
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
        if self.quiet {
            cmd.arg("--quiet");
        }
        
        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-list",
    description = "Lists installed cargo commands using 'cargo --list'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoListTool {}

impl CargoListTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("--list");
        execute_command(cmd)
    }
}
