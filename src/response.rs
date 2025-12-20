use crate::command::{AgentRecommendation, Output};

pub(crate) struct Response {
    output: Output,
    recommendations: Vec<AgentRecommendation>,
}

impl Response {
    pub(crate) fn add_recommendation(&mut self, recommendation: impl Into<String>) {
        self.recommendations
            .push(AgentRecommendation(recommendation.into()));
    }

    pub(crate) fn into_rmcp_result(
        self,
        ignore_recommendations: bool,
    ) -> rmcp::model::CallToolResult {
        let mut result: rmcp::model::CallToolResult = self.output.into();
        if !ignore_recommendations {
            for recommendation in self.recommendations {
                result.content.push(recommendation.into());
            }
        }
        result
    }
}

impl Into<Response> for Output {
    fn into(self) -> Response {
        Response {
            output: self,
            recommendations: Vec::new(),
        }
    }
}
