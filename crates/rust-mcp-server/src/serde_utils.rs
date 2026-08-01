use rmcp::ErrorData;

use crate::globals;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize)]
#[serde(transparent)]
pub struct Registry {
    #[serde(deserialize_with = "deserialize_string")]
    value: Option<String>,
}

impl Registry {
    pub fn value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or_else(|| globals::get_default_registry())
    }
}

impl schemars::JsonSchema for Registry {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("string")
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("string")
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string", "default": null })
    }
}

/// Utility function for parsing Option<String> fields in serde,
/// returning None if the string is "null" (case-insensitive) or empty.
pub fn deserialize_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        Some("null") | Some("") => Ok(None),
        _ => Ok(opt),
    }
}

/// Utility function for parsing Option<Vec<String>> fields in serde,
/// returning None if the value is a string "null" (case-insensitive) or empty.
pub fn deserialize_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) if s.to_lowercase() == "null" || s.is_empty() => Ok(None),
        Value::String(s) => Ok(Some(vec![s])),
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

/// A cargo package spec: a crate name, optionally `name@version`.
///
/// Distinct from a plain string field because the `"null"` sentinel used by
/// [`deserialize_string`] is a real crate name, as is `true`. Silently dropping those is
/// worse than letting cargo reject a name it doesn't recognise.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageName(String);

impl PackageName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PackageName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl AsRef<std::ffi::OsStr> for PackageName {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl PartialEq<&str> for PackageName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for PackageName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PackageNameVisitor;

        impl serde::de::Visitor<'_> for PackageNameVisitor {
            type Value = PackageName;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a package name")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(PackageName(value.to_owned()))
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
                Ok(PackageName(value))
            }
        }

        deserializer.deserialize_string(PackageNameVisitor)
    }
}

impl schemars::JsonSchema for PackageName {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("string")
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("string")
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}

/// Parses an optional package name, treating only JSON `null` and an empty string as absent.
pub fn deserialize_package<'de, D>(deserializer: D) -> Result<Option<PackageName>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let package = Option::<PackageName>::deserialize(deserializer)?;
    Ok(package.filter(|p| !p.0.is_empty()))
}

/// Parses an optional list of package names, accepting either a single name or an array.
/// Unlike [`deserialize_string_vec`], only JSON `null` and an empty string mean absent.
pub fn deserialize_package_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<PackageName>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(PackageName),
        Many(Vec<PackageName>),
    }

    let value = Option::<OneOrMany>::deserialize(deserializer)?;
    Ok(match value {
        None => None,
        Some(OneOrMany::One(package)) if package.0.is_empty() => None,
        Some(OneOrMany::One(package)) => Some(vec![package]),
        Some(OneOrMany::Many(packages)) => Some(packages),
    })
}

/// Convert locking mode string to CLI flags for cargo commands.
/// Returns a vector of flags to add to the command.
///
/// Valid modes:
/// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
/// - "unlocked": Allow `Cargo.lock` to be updated  
/// - "offline": Run without accessing the network
/// - "frozen": Equivalent to specifying both --locked and --offline
pub fn locking_mode_to_cli_flags(
    mode: Option<&str>,
    preferred: &str,
) -> Result<Vec<&'static str>, ErrorData> {
    Ok(match mode.unwrap_or(preferred) {
        "locked" => vec!["--locked"],
        "unlocked" => vec![], // No flags needed
        "offline" => vec!["--offline"],
        "frozen" => vec!["--frozen"],
        unknown => {
            return Err(ErrorData::invalid_params(
                format!(
                    "Unknown locking mode: {unknown}. Valid options are: locked, unlocked, offline, frozen"
                ),
                None,
            ));
        }
    })
}

/// Convert output verbosity string to CLI flags for cargo commands.
/// Returns a vector of flags to add to the command.
///
/// Valid modes:
/// - "quiet" (default): Show only the essential command output
/// - "normal": Show standard output (no additional flags)
/// - "verbose": Show detailed output including build information
pub fn output_verbosity_to_cli_flags(mode: Option<&str>) -> Result<Vec<&'static str>, ErrorData> {
    Ok(match mode.unwrap_or("quiet") {
        "quiet" => vec!["--quiet"],
        "normal" => vec![], // No flags needed
        "verbose" => vec!["--verbose"],
        unknown => {
            return Err(ErrorData::invalid_params(
                format!(
                    "Unknown output verbosity: {unknown}. Valid options are: quiet, normal, verbose"
                ),
                None,
            ));
        }
    })
}

/// A type that represents a package with an optional version.
/// When calling cargo commands, use `to_spec()` to get "package" or "package@version" format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, schemars::JsonSchema)]
pub struct PackageWithVersion {
    /// The package name
    pub package: PackageName,
    /// Optional version specification
    #[serde(default, deserialize_with = "deserialize_string")]
    pub version: Option<String>,
}

impl PackageWithVersion {
    /// Create a new PackageWithVersion with just a package name
    #[cfg(test)]
    pub fn new(package: String) -> Self {
        Self {
            package: package.into(),
            version: None,
        }
    }

    /// Create a new PackageWithVersion with a package name and version
    #[cfg(test)]
    pub fn with_version(package: String, version: String) -> Self {
        Self {
            package: package.into(),
            version: Some(version),
        }
    }

