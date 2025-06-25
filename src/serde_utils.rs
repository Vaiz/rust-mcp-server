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
}
