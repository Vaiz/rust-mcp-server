use rmcp::{
    ErrorData,
    model::{AnnotateAble, Annotated, Annotations, CallToolResult, RawContent, Role},
};

use crate::tools::apply_workspace_root;

#[derive(Debug, Clone)]
pub(crate) struct CommandLine(pub String);

impl From<CommandLine> for Annotated<RawContent> {
    fn from(val: CommandLine) -> Self {
        RawContent::text(format!("Executed command: `{}`", val.0)).annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(0.5),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Stdout(pub String);

impl From<Stdout> for Annotated<RawContent> {
    fn from(val: Stdout) -> Self {
        RawContent::text(val.0).annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(0.2),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Stderr(pub String);

impl From<Stderr> for Annotated<RawContent> {
    fn from(val: Stderr) -> Self {
        RawContent::text(val.0).annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(1.),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExitStatus(std::process::ExitStatus);

impl ExitStatus {
    fn as_content(&self, tool_name: &str) -> Annotated<RawContent> {
        let status_str = if self.0.success() {
            format!("✅ {tool_name}: Success")
        } else {
            format!("❌ {tool_name}: Failure")
        };

        RawContent::text(status_str).annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(1.),
        })
    }
}

pub(crate) struct Recommendation(pub String);

impl From<Recommendation> for Annotated<RawContent> {
    fn from(val: Recommendation) -> Self {
        RawContent::text(val.0).annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(0.7),
        })
    }
}

pub(crate) struct Output {
    pub(crate) tool_name: String,
    pub(crate) cmd_line: CommandLine,
    pub(crate) stdout: Option<Stdout>,
    pub(crate) stderr: Option<Stderr>,
    pub(crate) exit_status: ExitStatus,
}

impl Output {
    fn new(tool_name: String, cmd_line: String, output: std::process::Output) -> Self {
        let cmd_line = CommandLine(cmd_line);

        let stdout = if !output.stdout.is_empty() {
            Some(Stdout(
                String::from_utf8_lossy(output.stdout.trim_ascii()).to_string(),
            ))
        } else {
            None
        };

        let stderr = if !output.stderr.is_empty() {
            Some(Stderr(
                String::from_utf8_lossy(output.stderr.trim_ascii()).to_string(),
            ))
        } else {
            None
        };

        let exit_status = ExitStatus(output.status);

        Output {
            tool_name,
            cmd_line,
            stdout,
            stderr,
            exit_status,
        }
    }

    pub(crate) fn success(&self) -> bool {
        self.exit_status.0.success()
    }
}

impl From<Output> for CallToolResult {
    fn from(val: Output) -> Self {
        let mut content: Vec<Annotated<RawContent>> = Vec::new();

        content.push(val.cmd_line.into());

        if let Some(stdout) = val.stdout {
            content.push(stdout.into());
        }
        if let Some(stderr) = val.stderr {
            content.push(stderr.into());
        }

        content.push(val.exit_status.as_content(&val.tool_name));

        CallToolResult {
            content,
            is_error: Some(!val.exit_status.0.success()),
            meta: None,
            structured_content: None,
        }
    }
}

pub(crate) fn execute_command(
    mut cmd: std::process::Command,
    tool_name: &str,
) -> Result<Output, ErrorData> {
    apply_workspace_root(&mut cmd);

    let cmd_line = format!(
        "{} {}",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    tracing::info!("Executing command for {tool_name}: {cmd_line}");
    match cmd.output() {
        Ok(output) => {
            let output = Output::new(tool_name.to_owned(), cmd_line, output);
            if output.success() {
                tracing::info!(
                    "Command executed successfully for {tool_name}\nstdout=\n{}\n\nstderr=\n{}",
                    output.stdout.as_ref().map(|s| s.0.as_str()).unwrap_or(""),
                    output.stderr.as_ref().map(|s| s.0.as_str()).unwrap_or(""),
                );
            } else {
                tracing::warn!(
                    "Command execution failed for {tool_name} (status: {:?}): stdout='\n{}\n', stderr='\n{}\n'",
                    output.exit_status.0.code(),
                    output.stdout.as_ref().map(|s| s.0.as_str()).unwrap_or(""),
                    output.stderr.as_ref().map(|s| s.0.as_str()).unwrap_or(""),
                );
            }
            Ok(output)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::error!("Command not found: {e}");
            let program = cmd.get_program().to_string_lossy();
            Err(ErrorData::internal_error(
                format!(
                    "The command `{program}` was not found, please ensure it is installed and accessible. You can try running the following command yourself to verify: `{cmd_line}`",
                ),
                None,
            ))
        }
        Err(e) => {
            tracing::error!("Failed to execute command: {e}");
            Err(ErrorData::internal_error(e.to_string(), None))
        }
    }
}
