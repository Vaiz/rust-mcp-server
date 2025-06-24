use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-generate_lockfile",
    description = "Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoGenerateLockfileTool {
    /// The name of the package to generate lockfile for. If not specified, generates for the current package/workspace.
    package: Option<String>,
}

impl CargoGenerateLockfileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("generate-lockfile");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
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
    /// The name of the package to build. If not specified, the current package/workspace is built.
    package: Option<String>,

    /// The profile to use for the build. Defaults to "dev".
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    profile: Option<String>,
}

impl CargoBuildTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        cmd.arg("--locked");

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCleanTool {
    /// The name of the package to clean. If not specified, cleans the entire workspace.
    package: Option<String>,

    /// The profile to use for the build. Defaults to "dev".
    /// Default rust profiles:
    /// - `dev`: Optimized for development, with debug information.
    /// - `release`: Optimized for performance, without debug information.
    profile: Option<String>,
}

impl CargoCleanTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clean");

        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-fmt",
    description = "Formats Rust code using rustfmt. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoFmtTool {
    /// The name of the package(s) to format. If not specified, formats the current package.
    package: Option<Vec<String>>,

    /// Format all packages in the workspace and their dependencies
    #[serde(default)]
    all: bool,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default)]
    check: bool,

    /// No output printed to stdout
    #[serde(default)]
    quiet: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,
}

impl CargoFmtTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("fmt");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.all {
            cmd.arg("--all");
        }

        if self.check {
            cmd.arg("--check");
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-check",
    description = "Checks a Rust package and all of its dependencies for errors. Usually, run without any additional arguments.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoCheckTool {
    /// Package(s) to check
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default)]
    release: bool,

    /// Check for the specified target triple
    target: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default)]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default)]
    lib: bool,

    /// Check all binaries
    #[serde(default)]
    bins: bool,

    /// Check all examples
    #[serde(default)]
    examples: bool,

    /// Check all tests
    #[serde(default)]
    tests: bool,

    /// Space or comma separated list of features to activate
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default)]
    no_default_features: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default)]
    quiet: bool,
}

impl CargoCheckTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("check");
        cmd.arg("--locked");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if self.release {
            cmd.arg("--release");
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if self.all_targets {
            cmd.arg("--all-targets");
        }

        if self.lib {
            cmd.arg("--lib");
        }

        if self.bins {
            cmd.arg("--bins");
        }

        if self.examples {
            cmd.arg("--examples");
        }

        if self.tests {
            cmd.arg("--tests");
        }

        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

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
    name = "cargo-clippy",
    description = "Checks a Rust package to catch common mistakes and improve code quality using Clippy",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoClippyTool {
    /// Package(s) to check
    package: Option<Vec<String>>,

    /// Check all packages in the workspace
    #[serde(default)]
    workspace: bool,

    /// Run Clippy only on the given crate, without linting the dependencies
    #[serde(default)]
    no_deps: bool,

    /// Allow dirty working directory (unstaged changes)
    #[serde(default)]
    allow_dirty: bool,

    /// Automatically apply lint suggestions (implies --no-deps and --all-targets)
    #[serde(default)]
    fix: bool,

    /// Check artifacts in release mode, with optimizations
    #[serde(default)]
    release: bool,

    /// Check for the specified target triple
    target: Option<String>,

    /// Check all targets (lib, bins, examples, tests, benches)
    #[serde(default)]
    all_targets: bool,

    /// Check only this package's library
    #[serde(default)]
    lib: bool,

    /// Check all binaries
    #[serde(default)]
    bins: bool,

    /// Check all examples
    #[serde(default)]
    examples: bool,

    /// Check all tests
    #[serde(default)]
    tests: bool,

    /// Space or comma separated list of features to activate
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the default feature
    #[serde(default)]
    no_default_features: bool,

    /// Use verbose output
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default)]
    quiet: bool,

    /// Additional clippy arguments (e.g., lint warnings/denials)
    clippy_args: Option<Vec<String>>,
}

