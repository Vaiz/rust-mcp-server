pub mod cargo;
pub mod cargo_deny;
pub mod cargo_hack;
pub mod cargo_machete;
pub mod rustup;

use rust_mcp_sdk::schema::{
    Annotations, CallToolRequest, CallToolResult, CallToolResultContentItem, Role,
    schema_utils::CallToolError,
};

use cargo::{
    CargoAddTool, CargoBuildTool, CargoCheckTool, CargoCleanTool, CargoClippyTool, CargoFmtTool,
    CargoGenerateLockfileTool, CargoInfoTool, CargoListTool, CargoMetadataTool, CargoNewTool,
    CargoPackageTool, CargoRemoveTool, CargoSearchTool, CargoTestTool, CargoUpdateTool,
};
use cargo_deny::{CargoDenyCheckTool, CargoDenyInitTool, CargoDenyInstallTool, CargoDenyListTool};
use cargo_hack::{CargoHackInstallTool, CargoHackTool};
use cargo_machete::{CargoMacheteInstallTool, CargoMacheteTool};
use rustup::{RustupShowTool, RustupToolchainAddTool, RustupUpdateTool};

static WORKSPACE_ROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_workspace_root(root: String) {
    WORKSPACE_ROOT
        .set(root)
        .expect("Workspace root can only be set once");
}

fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = WORKSPACE_ROOT.get() {
        cmd.current_dir(root);
    }
}

fn execute_command(mut cmd: std::process::Command) -> Result<CallToolResult, CallToolError> {
    apply_workspace_root(&mut cmd);
    tracing::info!(
        command = ?cmd,
        "Executing command"
    );
    let output = cmd.output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(output.stdout.trim_ascii());
            let stderr = String::from_utf8_lossy(output.stderr.trim_ascii());

            if output.status.success() {
                tracing::info!(
                    stdout = ?stdout,
                    stderr = ?stderr,
                    "Command executed successfully"
                );
            } else {
                tracing::warn!(
                    stdout = ?stdout,
                    stderr = ?stderr,
                    status = ?output.status,
                    "Command execution failed",
                );
            }

            let mut content = Vec::new();
            if !stdout.is_empty() {
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    priority: Some(0.1),
                });
                content.push(CallToolResultContentItem::text_content(
                    stdout.into(),
                    annotations,
                ));
            }
            if !stderr.is_empty() {
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    priority: Some(1.),
                });
                content.push(CallToolResultContentItem::text_content(
                    stderr.into(),
                    annotations,
                ));
            }
            Ok(CallToolResult {
                content,
                is_error: Some(!output.status.success()),
                meta: None,
            })
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::error!(error = ?e, "Command not found");
            let annotations = Some(Annotations {
                audience: vec![Role::User, Role::Assistant],
                priority: Some(1.),
            });
            let program = cmd.get_program().to_string_lossy();
            let item = CallToolResultContentItem::text_content(
                format!(
                    "The command `{program}` was not found, please ensure it is installed and accessible. You can try running the following command yourself to verify: `{program} {}`",
                    cmd.get_args()
                        .map(|arg| arg.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                annotations,
            );

            Ok(CallToolResult {
                content: vec![item],
                is_error: Some(true),
                meta: None,
            })
        }
        Err(e) => {
            tracing::error!(error = ?e, "Failed to execute command");
            Err(CallToolError::new(e))
        }
    }
}

rust_mcp_sdk::tool_box!(
    AllTools,
    [
        CargoBuildTool,
        CargoCleanTool,
        CargoFmtTool,
        CargoCheckTool,
        CargoClippyTool,
        CargoGenerateLockfileTool,
        CargoHackTool,
        CargoHackInstallTool,
        CargoMacheteTool,
        CargoMacheteInstallTool,
        CargoDenyCheckTool,
        CargoDenyInitTool,
        CargoDenyListTool,
        CargoDenyInstallTool,
        CargoAddTool,
        CargoRemoveTool,
        CargoNewTool,
        CargoListTool,
        CargoPackageTool,
        CargoUpdateTool,
        CargoTestTool,
        CargoMetadataTool,
        RustupShowTool,
        RustupToolchainAddTool,
        RustupUpdateTool,
        CargoSearchTool,
        CargoInfoTool
    ]
);

/// Handles incoming CallToolRequest and processes it using the appropriate tool.
pub fn handle_request(
    request: CallToolRequest,
    disabled_tools: &[String],
) -> Result<CallToolResult, CallToolError> {
    if disabled_tools.contains(&request.params.name) {
        tracing::warn!(
            tool_name = ?request.params.name,
            "Tool is disabled, returning error"
        );
        return Err(CallToolError::unknown_tool(request.params.name));
    }

    let tool_params: AllTools = AllTools::try_from(request.params.clone()).map_err(|e| {
        tracing::error!(
            error = ?e,
            request = ?request,
            "Failed to parse request parameters"
        );
        CallToolError::new(e)
    })?;

    match tool_params {
        AllTools::CargoBuildTool(tool) => tool.call_tool(),
        AllTools::CargoCleanTool(tool) => tool.call_tool(),
        AllTools::CargoFmtTool(tool) => tool.call_tool(),
        AllTools::CargoCheckTool(tool) => tool.call_tool(),
        AllTools::CargoClippyTool(tool) => tool.call_tool(),
        AllTools::CargoGenerateLockfileTool(tool) => tool.call_tool(),
        AllTools::CargoHackTool(tool) => tool.call_tool(),
        AllTools::CargoHackInstallTool(tool) => tool.call_tool(),
        AllTools::CargoMacheteTool(tool) => tool.call_tool(),
        AllTools::CargoMacheteInstallTool(tool) => tool.call_tool(),
        AllTools::CargoDenyCheckTool(tool) => tool.call_tool(),
        AllTools::CargoDenyInitTool(tool) => tool.call_tool(),
        AllTools::CargoDenyListTool(tool) => tool.call_tool(),
        AllTools::CargoDenyInstallTool(tool) => tool.call_tool(),
        AllTools::CargoAddTool(tool) => tool.call_tool(),
        AllTools::CargoRemoveTool(tool) => tool.call_tool(),
        AllTools::CargoNewTool(tool) => tool.call_tool(),
        AllTools::CargoListTool(tool) => tool.call_tool(),
        AllTools::CargoPackageTool(tool) => tool.call_tool(),
        AllTools::CargoUpdateTool(tool) => tool.call_tool(),
        AllTools::CargoTestTool(tool) => tool.call_tool(),
        AllTools::CargoMetadataTool(tool) => tool.call_tool(),
        AllTools::RustupShowTool(tool) => tool.call_tool(),
        AllTools::RustupToolchainAddTool(tool) => tool.call_tool(),
        AllTools::RustupUpdateTool(tool) => tool.call_tool(),
        AllTools::CargoSearchTool(tool) => tool.call_tool(),
        AllTools::CargoInfoTool(tool) => tool.call_tool(),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    #[test]
    fn test_execute_command_with_nonexistent_tool() {
        // Try to execute a command that does not exist
        let mut cmd = Command::new("this_tool_does_not_exist");
        cmd.args(["--arg1", "value1", "--arg2", "value2"]);

        let result = execute_command(cmd).expect("Command execution should not panic");
        let text = &result.content[0]
            .as_text_content()
            .expect("First content item should be text")
            .text;

        println!("Command output: {text}");

        assert_eq!(result.is_error, Some(true));
        assert!(!result.content.is_empty());
        assert!(text.contains("The command `this_tool_does_not_exist` was not found"));
    }
}
