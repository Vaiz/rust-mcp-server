use std::process::Command;

use crate::{Tool, execute_rmcp_command, serde_utils::deserialize_string_vec};
use rmcp::{ErrorData, model::CallToolResult};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoMacheteRequest {
    /// Uses cargo-metadata to figure out the dependencies' names. May be useful if some dependencies are renamed.
    #[serde(default)]
    with_metadata: Option<bool>,

    /// Don't analyze anything contained in any target/ directories encountered.
    #[serde(default)]
    skip_target_dir: Option<bool>,

    /// Rewrite the Cargo.toml files to automatically remove unused dependencies.
    /// Note: all dependencies flagged by cargo-machete will be removed, including false positives.
    #[serde(default)]
    fix: Option<bool>,

    /// Also search in ignored files (.gitignore, .ignore, etc.) when searching for files.
    #[serde(default)]
    no_ignore: Option<bool>,

    /// Paths to analyze. If not specified, analyzes the current directory.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    paths: Option<Vec<String>>,
}

impl CargoMacheteRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("machete");

        if self.with_metadata.unwrap_or(false) {
            cmd.arg("--with-metadata");
        }

        if self.skip_target_dir.unwrap_or(false) {
            cmd.arg("--skip-target-dir");
        }

        if self.fix.unwrap_or(false) {
            cmd.arg("--fix");
        }

        if self.no_ignore.unwrap_or(false) {
            cmd.arg("--no-ignore");
        }

        if let Some(paths) = &self.paths {
            for path in paths {
                cmd.arg(path);
            }
        }

        Ok(cmd)
    }
}

pub struct CargoMacheteRmcpTool;

impl Tool for CargoMacheteRmcpTool {
    const NAME: &'static str = "cargo-machete";
    const TITLE: &'static str = "Find unused dependencies";
    const DESCRIPTION: &'static str = "Finds unused dependencies in a fast yet imprecise way. Helps identify dependencies that are declared in Cargo.toml but not actually used in the code.";
    type RequestArgs = CargoMacheteRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_rmcp_command(request.build_cmd()?, Self::NAME)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoMacheteInstallRequest {}

impl CargoMacheteInstallRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-machete");

        Ok(cmd)
    }
}

pub struct CargoMacheteInstallRmcpTool;

impl Tool for CargoMacheteInstallRmcpTool {
    const NAME: &'static str = "cargo-machete-install";
    const TITLE: &'static str = "Install cargo-machete";
    const DESCRIPTION: &'static str = "Installs cargo-machete tool for finding unused dependencies";
    type RequestArgs = CargoMacheteInstallRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_rmcp_command(request.build_cmd()?, Self::NAME)
    }
}