impl CargoClippyTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clippy");
        cmd.arg("--locked");

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if self.no_deps {
            cmd.arg("--no-deps");
        }

        if self.fix {
            cmd.arg("--fix");
        }

        if self.allow_dirty {
            cmd.arg("--allow-dirty");
        }

        if self.release {
            cmd.arg("--release");
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if self.all_targets {
            cmd.arg("--all-targets");
        }

        if self.lib {
            cmd.arg("--lib");
        }

        if self.bins {
            cmd.arg("--bins");
        }

        if self.examples {
            cmd.arg("--examples");
        }

        if self.tests {
            cmd.arg("--tests");
        }

        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.quiet {
            cmd.arg("--quiet");
        }

        // Add clippy-specific arguments after --
        if let Some(clippy_args) = &self.clippy_args {
            if !clippy_args.is_empty() {
                cmd.arg("--");
                for arg in clippy_args {
                    cmd.arg(arg);
                }
            }
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-add",
    description = "Adds a dependency to a Rust project using cargo add.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoAddTool {
    /// The name of the dependency to add.
    pub package: String,
    /// Optional version requirement.
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
}

impl CargoAddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("add");
        if let Some(version) = &self.version {
            cmd.arg(format!("{}@{version}", self.package));
        } else {
            cmd.arg(&self.package);
        }
        if self.dev {
            cmd.arg("--dev");
        }
        if self.build {
            cmd.arg("--build");
        }
        if self.optional {
            cmd.arg("--optional");
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

#[mcp_tool(
    name = "cargo-test",
    description = "Run `cargo test` to execute Rust tests in the current project.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoTestTool {
    /// If specified, only run tests containing this string in their names
    testname: Option<String>,
    /// Arguments for the test binary (after --)
    test_args: Option<Vec<String>>,
    /// Package to run tests for
    package: Option<String>,
    /// Test all packages in the workspace
    #[serde(default)]
    workspace: bool,
    /// Exclude packages from the test
    exclude: Option<Vec<String>>,
    /// Test only this package's library
    #[serde(default)]
    lib: bool,
    /// Test all binaries
    #[serde(default)]
    bins: bool,
    /// Test only the specified binary
    bin: Option<String>,
    /// Test all examples
    #[serde(default)]
    examples: bool,
    /// Test only the specified example
    example: Option<String>,
    /// Test all targets that have `test = true` set
    #[serde(default)]
    tests: bool,
    /// Test only the specified test target
    test: Option<String>,
    /// Test all targets that have `bench = true` set
    #[serde(default)]
    benches: bool,
    /// Test only the specified bench target
    bench: Option<String>,
    /// Test all targets (does not include doctests)
    #[serde(default)]
    all_targets: bool,
    /// Test only this library's documentation
    #[serde(default)]
    doc: bool,
    /// Space or comma separated list of features to activate
    features: Option<String>,
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
    profile: Option<String>,
    /// Build for the target triple
    target: Option<String>,
    /// Path to Cargo.toml
    manifest_path: Option<String>,
}

impl CargoTestTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("test");
        cmd.arg("--locked");
        if let Some(testname) = &self.testname {
            cmd.arg(testname);
        }
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }
        if self.workspace {
            cmd.arg("--workspace");
        }
        if let Some(excludes) = &self.exclude {
            for ex in excludes {
                cmd.arg("--exclude").arg(ex);
            }
        }
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
        if self.doc {
            cmd.arg("--doc");
        }
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features);
        }
        if self.all_features {
            cmd.arg("--all-features");
        }
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }
        if self.release {
            cmd.arg("--release");
        }
        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        // Pass test binary args after --
        if let Some(test_args) = &self.test_args {
            cmd.arg("--");
            for arg in test_args {
                cmd.arg(arg);
            }
        }
        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-metadata",
    description = "Outputs a listing of a project's resolved dependencies and metadata in machine-readable format (JSON).",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoMetadataTool {
    /// Only include resolve dependencies matching the given target-triple
    filter_platform: Option<String>,

    /// Output information only about the workspace members and don't fetch dependencies
    #[serde(default)]
    no_deps: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[serde(default)]
    verbose: bool,

    /// Do not print cargo log messages
    #[serde(default)]
    quiet: bool,

    /// Coloring [possible values: auto, always, never]
    color: Option<String>,

    /// Override a configuration value
    config: Option<String>,

    /// Space or comma separated list of features to activate
    features: Option<String>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: bool,

    /// Path to Cargo.toml
    manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    lockfile_path: Option<String>,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    frozen: bool,
}

impl CargoMetadataTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("metadata");
        cmd.arg("--locked");
        cmd.arg("--format-version").arg("1");

        if let Some(ref triple) = self.filter_platform {
            cmd.arg("--filter-platform").arg(triple);
        }
        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if self.verbose {
            cmd.arg("--verbose");
        }
        if self.quiet {
            cmd.arg("--quiet");
        }
        if let Some(ref color) = self.color {
            cmd.arg("--color").arg(color);
        }
        if let Some(ref config) = self.config {
            cmd.arg("--config").arg(config);
        }
        if let Some(ref features) = self.features {
            cmd.arg("--features").arg(features);
        }
        if self.all_features {
            cmd.arg("--all-features");
        }
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }
        if let Some(ref manifest_path) = self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(ref lockfile_path) = self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }
        if self.offline {
            cmd.arg("--offline");
        }
        if self.frozen {
            cmd.arg("--frozen");
        }

        execute_command(cmd)
    }
}
