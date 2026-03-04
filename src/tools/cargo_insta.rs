use std::process::Command;

use rmcp::ErrorData;

use crate::{Tool, execute_command, serde_utils::deserialize_string};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoInstaUpdateSnapshotsRequest {
    /// Path to `Cargo.toml`
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Explicit path to the workspace root
    #[serde(default, deserialize_with = "deserialize_string")]
    workspace_root: Option<String>,
}

impl CargoInstaUpdateSnapshotsRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("insta").arg("test").arg("--accept");

        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        } else {
            // by default, run in workspace mode to update snapshots for all members
            cmd.arg("--workspace");
        }

        if let Some(workspace_root) = &self.workspace_root {
            cmd.arg("--workspace-root").arg(workspace_root);
        }

        Ok(cmd)
    }
}

pub struct CargoInstaUpdateSnapshotsRmcpTool;

impl Tool for CargoInstaUpdateSnapshotsRmcpTool {
    const NAME: &'static str = "cargo-insta-update-snapshots";
    const TITLE: &'static str = "Update insta snapshots";
    const DESCRIPTION: &'static str =
        "Generates and updates `cargo insta` snapshot files fixing tests with outdated snapshots";
    type RequestArgs = CargoInstaUpdateSnapshotsRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::CargoInstaUpdateSnapshotsRequest;
    use insta::assert_debug_snapshot;
    use serde_json::json;

    fn cmd_args(request: CargoInstaUpdateSnapshotsRequest) -> Vec<String> {
        request
            .build_cmd()
            .expect("Should build command")
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect()
    }

    #[test]
    fn test_update_snapshots_default_args() {
        let request: CargoInstaUpdateSnapshotsRequest =
            serde_json::from_value(json!({})).expect("Should deserialize empty request");
        let args = cmd_args(request);
        assert_debug_snapshot!("cargo_insta_update_snapshots_default_args", args);
    }

    #[test]
    fn test_update_snapshots_with_manifest_and_workspace_root() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "manifest_path": "Cargo.toml",
            "workspace_root": "."
        }))
        .expect("Should deserialize request");

        let args = cmd_args(request);
        assert_debug_snapshot!(
            "cargo_insta_update_snapshots_manifest_workspace_root_args",
            args
        );
    }
}
