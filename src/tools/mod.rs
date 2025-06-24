pub mod cargo;
pub mod test;

use rust_mcp_sdk::schema::{
    CallToolResult, CallToolResultContentItem, schema_utils::CallToolError,
};

use test::{SayHelloTool, SayGoodbyeTool};
use cargo::CargoBuildTool;

fn execute_command(mut cmd: std::process::Command) -> Result<CallToolResult, CallToolError> {
    let output = cmd.output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            let content = vec![
                CallToolResultContentItem::text_content(stdout.to_string(), None),
                CallToolResultContentItem::text_content(stderr.to_string(), None),
            ];
            Ok(CallToolResult {
                content,
                is_error: Some(!output.status.success()),
                meta: None,
            })
        }
        Err(e) => Err(CallToolError::new(e)),
    }
}

rust_mcp_sdk::tool_box!(AllTools, [SayHelloTool, SayGoodbyeTool, CargoBuildTool]);