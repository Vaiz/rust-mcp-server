pub mod cargo;
pub mod cargo_deny;
pub mod cargo_machete;
pub mod test;

use rust_mcp_sdk::schema::{
    Annotations, CallToolRequest, CallToolResult, CallToolResultContentItem, Role,
    schema_utils::CallToolError,
};

use cargo::{
    CargoAddTool, CargoBuildTool, CargoCheckTool, CargoCleanTool, CargoClippyTool, CargoFmtTool,
    CargoGenerateLockfileTool, CargoListTool, CargoMetadataTool, CargoTestTool,
};
use cargo_deny::{CargoDenyCheckTool, CargoDenyInitTool, CargoDenyInstallTool, CargoDenyListTool};
use cargo_machete::{CargoMacheteInstallTool, CargoMacheteTool};
use test::{SayGoodbyeTool, SayHelloTool};

fn execute_command(mut cmd: std::process::Command) -> Result<CallToolResult, CallToolError> {
    tracing::info!(
        command = ?cmd,
        "Executing command"
    );
    let output = cmd.output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(output.stdout.trim_ascii());
            let stderr = String::from_utf8_lossy(output.stderr.trim_ascii());

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
        Err(e) => Err(CallToolError::new(e)),
    }
}

rust_mcp_sdk::tool_box!(
    AllTools,
    [
        SayHelloTool,
        SayGoodbyeTool,
        CargoBuildTool,
        CargoCleanTool,
        CargoFmtTool,
        CargoCheckTool,
        CargoClippyTool,
        CargoGenerateLockfileTool,
        CargoMacheteTool,
        CargoMacheteInstallTool,
        CargoDenyCheckTool,
        CargoDenyInitTool,
        CargoDenyListTool,
        CargoDenyInstallTool,
        CargoAddTool,
        CargoListTool,
        CargoTestTool,
        CargoMetadataTool
    ]
);

/// Handles incoming CallToolRequest and processes it using the appropriate tool.
pub fn handle_request(request: CallToolRequest) -> Result<CallToolResult, CallToolError> {
    // Attempt to convert request parameters into GreetingTools enum
    let tool_params: AllTools = AllTools::try_from(request.params).map_err(CallToolError::new)?;

    // Match the tool variant and execute its corresponding logic
    match tool_params {
        AllTools::SayHelloTool(say_hello_tool) => say_hello_tool.call_tool(),
        AllTools::SayGoodbyeTool(say_goodbye_tool) => say_goodbye_tool.call_tool(),
        AllTools::CargoBuildTool(cargo_build_tool) => cargo_build_tool.call_tool(),
        AllTools::CargoCleanTool(cargo_clean_tool) => cargo_clean_tool.call_tool(),
        AllTools::CargoFmtTool(cargo_fmt_tool) => cargo_fmt_tool.call_tool(),
        AllTools::CargoCheckTool(cargo_check_tool) => cargo_check_tool.call_tool(),
        AllTools::CargoClippyTool(cargo_clippy_tool) => cargo_clippy_tool.call_tool(),
        AllTools::CargoGenerateLockfileTool(cargo_generate_lockfile_tool) => {
            cargo_generate_lockfile_tool.call_tool()
        }
        AllTools::CargoMacheteTool(cargo_machete_tool) => cargo_machete_tool.call_tool(),
        AllTools::CargoMacheteInstallTool(cargo_machete_install_tool) => {
            cargo_machete_install_tool.call_tool()
        }
        AllTools::CargoDenyCheckTool(cargo_deny_check_tool) => cargo_deny_check_tool.call_tool(),
        AllTools::CargoDenyInitTool(cargo_deny_init_tool) => cargo_deny_init_tool.call_tool(),
        AllTools::CargoDenyListTool(cargo_deny_list_tool) => cargo_deny_list_tool.call_tool(),
        AllTools::CargoDenyInstallTool(cargo_deny_install_tool) => {
            cargo_deny_install_tool.call_tool()
        }
        AllTools::CargoAddTool(cargo_add_tool) => cargo_add_tool.call_tool(),
        AllTools::CargoListTool(cargo_list_tool) => cargo_list_tool.call_tool(),
        AllTools::CargoTestTool(cargo_test_tool) => cargo_test_tool.call_tool(),
        AllTools::CargoMetadataTool(cargo_metadata_tool) => cargo_metadata_tool.call_tool(),
    }
}
