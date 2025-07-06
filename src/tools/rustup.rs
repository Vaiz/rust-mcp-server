use std::process::Command;

use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::serde_utils::{deserialize_string, deserialize_string_vec};
use crate::tools::execute_command;

use crate::serde_utils::Tool;

#[mcp_tool(
    name = "rustup-show",
    description = "Show the active and installed toolchains or profiles. Shows the name of the active toolchain and the version of rustc. If the active toolchain has installed support for additional compilation targets, then they are listed as well.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupShowTool {
    /// Enable verbose output with rustc information for all installed toolchains
    #[serde(default)]
    verbose: bool,
}

impl RustupShowTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("rustup");
        cmd.arg("show");

        if self.verbose {
            cmd.arg("--verbose");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "rustup-toolchain-add",
    description = "Install or update the given toolchains, or by default the active toolchain. Toolchain name can be 'stable', 'nightly', or a specific version like '1.8.0'.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupToolchainAddTool {
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

impl RustupToolchainAddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("rustup");
        cmd.arg("toolchain").arg("install").arg(&self.toolchain);

        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(components) = &self.components {
            if !components.is_empty() {
                cmd.arg("--component").arg(components.join(","));
            }
        }

        if let Some(targets) = &self.targets {
            if !targets.is_empty() {
                cmd.arg("--target").arg(targets.join(","));
            }
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

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "rustup-update",
    description = "Update Rust toolchains and rustup. With no toolchain specified, updates each of the installed toolchains from the official release channels, then updates rustup itself. If given a toolchain argument then updates that toolchain.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct RustupUpdateTool {
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

impl RustupUpdateTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
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

        execute_command(cmd)
    }
}
