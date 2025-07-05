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
        let schema = schema_for!(Self).to_value();
        if let serde_json::Value::Object(mut map) = schema {
            map.remove("$schema");
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
}
