pub mod cargo;
pub mod cargo_deny;
pub mod cargo_hack;
pub mod cargo_machete;
pub mod rustc;
pub mod rustup;

use rust_mcp_sdk::schema::{
    Annotations, CallToolRequest, CallToolResult, Role, TextContent, schema_utils::CallToolError,
};

use cargo::CargoCheckRmcpTool;
use cargo::{
    CargoAddTool, CargoBuildTool, CargoCheckRequest, CargoCleanTool, CargoClippyTool, CargoDocTool,
    CargoFmtTool, CargoGenerateLockfileTool, CargoInfoTool, CargoListTool, CargoMetadataTool,
    CargoNewTool, CargoPackageTool, CargoRemoveTool, CargoSearchTool, CargoTestTool,
    CargoUpdateTool,
};
use cargo_deny::{CargoDenyCheckTool, CargoDenyInitTool, CargoDenyInstallTool, CargoDenyListTool};
use cargo_hack::{CargoHackInstallTool, CargoHackTool};
use cargo_machete::{CargoMacheteInstallTool, CargoMacheteTool};
use rustc::RustcExplainTool;
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

fn execute_command(
    mut cmd: std::process::Command,
    tool_name: &str,
) -> Result<CallToolResult, CallToolError> {
    apply_workspace_root(&mut cmd);
    tracing::info!(
        command = ?cmd,
        tool_name = tool_name,
        "Executing command"
    );
    let output = cmd.output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(output.stdout.trim_ascii());
            let stderr = String::from_utf8_lossy(output.stderr.trim_ascii());

            let mut content = Vec::new();
            if output.status.success() {
                tracing::info!(
                    stdout = ?stdout,
                    stderr = ?stderr,
                    tool_name = tool_name,
                    "Command executed successfully"
                );
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    last_modified: None,
                    priority: Some(0.3),
                });
                content.push(
                    TextContent::new(format!("✅ {tool_name}: Success"), annotations, None).into(),
                );
            } else {
                tracing::warn!(
                    stdout = ?stdout,
                    stderr = ?stderr,
                    status = ?output.status,
                    tool_name = tool_name,
                    "Command execution failed",
                );
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    last_modified: None,
                    priority: Some(0.3),
                });
                content.push(
                    TextContent::new(format!("❌ {tool_name}: Failure"), annotations, None).into(),
                );
            }

            if !stdout.is_empty() {
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    last_modified: None,
                    priority: Some(0.2),
                });
                content.push(TextContent::new(stdout.into(), annotations, None).into());
            }
            if !stderr.is_empty() {
                let annotations = Some(Annotations {
                    audience: vec![Role::User, Role::Assistant],
                    last_modified: None,
                    priority: Some(1.),
                });
                content.push(TextContent::new(stderr.into(), annotations, None).into());
            }
            Ok(CallToolResult {
                content,
                is_error: Some(!output.status.success()),
                meta: None,
                structured_content: None,
            })
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::error!(error = %e, "Command not found");
            let annotations = Some(Annotations {
                audience: vec![Role::User, Role::Assistant],
                last_modified: None,
                priority: Some(1.),
            });
            let program = cmd.get_program().to_string_lossy();
            let item = TextContent::new(
                format!(
                    "The command `{program}` was not found, please ensure it is installed and accessible. You can try running the following command yourself to verify: `{program} {}`",
                    cmd.get_args()
                        .map(|arg| arg.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                annotations,
                None,
            );

            Ok(CallToolResult {
                content: vec![item.into()],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            })
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to execute command");
            Err(CallToolError::new(e))
        }
    }
}
