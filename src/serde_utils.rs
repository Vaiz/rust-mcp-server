use rust_mcp_sdk::schema::schema_utils::CallToolError;

/// Utility function for parsing Option<String> fields in serde,
/// returning None if the string is "null" (case-insensitive) or empty.
pub fn deserialize_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        Some("null") => Ok(None),
        _ => Ok(opt),
    }
}

/// Utility function for parsing Option<Vec<String>> fields in serde,
/// returning None if the value is a string "null" (case-insensitive).
pub fn deserialize_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) if s.to_lowercase() == "null" => Ok(None),
        Value::Null => Ok(None),
        Value::Array(arr) => {
            let strings: Result<Vec<String>, _> = arr
                .into_iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s),
                    _ => Err(serde::de::Error::custom("Expected string in array")),
                })
                .collect();
            Ok(Some(strings?))
        }
        _ => Err(serde::de::Error::custom("Expected array or null")),
    }
}

/// Convert locking mode string to CLI flags for cargo commands.
/// Returns a vector of flags to add to the command.
/// 
/// Valid modes:
/// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
/// - "unlocked": Allow `Cargo.lock` to be updated  
/// - "offline": Run without accessing the network
/// - "frozen": Equivalent to specifying both --locked and --offline
pub fn locking_mode_to_cli_flags(mode: Option<&str>) -> Result<Vec<&'static str>, CallToolError> {
    Ok(match mode.unwrap_or("locked") {
        "locked" => vec!["--locked"],
        "unlocked" => vec![], // No flags needed
        "offline" => vec!["--offline"], 
        "frozen" => vec!["--frozen"],
        unknown => {
            return Err(CallToolError(
                anyhow::anyhow!(
                    "Unknown locking mode: {}. Valid options are: locked, unlocked, offline, frozen", 
                    unknown
                ).into()
            ));
        }
    })
}

pub const fn default_true() -> bool {
    true
}

/// A type that represents a package with an optional version.
/// When calling cargo commands, use `to_spec()` to get "package" or "package@version" format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, schemars::JsonSchema)]
pub struct PackageWithVersion {
    /// The package name
    pub package: String,
    /// Optional version specification
    #[serde(default, deserialize_with = "deserialize_string")]
    pub version: Option<String>,
}

impl PackageWithVersion {
    /// Create a new PackageWithVersion with just a package name
    #[cfg(test)]
    pub fn new(package: String) -> Self {
        Self {
            package,
            version: None,
        }
    }

    /// Create a new PackageWithVersion with a package name and version
    #[cfg(test)]
    pub fn with_version(package: String, version: String) -> Self {
        Self {
            package,
            version: Some(version),
        }
    }

    /// Get the formatted string representation (package or package@version)
    pub fn to_spec(&self) -> String {
        match &self.version {
            Some(version) => format!("{}@{}", self.package, version),
            None => self.package.clone(),
        }
    }
}

pub trait Tool: schemars::JsonSchema {
    /// Returns the JSON schema for this type.
    fn json_schema() -> serde_json::Map<String, serde_json::Value> {
        use schemars::schema_for;
        use serde_json::Value;

        let schema = schema_for!(Self).to_value();
        if let serde_json::Value::Object(mut map) = schema {
            map.remove("$schema");

            // Gemini doesn't like "type": ["string", "null"]
            let null_string = Value::String("null".to_string());
            if let Some(Value::Object(props_map)) = map.get_mut("properties") {
                for value in props_map.values_mut() {
                    if let Value::Object(prop_obj) = value {
                        if let Some(Value::Array(ty)) = prop_obj.get("type") {
                            if ty.len() == 2 && ty.contains(&null_string) {
                                let new_ty = ty.iter().find(|v| v != &&null_string).cloned();

                                if let Some(new_ty) = new_ty {
                                    prop_obj.insert("type".to_string(), new_ty);
                                }
                            }
                        }
                    }
                }
            }

            map
        } else {
            panic!("Expected schema to be an object, got: {schema:?}");
        }
    }
}

impl<T: schemars::JsonSchema> Tool for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestString {
        #[serde(deserialize_with = "deserialize_string")]
        value: Option<String>,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestStringVec {
        #[serde(deserialize_with = "deserialize_string_vec")]
        value: Option<Vec<String>>,
    }

