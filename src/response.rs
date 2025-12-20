use rmcp::model::Annotated;

use crate::command::{AgentRecommendation, Output};

pub(crate) struct Response {
    output: Output,
    additional_content: Vec<Annotated<rmcp::model::RawContent>>,
    recommendations: Vec<AgentRecommendation>,
}

impl Response {
    pub(crate) fn add_content(&mut self, content: Annotated<rmcp::model::RawContent>) {
        self.additional_content.push(content);
    }

    pub(crate) fn add_recommendation(&mut self, recommendation: impl Into<String>) {
        self.recommendations
            .push(AgentRecommendation(recommendation.into()));
    }

    pub(crate) fn into_rmcp_result(
        self,
        ignore_recommendations: bool,
    ) -> rmcp::model::CallToolResult {
        let mut result: rmcp::model::CallToolResult = self.output.into();
        result.content.extend(self.additional_content);
        if !ignore_recommendations {
            result
                .content
                .extend(self.recommendations.into_iter().map(Into::into));
        }
        result
    }
}

impl Into<Response> for Output {
    fn into(self) -> Response {
        Response {
            output: self,
            additional_content: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}
