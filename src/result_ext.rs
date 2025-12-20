use crate::command::AgentRecommendation;

pub trait ResultExt {
    fn add_recommendation(&mut self, recommendation: impl Into<String>);
}

impl ResultExt for rmcp::model::CallToolResult {
    fn add_recommendation(&mut self, recommendation: impl Into<String>) {
        self.content
            .push(AgentRecommendation(recommendation.into()).into());
    }
}
