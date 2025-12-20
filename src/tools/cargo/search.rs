use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, output_verbosity_to_cli_flags},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoSearchRequest {
    /// The query to search for. Generally, this is a substring of the package name or description.
    pub query: String,
    /// Limit the number of results (default: 10, max: 100)
    pub limit: Option<u32>,
    /// Registry to search packages in
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,
    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}
impl CargoSearchRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("search");
        cmd.arg(&self.query);
        if let Some(limit) = self.limit {
            cmd.arg("--limit").arg(limit.to_string());
        }
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);
        Ok(cmd)
    }
}

pub struct CargoSearchRmcpTool;

impl Tool for CargoSearchRmcpTool {
    const NAME: &'static str = "cargo-search";
    const TITLE: &'static str = "cargo search";
    const DESCRIPTION: &'static str = "Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search <code>QUERY</code>'.";
    type RequestArgs = CargoSearchRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        execute_command(cmd, Self::NAME).map(Into::into)
    }
}
