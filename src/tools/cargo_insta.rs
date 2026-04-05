use std::process::Command;

use rmcp::ErrorData;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec},
};

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoInstaUpdateSnapshotsRequest {
    /// Forcibly updates snapshot files, even if assertions pass (`INSTA_UPDATE=force`).
    /// When `false` (default), uses `INSTA_UPDATE=always` to update failing snapshots.
    #[serde(default)]
    force: Option<bool>,

    /// Path to `Cargo.toml`
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

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
        cmd.arg("test");

        // Set INSTA_UPDATE env var (defaults to "always" to bypass review)
        let insta_update = if self.force.unwrap_or(false) {
            "force"
        } else {
            "always"
        };
        cmd.env("INSTA_UPDATE", insta_update);

        // Workspace / manifest selection
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        } else {
            // by default, run in workspace mode to update snapshots for all members
            cmd.arg("--workspace");
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
    const DESCRIPTION: &'static str = "Runs `cargo test` with the `INSTA_UPDATE` environment variable to update insta snapshot files.";
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

    fn cmd_args(request: &CargoInstaUpdateSnapshotsRequest) -> Vec<String> {
        request
            .build_cmd()
            .expect("Should build command")
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect()
    }

    fn cmd_env_insta_update(request: &CargoInstaUpdateSnapshotsRequest) -> String {
        request
            .build_cmd()
            .expect("Should build command")
            .get_envs()
            .find(|(k, _)| k == &"INSTA_UPDATE")
            .and_then(|(_, v)| v)
            .map(|v| v.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    #[test]
    fn test_update_snapshots_default_args() {
        let request: CargoInstaUpdateSnapshotsRequest =
            serde_json::from_value(json!({})).expect("Should deserialize empty request");
        let args = cmd_args(&request);
        assert_debug_snapshot!("cargo_insta_update_snapshots_default_args", args);
        assert_eq!(cmd_env_insta_update(&request), "always");
    }

    #[test]
    fn test_update_snapshots_with_manifest_path() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "manifest_path": "Cargo.toml"
        }))
        .expect("Should deserialize request");

        let args = cmd_args(&request);
        assert_debug_snapshot!("cargo_insta_update_snapshots_manifest_path_args", args);
    }

    #[test]
    fn test_update_snapshots_all_features() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "all_features": true
        }))
        .expect("Should deserialize request");
        let args = cmd_args(&request);
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
        let args = cmd_args(&request);
        assert_debug_snapshot!(
            "cargo_insta_update_snapshots_features_and_targets_args",
            args
        );
    }

    #[test]
    fn test_update_snapshots_force_flag() {
        let request: CargoInstaUpdateSnapshotsRequest = serde_json::from_value(json!({
            "force": true
        }))
        .expect("Should deserialize request");
        assert_eq!(cmd_env_insta_update(&request), "force");
    }
}
