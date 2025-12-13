use rmcp::ErrorData;
use rmcp::model::{
    AnnotateAble, Annotated, Annotations, CallToolRequestParam, CallToolResult, RawContent, Role,
};
use schemars::JsonSchema;

use crate::tools::apply_workspace_root;

/// Dyn compatible Tool trait
pub(crate) trait DynTool {
    fn name(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn json_schema(&self) -> serde_json::Map<String, serde_json::Value>;
    fn call_rmcp_tool(&self, request: CallToolRequestParam) -> Result<CallToolResult, ErrorData>;
}

/// Actual trait that all tools must implement
pub(crate) trait Tool {
    const NAME: &'static str;
    const TITLE: &'static str;
    const DESCRIPTION: &'static str;
    type RequestArgs: serde::de::DeserializeOwned + schemars::JsonSchema;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<CallToolResult, ErrorData>;
}

impl<T> DynTool for T
where
    T: Tool,
{
    fn name(&self) -> &'static str {
        T::NAME
    }

    fn title(&self) -> &'static str {
        T::TITLE
    }

    fn description(&self) -> &'static str {
        T::DESCRIPTION
    }

    fn json_schema(&self) -> serde_json::Map<String, serde_json::Value> {
        json_schema_impl::<T::RequestArgs>()
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
    apply_workspace_root(&mut cmd);
    tracing::info!("Executing command for {}: {:?}", tool_name, cmd);
    let output = cmd.output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(output.stdout.trim_ascii());
            let stderr = String::from_utf8_lossy(output.stderr.trim_ascii());

            let mut content: Vec<Annotated<RawContent>> = Vec::new();
            if output.status.success() {
                tracing::info!(
                    "Command executed successfully for {tool_name}\nstdout=\n{stdout}\n\nstderr=\n{stderr}",
                );
                content.push(
                    RawContent::text(format!("✅ {tool_name}: Success")).annotate(Annotations {
                        audience: Some(vec![Role::User, Role::Assistant]),
                        last_modified: None,
                        priority: Some(0.3),
                    }),
                );
            } else {
                tracing::warn!(
                    "Command execution failed for {tool_name} (status: {:?}): stdout='\n{stdout}\n', stderr='\n{stderr}\n'",
                    output.status.code(),
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
            tracing::error!("Command not found: {e}");
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
            tracing::error!("Failed to execute command: {}", e);
            Err(ErrorData::internal_error(e.to_string(), None))
        }
    }
}

fn json_schema_impl<T: JsonSchema>() -> serde_json::Map<String, serde_json::Value> {
    use schemars::schema_for;
    use serde_json::Value;

    let schema = schema_for!(T).to_value();
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

#[cfg(test)]
mod tests {
    use super::json_schema_impl;

    #[test]
    fn test_tool_json_schema_removes_null_type_first() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            #[serde(default)]
            value: Option<String>,
        }

        let schema = json_schema_impl::<Example>();
        let props = schema.get("properties").unwrap();
        let value_schema = props.get("value").unwrap();
        if let serde_json::Value::Object(obj) = value_schema {
            // Should not be an array of types, just "string"
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "string");
        } else {
            panic!("Expected value property to be an object");
        }
    }

    #[test]
    fn test_tool_json_schema_removes_null_type_second() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            #[serde(default)]
            value: Option<i32>,
        }

        let schema = json_schema_impl::<Example>();
        let props = schema.get("properties").unwrap();
        let value_schema = props.get("value").unwrap();
        if let serde_json::Value::Object(obj) = value_schema {
            // Should not be an array of types, just "integer"
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "integer");
        } else {
            panic!("Expected value property to be an object");
        }
    }

    #[test]
    fn test_tool_json_schema_leaves_non_null_type_untouched() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            value: String,
        }

        let schema = json_schema_impl::<Example>();
        let props = schema.get("properties").unwrap();
        let value_schema = props.get("value").unwrap();
        if let serde_json::Value::Object(obj) = value_schema {
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "string");
        } else {
            panic!("Expected value property to be an object");
        }
    }

    #[test]
    fn test_tool_json_schema_handles_multiple_properties() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            #[serde(default)]
            opt: Option<String>,
            num: i32,
        }

        let schema = json_schema_impl::<Example>();
        let props = schema.get("properties").unwrap();
        let opt_schema = props.get("opt").unwrap();
        let num_schema = props.get("num").unwrap();

        if let serde_json::Value::Object(obj) = opt_schema {
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "string");
        } else {
            panic!("Expected opt property to be an object");
        }

        if let serde_json::Value::Object(obj) = num_schema {
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "integer");
        } else {
            panic!("Expected num property to be an object");
        }
    }

    #[test]
    fn test_tool_json_schema_ignores_non_array_type() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            value: bool,
        }

        let schema = json_schema_impl::<Example>();
        let props = schema.get("properties").unwrap();
        let value_schema = props.get("value").unwrap();
        if let serde_json::Value::Object(obj) = value_schema {
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "boolean");
        } else {
            panic!("Expected value property to be an object");
        }
    }
}
