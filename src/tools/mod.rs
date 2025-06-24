pub mod cargo;
pub mod test;

use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, Annotations, CallToolResult, CallToolResultContentItem, Role
};

use test::{SayHelloTool, SayGoodbyeTool};
use cargo::CargoBuildTool;

fn execute_command(mut cmd: std::process::Command) -> Result<CallToolResult, CallToolError> {
    let output = cmd.output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout.trim_ascii());
            let stderr = String::from_utf8_lossy(&output.stderr.trim_ascii());

            let mut content = Vec::new();
            if !stdout.is_empty() {
                let annotations = Some(Annotations{
                    audience: vec![Role::User, Role::Assistant],
                    priority: Some(0.1)
                });
                content.push(CallToolResultContentItem::text_content(stdout.into(), annotations));
            }
            if !stderr.is_empty() {
                let annotations = Some(Annotations{
                    audience: vec![Role::User, Role::Assistant],
                    priority: Some(1.)
                });
                content.push(CallToolResultContentItem::text_content(stderr.into(), annotations));
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

rust_mcp_sdk::tool_box!(AllTools, [SayHelloTool, SayGoodbyeTool, CargoBuildTool]);