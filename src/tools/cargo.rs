use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo.build",
    description = "Builds a Rust project using Cargo"
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
    name = "cargo.clean",
    description = "Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace."
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
