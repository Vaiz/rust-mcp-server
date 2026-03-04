use std::process::Command;

use rmcp::ErrorData;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoInstaUpdateSnapshotsRequest {
    /// Path to `Cargo.toml`
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Explicit path to the workspace root
    #[serde(default, deserialize_with = "deserialize_string")]
    workspace_root: Option<String>,

    /// Handle unreferenced snapshots after a successful test run.
    ///
    /// Valid options: auto, reject, delete, warn, ignore
    #[serde(default, deserialize_with = "deserialize_string")]
    unreferenced: Option<String>,

    /// Picks the test runner.
    ///
    /// Valid options: auto (default), cargo-test, nextest
    #[serde(default, deserialize_with = "deserialize_string")]
    test_runner: Option<String>,

    // ── Package / target selection ────────────────────────────────────────────
    /// Package(s) to run tests for
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Exclude packages from the test
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// Test only this package's library unit tests
    #[serde(default)]
    lib: Option<bool>,

    /// Test all targets that have `test = true` set
    #[serde(default)]
    tests: Option<bool>,

    /// Test only the specified test target
    #[serde(default, deserialize_with = "deserialize_string")]
    test: Option<String>,

    /// Test all targets (does not include doctests)
    #[serde(default)]
    all_targets: Option<bool>,

    // ── Feature selection ─────────────────────────────────────────────────────
    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Number of parallel jobs, defaults to # of CPUs
    #[serde(default)]
    jobs: Option<u32>,
}

impl CargoInstaUpdateSnapshotsRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        cmd.arg("insta").arg("test").arg("--accept");

        // Workspace / manifest selection
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        } else {
            // by default, run in workspace mode to update snapshots for all members
            cmd.arg("--workspace");
        }

        if let Some(workspace_root) = &self.workspace_root {
            cmd.arg("--workspace-root").arg(workspace_root);
        }

        // Insta-specific flags
        if let Some(unreferenced) = &self.unreferenced {
            cmd.arg("--unreferenced").arg(unreferenced);
        }

        if let Some(test_runner) = &self.test_runner {
            cmd.arg("--test-runner").arg(test_runner);
        }

        // Package selection
        if let Some(packages) = &self.package {
            for p in packages {
                cmd.arg("--package").arg(p);
            }
        }

        if let Some(excludes) = &self.exclude {
            for e in excludes {
                cmd.arg("--exclude").arg(e);
            }
        }

        // Target selection
        if self.lib.unwrap_or(false) {
            cmd.arg("--lib");
        }

        if self.tests.unwrap_or(false) {
            cmd.arg("--tests");
        }

        if let Some(test) = &self.test {
            cmd.arg("--test").arg(test);
        }

        if self.all_targets.unwrap_or(false) {
            cmd.arg("--all-targets");
        }

        // Feature selection
        if let Some(features) = &self.features
            && !features.is_empty()
        {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.all_features.unwrap_or(false) {
            cmd.arg("--all-features");
        }

        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }

        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
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

    #[test]
    fn test_update_snapshots_all_features() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "all_features": true
        }))
        .expect("Should deserialize request");
        let args = cmd_args(request);
        assert_debug_snapshot!("cargo_insta_update_snapshots_all_features_args", args);
    }

    #[test]
    fn test_update_snapshots_features_and_targets() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "features": ["serde", "async"],
            "no_default_features": true,
            "lib": true,
            "jobs": 4
        }))
        .expect("Should deserialize request");
        let args = cmd_args(request);
        assert_debug_snapshot!(
            "cargo_insta_update_snapshots_features_and_targets_args",
            args
        );
    }

    #[test]
    fn test_update_snapshots_nextest_runner() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "test_runner": "nextest",
            "unreferenced": "delete"
        }))
        .expect("Should deserialize request");
        let args = cmd_args(request);
        assert_debug_snapshot!("cargo_insta_update_snapshots_nextest_runner_args", args);
    }
}