    #[test]
    fn test_deserialize_string_some() {
        let json = r#"{ "value": "hello" }"#;
        let result: TestString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, Some("hello".to_string()));
    }

    #[test]
    fn test_deserialize_string_null_string() {
        let json = r#"{ "value": "null" }"#;
        let result: TestString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_deserialize_string_null_value() {
        let json = r#"{ "value": null }"#;
        let result: TestString = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_deserialize_string_vec_some() {
        let json = r#"{ "value": ["a", "b", "c"] }"#;
        let result: TestStringVec = serde_json::from_str(json).unwrap();
        assert_eq!(
            result.value,
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn test_deserialize_string_vec_null_string() {
        let json = r#"{ "value": "null" }"#;
        let result: TestStringVec = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_deserialize_string_vec_null_value() {
        let json = r#"{ "value": null }"#;
        let result: TestStringVec = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_deserialize_string_vec_empty_array() {
        let json = r#"{ "value": [] }"#;
        let result: TestStringVec = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, Some(vec![]));
    }

    #[test]
    fn test_deserialize_string_vec_invalid_element() {
        let json = r#"{ "value": [1, 2, 3] }"#;
        let result: Result<TestStringVec, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // PackageWithVersion tests

    #[test]
    fn test_package_with_version_new() {
        let pkg = PackageWithVersion::new("serde".to_string());
        assert_eq!(pkg.package, "serde");
        assert_eq!(pkg.version, None);
        assert_eq!(pkg.to_spec(), "serde");
    }

    #[test]
    fn test_package_with_version_with_version() {
        let pkg = PackageWithVersion::with_version("serde".to_string(), "1.0.0".to_string());
        assert_eq!(pkg.package, "serde");
        assert_eq!(pkg.version, Some("1.0.0".to_string()));
        assert_eq!(pkg.to_spec(), "serde@1.0.0");
    }

    #[test]
    fn test_package_with_version_deserialize_package_only() {
        let json = r#"{"package":"serde"}"#;
        let result: PackageWithVersion = serde_json::from_str(json).unwrap();
        assert_eq!(result.package, "serde");
        assert_eq!(result.version, None);
    }

    #[test]
    fn test_package_with_version_deserialize_package_with_version() {
        let json = r#"{"package":"serde","version":"1.0.0"}"#;
        let result: PackageWithVersion = serde_json::from_str(json).unwrap();
        assert_eq!(result.package, "serde");
        assert_eq!(result.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_package_with_version_deserialize_null_version() {
        let json = r#"{"package":"serde","version":null}"#;
        let result: PackageWithVersion = serde_json::from_str(json).unwrap();
        assert_eq!(result.package, "serde");
        assert_eq!(result.version, None);
    }

    #[test]
    fn test_package_with_version_deserialize_version_null_string() {
        let json = r#"{"package":"serde","version":"null"}"#;
        let result: PackageWithVersion = serde_json::from_str(json).unwrap();
        assert_eq!(result.package, "serde");
        assert_eq!(result.version, None); // "null" string is treated as None by deserialize_string
    }

    #[test]
    fn test_package_with_version_to_spec() {
        let pkg1 = PackageWithVersion::new("serde".to_string());
        assert_eq!(pkg1.to_spec(), "serde");

        let pkg2 = PackageWithVersion::with_version("tokio".to_string(), "1.0.0".to_string());
        assert_eq!(pkg2.to_spec(), "tokio@1.0.0");

        let pkg3 = PackageWithVersion::with_version("clap".to_string(), "4.0.0-beta.1".to_string());
        assert_eq!(pkg3.to_spec(), "clap@4.0.0-beta.1");
    }

    #[test]
    fn test_tool_json_schema_removes_null_type_first() {
        #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct Example {
            #[serde(default)]
            value: Option<String>,
        }

        let schema = Example::json_schema();
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

        let schema = Example::json_schema();
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

        let schema = Example::json_schema();
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

        let schema = Example::json_schema();
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

        let schema = Example::json_schema();
        let props = schema.get("properties").unwrap();
        let value_schema = props.get("value").unwrap();
        if let serde_json::Value::Object(obj) = value_schema {
            let ty = obj.get("type").unwrap();
            assert_eq!(ty, "boolean");
        } else {
            panic!("Expected value property to be an object");
        }
    }

    #[test]
    fn test_locking_mode_cli_flags() {
        // Test default (locked)
        assert_eq!(locking_mode_to_cli_flags(None).unwrap(), vec!["--locked"]);
        
        // Test explicit modes
        assert_eq!(locking_mode_to_cli_flags(Some("locked")).unwrap(), vec!["--locked"]);
        assert_eq!(locking_mode_to_cli_flags(Some("unlocked")).unwrap(), Vec::<&str>::new());
        assert_eq!(locking_mode_to_cli_flags(Some("offline")).unwrap(), vec!["--offline"]);
        assert_eq!(locking_mode_to_cli_flags(Some("frozen")).unwrap(), vec!["--frozen"]);
        
        // Test unknown values return error
        assert!(locking_mode_to_cli_flags(Some("invalid")).is_err());
        let error = locking_mode_to_cli_flags(Some("invalid")).unwrap_err();
        assert!(error.to_string().contains("Unknown locking mode: invalid"));
    }
}
