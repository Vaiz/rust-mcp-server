use std::collections::HashMap;

use rust_mcp_sdk::schema::{GetPromptResult, Prompt, PromptMessage, Role, TextContent};

pub struct PromptHandler {
    prompts: HashMap<String, (Prompt, GetPromptResult)>,
}

impl Default for PromptHandler {
    fn default() -> Self {
        let mut prompts = HashMap::new();

        let prompt = Prompt {
            arguments: Vec::new(),
            description: Some("Provides instruction on how to update Rust toolset".into()),
            name: "rustup-update-toolset".into(),
        };

        let prompt_message = PromptMessage {
            role: Role::Assistant,
            content: TextContent::new(include_str!("../prompts/update-toolset.md").into(), None)
                .into(),
        };

        let result = GetPromptResult {
            description: prompt.description.clone(),
            meta: None,
            messages: vec![prompt_message],
        };

        prompts.insert(prompt.name.clone(), (prompt, result));

        Self { prompts }
    }
}

impl PromptHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_prompts(&self) -> Vec<Prompt> {
        self.prompts
            .values()
            .map(|(prompt, _)| prompt.clone())
            .collect()
    }

    pub fn get_prompt_result(&self, name: &str) -> Option<&GetPromptResult> {
        self.prompts.get(name).map(|(_, result)| result)
    }
}
