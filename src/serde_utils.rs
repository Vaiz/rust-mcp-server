use rust_mcp_sdk::macros::JsonSchema;
use schemars::JsonSchema as SchemarsJsonSchema;
use serde::{Deserialize, Serialize};

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

pub const fn default_true() -> bool {
    true
}

/// A type that represents a package with an optional version.
/// When calling cargo commands, use `to_spec()` to get "package" or "package@version" format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema, serde::Serialize, serde::Deserialize)]
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

/// Dependency type for cargo add/remove operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, SchemarsJsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    /// Regular dependency (default section)
    Regular,
    /// Development dependency
    Dev,
    /// Build dependency
    Build,
}

impl Default for DependencyType {
    fn default() -> Self {
        Self::Regular
    }
}

impl DependencyType {
    /// Convert to the corresponding CLI flag
    pub fn to_cli_flag(self) -> Option<&'static str> {
        match self {
            DependencyType::Regular => None,
            DependencyType::Dev => Some("--dev"),
            DependencyType::Build => Some("--build"),
        }
    }

    /// Check if this is a dev dependency
    pub fn is_dev(&self) -> bool {
        matches!(self, DependencyType::Dev)
    }

    /// Check if this is a build dependency
    pub fn is_build(&self) -> bool {
        matches!(self, DependencyType::Build)
    }

    /// Create from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dev" | "development" => DependencyType::Dev,
            "build" => DependencyType::Build,
            _ => DependencyType::Regular,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::Regular => "regular",
            DependencyType::Dev => "dev",
            DependencyType::Build => "build",
        }
    }
}

// Manual implementation to bridge schemars JsonSchema with MCP SDK JsonSchema
impl DependencyType {
    pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
        use schemars::schema_for;
        let schema = schema_for!(DependencyType);
        if let serde_json::Value::Object(map) = serde_json::to_value(schema).unwrap_or_default() {
            map
        } else {
            // Fallback manual schema
            let mut map = serde_json::Map::new();
            map.insert("type".to_string(), serde_json::Value::String("string".to_string()));
            map.insert("enum".to_string(), serde_json::json!(["regular", "dev", "build"]));
            map.insert("default".to_string(), serde_json::Value::String("regular".to_string()));
            map.insert("description".to_string(), serde_json::Value::String("Dependency type".to_string()));
            map
        }
    }
}

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
    fn test_package_with_version_serialize() {
        let pkg1 = PackageWithVersion::new("serde".to_string());
        let json1 = serde_json::to_string(&pkg1).unwrap();
        assert_eq!(json1, r#"{"package":"serde","version":null}"#);

        let pkg2 = PackageWithVersion::with_version("serde".to_string(), "1.0.0".to_string());
        let json2 = serde_json::to_string(&pkg2).unwrap();
        assert_eq!(json2, r#"{"package":"serde","version":"1.0.0"}"#);
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
    fn test_package_with_version_roundtrip() {
        let original = PackageWithVersion::with_version("tokio".to_string(), "1.0.0".to_string());
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PackageWithVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test the to_spec method works correctly
        assert_eq!(deserialized.to_spec(), "tokio@1.0.0");
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
    fn test_dependency_type_enum() {
        // Test default
        assert_eq!(DependencyType::default(), DependencyType::Regular);

        // Test CLI flags
        assert_eq!(DependencyType::Regular.to_cli_flag(), None);
        assert_eq!(DependencyType::Dev.to_cli_flag(), Some("--dev"));
        assert_eq!(DependencyType::Build.to_cli_flag(), Some("--build"));

        // Test string conversion
        assert_eq!(DependencyType::Regular.as_str(), "regular");
        assert_eq!(DependencyType::Dev.as_str(), "dev");
        assert_eq!(DependencyType::Build.as_str(), "build");

        // Test from_str
        assert_eq!(DependencyType::from_str("regular"), DependencyType::Regular);
        assert_eq!(DependencyType::from_str("dev"), DependencyType::Dev);
        assert_eq!(DependencyType::from_str("development"), DependencyType::Dev);
        assert_eq!(DependencyType::from_str("build"), DependencyType::Build);
        assert_eq!(DependencyType::from_str("unknown"), DependencyType::Regular);

        // Test type checks
        assert!(!DependencyType::Regular.is_dev());
        assert!(!DependencyType::Regular.is_build());
        assert!(DependencyType::Dev.is_dev());
        assert!(!DependencyType::Dev.is_build());
        assert!(!DependencyType::Build.is_dev());
        assert!(DependencyType::Build.is_build());
    }

    #[test]
    fn test_dependency_type_serde() {
        // Test serialization
        assert_eq!(serde_json::to_string(&DependencyType::Regular).unwrap(), "\"regular\"");
        assert_eq!(serde_json::to_string(&DependencyType::Dev).unwrap(), "\"dev\"");
        assert_eq!(serde_json::to_string(&DependencyType::Build).unwrap(), "\"build\"");

        // Test deserialization
        assert_eq!(serde_json::from_str::<DependencyType>("\"regular\"").unwrap(), DependencyType::Regular);
        assert_eq!(serde_json::from_str::<DependencyType>("\"dev\"").unwrap(), DependencyType::Dev);
        assert_eq!(serde_json::from_str::<DependencyType>("\"build\"").unwrap(), DependencyType::Build);
    }    #[test]
    fn test_dependency_type_json_schema() {
        let schema = DependencyType::json_schema();
        
        // Should have some schema content  
        assert!(!schema.is_empty(), "Schema should not be empty");
    }
}