    /// Get the formatted string representation (package or package@version)
    pub fn to_spec(&self) -> String {
        match &self.version {
            Some(version) => format!("{}@{}", self.package, version),
            None => self.package.as_str().to_owned(),
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
    fn test_deserialize_string_empty_string() {
        let json = r#"{ "value": "" }"#;
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
    fn test_deserialize_string_vec_empty_string() {
        let json = r#"{ "value": "" }"#;
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

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestPackage {
        #[serde(deserialize_with = "deserialize_package")]
        value: Option<PackageName>,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestPackageVec {
        #[serde(deserialize_with = "deserialize_package_vec")]
        value: Option<Vec<PackageName>>,
    }

    #[test]
    fn test_deserialize_package_keeps_odd_names() {
        for name in ["true", "false", "null", "NULL"] {
            let json = format!(r#"{{ "value": "{name}" }}"#);
            let result: TestPackage = serde_json::from_str(&json).unwrap();
            assert_eq!(result.value.unwrap(), name);
        }
    }

    #[test]
    fn test_deserialize_package_absent() {
        for json in [r#"{ "value": null }"#, r#"{ "value": "" }"#] {
            let result: TestPackage = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, None);
        }
    }

    #[test]
    fn test_deserialize_package_rejects_non_string() {
        let error = serde_json::from_str::<TestPackage>(r#"{ "value": true }"#)
            .expect_err("a boolean is not a package name");
        assert!(
            error.to_string().contains("expected a package name"),
            "{error}"
        );
    }

    #[test]
    fn test_deserialize_package_vec_single_or_array() {
        let result: TestPackageVec = serde_json::from_str(r#"{ "value": "tokio" }"#).unwrap();
        assert_eq!(result.value.unwrap(), ["tokio"]);

        let result: TestPackageVec = serde_json::from_str(r#"{ "value": ["null"] }"#).unwrap();
        assert_eq!(result.value.unwrap(), ["null"]);
    }

    #[test]
    fn test_deserialize_package_vec_absent() {
        for json in [r#"{ "value": null }"#, r#"{ "value": "" }"#] {
            let result: TestPackageVec = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, None);
        }
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
    fn test_locking_mode_cli_flags() {
        // Test default (locked)
        assert_eq!(
            locking_mode_to_cli_flags(None, "locked").unwrap(),
            vec!["--locked"]
        );

        // Test explicit modes
        assert_eq!(
            locking_mode_to_cli_flags(Some("locked"), "locked").unwrap(),
            vec!["--locked"]
        );
        assert_eq!(
            locking_mode_to_cli_flags(Some("unlocked"), "locked").unwrap(),
            Vec::<&str>::new()
        );
        assert_eq!(
            locking_mode_to_cli_flags(Some("offline"), "locked").unwrap(),
            vec!["--offline"]
        );
        assert_eq!(
            locking_mode_to_cli_flags(Some("frozen"), "locked").unwrap(),
            vec!["--frozen"]
        );

        // Test unknown values return error
        assert!(locking_mode_to_cli_flags(Some("invalid"), "locked").is_err());
        let error = locking_mode_to_cli_flags(Some("invalid"), "locked").unwrap_err();
        assert!(error.to_string().contains("Unknown locking mode: invalid"));

        // Test with unlocked as preferred
        assert_eq!(
            locking_mode_to_cli_flags(None, "unlocked").unwrap(),
            Vec::<&str>::new()
        );
        assert_eq!(
            locking_mode_to_cli_flags(Some("locked"), "unlocked").unwrap(),
            vec!["--locked"]
        );
    }

    #[test]
    fn test_output_verbosity_to_cli_flags() {
        // Test default (quiet)
        assert_eq!(
            output_verbosity_to_cli_flags(None).unwrap(),
            vec!["--quiet"]
        );

        // Test explicit modes
        assert_eq!(
            output_verbosity_to_cli_flags(Some("quiet")).unwrap(),
            vec!["--quiet"]
        );
        assert_eq!(
            output_verbosity_to_cli_flags(Some("normal")).unwrap(),
            Vec::<&str>::new()
        );
        assert_eq!(
            output_verbosity_to_cli_flags(Some("verbose")).unwrap(),
            vec!["--verbose"]
        );

        // Test unknown values return error
        assert!(output_verbosity_to_cli_flags(Some("invalid")).is_err());
        let error = output_verbosity_to_cli_flags(Some("invalid")).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Unknown output verbosity: invalid")
        );
    }

    #[test]
    fn test_output_verbosity_cli_flags() {
        // Test default (quiet)
        assert_eq!(
            output_verbosity_to_cli_flags(None).unwrap(),
            vec!["--quiet"]
        );

        // Test explicit quiet
        assert_eq!(
            output_verbosity_to_cli_flags(Some("quiet")).unwrap(),
            vec!["--quiet"]
        );

        // Test normal (no flags)
        assert_eq!(
            output_verbosity_to_cli_flags(Some("normal")).unwrap(),
            Vec::<&'static str>::new()
        );

        // Test verbose
        assert_eq!(
            output_verbosity_to_cli_flags(Some("verbose")).unwrap(),
            vec!["--verbose"]
        );

        // Test invalid option
        assert!(output_verbosity_to_cli_flags(Some("invalid")).is_err());
    }
}
