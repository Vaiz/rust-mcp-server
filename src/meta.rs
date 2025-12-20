pub(crate) struct Meta {
    inner: serde_json::Map<String, serde_json::Value>,
}

impl Meta {
    pub(crate) fn new() -> Self {
        Meta {
            inner: serde_json::Map::new(),
        }
    }
    pub(crate) fn with_description(mut self, description: impl Into<String>) -> Self {
        self.inner.insert(
            "description".to_owned(),
            serde_json::Value::String(description.into()),
        );
        self
    }

    pub(crate) fn with_i32(mut self, key: impl Into<String>, value: i32) -> Self {
        self.inner
            .insert(key.into(), serde_json::Value::Number(value.into()));
        self
    }
}

impl From<Meta> for rmcp::model::Meta {
    fn from(val: Meta) -> Self {
        rmcp::model::Meta(val.inner)
    }
}
