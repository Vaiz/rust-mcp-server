use std::process::Command;

use crate::{
    serde_utils::{deserialize_string, deserialize_string_vec},
    tools::execute_command,
};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "cargo-test",
    description = "Run `cargo test` to execute Rust tests in the current project.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoTestTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(deserialize_with = "deserialize_string")]
    toolchain: Option<String>,
    /// If specified, only run tests containing this string in their names
    #[serde(deserialize_with = "deserialize_string")]
    testname: Option<String>,
    /// Arguments for the test binary (after --)
    #[serde(deserialize_with = "deserialize_string_vec")]
    test_args: Option<Vec<String>>,
    /// Package to run tests for
    #[serde(deserialize_with = "deserialize_string")]
    package: Option<String>,
    /// Test all packages in the workspace
    #[serde(default)]
    workspace: bool,
    /// Exclude packages from the test
    #[serde(deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,
    /// Test only this package's library
    #[serde(default)]
    lib: bool,
    /// Test all binaries
    #[serde(default)]
    bins: bool,
    /// Test only the specified binary
    #[serde(deserialize_with = "deserialize_string")]
    bin: Option<String>,
    /// Test all examples
    #[serde(default)]
    examples: bool,
    /// Test only the specified example
    #[serde(deserialize_with = "deserialize_string")]
    example: Option<String>,
    /// Test all targets that have `test = true` set
    #[serde(default)]
    tests: bool,
    /// Test only the specified test target
    #[serde(deserialize_with = "deserialize_string")]
    test: Option<String>,
    /// Test all targets that have `bench = true` set
    #[serde(default)]
    benches: bool,
    /// Test only the specified bench target
    #[serde(deserialize_with = "deserialize_string")]
    bench: Option<String>,
    /// Test all targets (does not include doctests)
    #[serde(default)]
    all_targets: bool,
    /// Test only this library's documentation
    #[serde(default)]
    doc: bool,
    /// Space or comma separated list of features to activate
    #[serde(deserialize_with = "deserialize_string")]
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
    #[serde(deserialize_with = "deserialize_string")]
    profile: Option<String>,
    /// Build for the target triple
    #[serde(deserialize_with = "deserialize_string")]
    target: Option<String>,
    /// Path to Cargo.toml
    #[serde(deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,
}

impl CargoTestTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
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
