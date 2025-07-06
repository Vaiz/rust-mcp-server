use std::process::Command;

use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::deserialize_string_vec;
use crate::tools::execute_command;

use crate::serde_utils::Tool;

#[mcp_tool(
    name = "cargo-machete",
    description = "Finds unused dependencies in a fast yet imprecise way. Helps identify dependencies that are declared in Cargo.toml but not actually used in the code.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoMacheteTool {
    /// Uses cargo-metadata to figure out the dependencies' names. May be useful if some dependencies are renamed.
    #[serde(default)]
    with_metadata: bool,

    /// Don't analyze anything contained in any target/ directories encountered.
    #[serde(default)]
    skip_target_dir: bool,

    /// Rewrite the Cargo.toml files to automatically remove unused dependencies.
    /// Note: all dependencies flagged by cargo-machete will be removed, including false positives.
    #[serde(default)]
    fix: bool,

    /// Also search in ignored files (.gitignore, .ignore, etc.) when searching for files.
    #[serde(default)]
    no_ignore: bool,

    /// Paths to analyze. If not specified, analyzes the current directory.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    paths: Option<Vec<String>>,
}

impl CargoMacheteTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("machete");

        if self.with_metadata {
            cmd.arg("--with-metadata");
        }

        if self.skip_target_dir {
            cmd.arg("--skip-target-dir");
        }

        if self.fix {
            cmd.arg("--fix");
        }

        if self.no_ignore {
            cmd.arg("--no-ignore");
        }

        if let Some(paths) = &self.paths {
            for path in paths {
                cmd.arg(path);
            }
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-machete-install",
    description = "Installs cargo-machete tool for finding unused dependencies",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoMacheteInstallTool {}

impl CargoMacheteInstallTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-machete");

        execute_command(cmd)
    }
}
