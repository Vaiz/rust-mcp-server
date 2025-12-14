use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};
use rmcp::{ErrorData, model::CallToolResult};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupShowRequest {
    /// Enable verbose output with rustc information for all installed toolchains
    #[serde(default)]
    verbose: bool,
}

impl RustupShowRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("rustup");
        cmd.arg("show");

        if self.verbose {
            cmd.arg("--verbose");
        }

        Ok(cmd)
    }
}

pub struct RustupShowRmcpTool;

impl Tool for RustupShowRmcpTool {
    const NAME: &'static str = "rustup-show";
    const TITLE: &'static str = "Show Rust toolchains";
    const DESCRIPTION: &'static str = "Show the active and installed toolchains or profiles. Shows the name of the active toolchain and the version of rustc. If the active toolchain has installed support for additional compilation targets, then they are listed as well.";
    type RequestArgs = RustupShowRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupToolchainAddRequest {
    /// Toolchain name, such as 'stable', 'nightly', or '1.8.0'
    pub toolchain: String,

    /// Profile to use for installation (minimal, default, complete)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub profile: Option<String>,

    /// Comma-separated list of components to be added on installation
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub components: Option<Vec<String>>,

    /// Comma-separated list of targets to be added on installation
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub targets: Option<Vec<String>>,

    /// Don't perform self update when running the command
    #[serde(default)]
    pub no_self_update: bool,

    /// Force an update, even if some components are missing
    #[serde(default)]
    pub force: bool,

    /// Allow rustup to downgrade the toolchain to satisfy your component choice
    #[serde(default)]
    pub allow_downgrade: bool,

    /// Install toolchains that require an emulator
    #[serde(default)]
    pub force_non_host: bool,
}

impl RustupToolchainAddRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("rustup");
        cmd.arg("toolchain").arg("install").arg(&self.toolchain);

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(components) = &self.components
            && !components.is_empty()
        {
            cmd.arg("--component").arg(components.join(","));
        }

        if let Some(targets) = &self.targets
            && !targets.is_empty()
        {
            cmd.arg("--target").arg(targets.join(","));
        }

        if self.no_self_update {
            cmd.arg("--no-self-update");
        }

        if self.force {
            cmd.arg("--force");
        }

        if self.allow_downgrade {
            cmd.arg("--allow-downgrade");
        }

        if self.force_non_host {
            cmd.arg("--force-non-host");
        }

        Ok(cmd)
    }
}

pub struct RustupToolchainAddRmcpTool;

impl Tool for RustupToolchainAddRmcpTool {
    const NAME: &'static str = "rustup-toolchain-add";
    const TITLE: &'static str = "Install Rust toolchain";
    const DESCRIPTION: &'static str = "Install or update the given toolchains, or by default the active toolchain. Toolchain name can be 'stable', 'nightly', or a specific version like '1.8.0'.";
    type RequestArgs = RustupToolchainAddRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupUpdateRequest {
    /// Toolchain name to update, such as 'stable', 'nightly', or '1.8.0'. If not specified, updates all installed toolchains
    #[serde(default, deserialize_with = "deserialize_string")]
    pub toolchain: Option<String>,

    /// Don't perform self update when running the command
    #[serde(default)]
    pub no_self_update: bool,

    /// Force an update, even if some components are missing
    #[serde(default)]
    pub force: bool,

    /// Install toolchains that require an emulator
    #[serde(default)]
    pub force_non_host: bool,
}

impl RustupUpdateRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("rustup");
        cmd.arg("update");

        if let Some(toolchain) = &self.toolchain {
            cmd.arg(toolchain);
        }

        if self.no_self_update {
            cmd.arg("--no-self-update");
        }

        if self.force {
            cmd.arg("--force");
        }

        if self.force_non_host {
            cmd.arg("--force-non-host");
        }

        Ok(cmd)
    }
}

pub struct RustupUpdateRmcpTool;

impl Tool for RustupUpdateRmcpTool {
    const NAME: &'static str = "rustup-update";
    const TITLE: &'static str = "Update Rust toolchains";
    const DESCRIPTION: &'static str = "Update Rust toolchains and rustup. With no toolchain specified, updates each of the installed toolchains from the official release channels, then updates rustup itself. If given a toolchain argument then updates that toolchain.";
    type RequestArgs = RustupUpdateRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}
