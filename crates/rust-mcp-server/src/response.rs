use rmcp::model::ContentBlock;

use crate::command::{AgentRecommendation, Output};

pub(crate) struct Response {
    outputs: Vec<Output>,
    additional_content: Vec<ContentBlock>,
    recommendations: Vec<AgentRecommendation>,
}

impl Response {
    pub(crate) fn from_outputs(outputs: Vec<Output>) -> Self {
        debug_assert!(!outputs.is_empty());
        Self {
            outputs,
            additional_content: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub(crate) fn add_content(&mut self, content: ContentBlock) {
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
        let mut result = rmcp::model::CallToolResult::default();
        result.is_error = Some(false);
        for output in self.outputs {
            let output: rmcp::model::CallToolResult = output.into();
            result.content.extend(output.content);
            if output.is_error == Some(true) {
                result.is_error = Some(true);
            }
        }
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
        Self::from_outputs(vec![val])
    }
}

#[cfg(test)]
mod tests {
    use rmcp::model::{Annotations, ContentBlock, TextContent};

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
    fn multiple_outputs_remain_distinct() {
        let output = |command: &str| Output {
            tool_name: "test_tool".into(),
            stdout: None,
            stderr: None,
            cmd_line: CommandLine(command.into()),
            exit_status: ExitStatus(std::process::ExitStatus::default()),
        };

        let result =
            Response::from_outputs(vec![output("first"), output("second")]).into_rmcp_result(false);

        assert_eq!(result.is_error, Some(false));
        assert_eq!(result.content.len(), 4);
        assert_eq!(
            result.content[0].as_text().unwrap().text,
            "Executed command: `first`"
        );
        assert_eq!(
            result.content[2].as_text().unwrap().text,
            "Executed command: `second`"
        );
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
        response.add_content(ContentBlock::Text(
            TextContent::new("additional content").with_annotations(Annotations::default()),
        ));
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
