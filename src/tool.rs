use rmcp::ErrorData;
use rmcp::model::{
    AnnotateAble, Annotated, Annotations, CallToolRequestParam, CallToolResult, RawContent,
    RawTextContent, Role, TextContent,
};

/// Dyn compatible Tool trait
pub(crate) trait Tool {
    fn name(&self) -> &'static str;
    fn json_schema(&self) -> serde_json::Map<String, serde_json::Value>;
    fn call_rmcp_tool(&self, request: CallToolRequestParam) -> Result<CallToolResult, ErrorData>;
}

/// Actual trait that all tools must implement
pub(crate) trait ToolImpl {
    const NAME: &'static str;
    type RequestArgs: serde::de::DeserializeOwned + schemars::JsonSchema;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData>;
}

impl<T> Tool for T
where
    T: ToolImpl,
{
    fn name(&self) -> &'static str {
        T::NAME
    }

    fn json_schema(&self) -> serde_json::Map<String, serde_json::Value> {
        use schemars::schema_for;
        use serde_json::Value;

        let schema = schema_for!(T::RequestArgs).to_value();
        if let serde_json::Value::Object(mut map) = schema {
            map.remove("$schema");

            // Gemini doesn't like "type": ["string", "null"]
            let null_string = Value::String("null".to_string());
            if let Some(Value::Object(props_map)) = map.get_mut("properties") {
                for value in props_map.values_mut() {
                    if let Value::Object(prop_obj) = value
                        && let Some(Value::Array(ty)) = prop_obj.get("type")
                        && ty.len() == 2
                        && ty.contains(&null_string)
                    {
                        let new_ty = ty.iter().find(|v| v != &&null_string).cloned();

                        if let Some(new_ty) = new_ty {
                            prop_obj.insert("type".to_string(), new_ty);
                        }
                    }
                }
            }

            map
        } else {
            panic!("Expected schema to be an object, got: {schema:?}");
        }
    }

    fn call_rmcp_tool(&self, request: CallToolRequestParam) -> Result<CallToolResult, ErrorData> {
        let Some(args) = request.arguments else {
            return Err(ErrorData::invalid_params("Missing tool arguments", None));
        };

        let args: T::RequestArgs = serde_json::from_value(args.into()).map_err(|e| {
            ErrorData::invalid_params(format!("Failed to parse tool arguments: {e}"), None)
        })?;

        self.call_rmcp_tool(args)
    }
}

pub(crate) fn execute_rmcp_command(
    mut cmd: std::process::Command,
    tool_name: &str,
) -> Result<CallToolResult, ErrorData> {
    // apply_workspace_root(&mut cmd); // FIXME
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

            let mut content: Vec<Annotated<RawContent>> = Vec::new();
            if output.status.success() {
                tracing::info!(%stdout, %stderr, %tool_name, "Command executed successfully");
                content.push(
                    RawContent::text(format!("✅ {tool_name}: Success")).annotate(Annotations {
                        audience: Some(vec![Role::User, Role::Assistant]),
                        last_modified: None,
                        priority: Some(0.3),
                    }),
                );
            } else {
                tracing::warn!(
                    %stdout,
                    %stderr,
                    status = output.status.code(),
                    %tool_name,
                    "Command execution failed",
                );
                content.push(
                    RawContent::text(format!("❌ {tool_name}: Failure")).annotate(Annotations {
                        audience: Some(vec![Role::User, Role::Assistant]),
                        last_modified: None,
                        priority: Some(0.3),
                    }),
                );
            }

            if !stdout.is_empty() {
                content.push(RawContent::text(stdout).annotate(Annotations {
                    audience: Some(vec![Role::User, Role::Assistant]),
                    last_modified: None,
                    priority: Some(0.2),
                }));
            }
            if !stderr.is_empty() {
                content.push(RawContent::text(stderr).annotate(Annotations {
                    audience: Some(vec![Role::User, Role::Assistant]),
                    last_modified: None,
                    priority: Some(1.),
                }));
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
            let program = cmd.get_program().to_string_lossy();
            let args = cmd
                .get_args()
                .map(|arg| arg.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ");
            let item = RawContent::text(
                format!(
                    "The command `{program}` was not found, please ensure it is installed and accessible. You can try running the following command yourself to verify: `{program} {args}`",
                )).annotate(
                Annotations {
                    audience: Some(vec![Role::User, Role::Assistant]),
                    last_modified: None,
                    priority: Some(1.),
                });

            Ok(CallToolResult {
                content: vec![item],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            })
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to execute command");
            Err(ErrorData::internal_error(e.to_string(), None))
        }
    }
}
