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

impl From<Output> for Response {
    fn from(val: Output) -> Self {
        Response {
            output: val,
            additional_content: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rmcp::model::{AnnotateAble, Annotations, RawContent};

    use crate::command::{CommandLine, ExitStatus, Stdout};

    use super::*;

    #[test]
    fn only_output() {
        let output = Output {
            tool_name: "test_tool".into(),
            stdout: Some(Stdout("This is a test output".into())),
            stderr: None,
            cmd_line: CommandLine("test_command --option".into()),
            exit_status: ExitStatus(std::process::ExitStatus::default()),
        };
        let response: Response = output.into();
        let rmcp_result = response.into_rmcp_result(false);

        let [cmd_line, stdout, exit_status] = &rmcp_result.content[..] else {
            panic!("expected 3 content items: {rmcp_result:?}");
        };

        assert_eq!(
            cmd_line.as_text().unwrap().text,
            "Executed command: `test_command --option`"
        );
        assert_eq!(stdout.as_text().unwrap().text, "This is a test output");
        assert_eq!(exit_status.as_text().unwrap().text, "✅ test_tool: Success");
    }

    #[test]
    fn with_additional_content_and_recommendations() {
        let output = Output {
            tool_name: "test_tool".into(),
            stdout: Some(Stdout("This is a test output".into())),
            stderr: None,
            cmd_line: CommandLine("test_command --option".into()),
            exit_status: ExitStatus(std::process::ExitStatus::default()),
        };
        let mut response: Response = output.into();
        response.add_content(
            RawContent::text("additional content").annotate(Annotations {
                audience: None,
                priority: None,
                last_modified: None,
            }),
        );
        response.add_recommendation("Consider checking the logs.");

        let rmcp_result = response.into_rmcp_result(false);

        let [
            cmd_line,
            stdout,
            exit_status,
            additional_content,
            recommendation,
        ] = &rmcp_result.content[..]
        else {
            panic!("expected 5 content items: {rmcp_result:?}");
        };

        assert_eq!(
            cmd_line.as_text().unwrap().text,
            "Executed command: `test_command --option`"
        );
        assert_eq!(stdout.as_text().unwrap().text, "This is a test output");
        assert_eq!(exit_status.as_text().unwrap().text, "✅ test_tool: Success");
        assert_eq!(
            additional_content.as_text().unwrap().text,
            "additional content"
        );
        assert_eq!(
            recommendation.as_text().unwrap().text,
            "RECOMMENDATION: Consider checking the logs."
        );
    }

    #[test]
    fn ignore_recommendations() {
        let output = Output {
            tool_name: "test_tool".into(),
            stdout: Some(Stdout("This is a test output".into())),
            stderr: None,
            cmd_line: CommandLine("test_command --option".into()),
            exit_status: ExitStatus(std::process::ExitStatus::default()),
        };
        let mut response: Response = output.into();
        response.add_recommendation("Consider checking the logs.");

        let rmcp_result = response.into_rmcp_result(true);

        let [cmd_line, stdout, exit_status] = &rmcp_result.content[..] else {
            panic!("expected 3 content items: {rmcp_result:?}");
        };

        assert_eq!(
            cmd_line.as_text().unwrap().text,
            "Executed command: `test_command --option`"
        );
        assert_eq!(stdout.as_text().unwrap().text, "This is a test output");
        assert_eq!(exit_status.as_text().unwrap().text, "✅ test_tool: Success");
    }
}
