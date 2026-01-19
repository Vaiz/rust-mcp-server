use std::process::Command;

use crate::{
    Tool, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags},
};
use rmcp::ErrorData;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct CargoTreeRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package to be used as the root of the tree
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Display the tree for all packages in the workspace
    #[serde(default)]
    workspace: Option<bool>,

    /// Exclude specific workspace members
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    exclude: Option<Vec<String>>,

    /// The kinds of dependencies to display (features, normal, build, dev, all, no-normal, no-build, no-dev, no-proc-macro)
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    edges: Option<Vec<String>>,

    /// Invert the tree direction and focus on the given package
    #[serde(default, deserialize_with = "deserialize_string")]
    invert: Option<String>,

    /// Prune the given package from the display of the dependency tree
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    prune: Option<Vec<String>>,

    /// Maximum display depth of the dependency tree
    #[serde(default)]
    depth: Option<u32>,

    /// Change the prefix (indentation) of how each entry is displayed.
    /// Possible values: depth, indent, none
    #[serde(default, deserialize_with = "deserialize_string")]
    prefix: Option<String>,

    /// Do not de-duplicate (repeats all shared dependencies)
    #[serde(default)]
    no_dedupe: Option<bool>,

    /// Show only dependencies which come in multiple versions (implies -i)
    #[serde(default)]
    duplicates: Option<bool>,

    /// Format string used for printing dependencies (default: {p})
    #[serde(default, deserialize_with = "deserialize_string")]
    format: Option<String>,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    features: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: Option<bool>,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: Option<bool>,

    /// Filter dependencies matching the given target-triple (default host platform).
    /// Pass `all` to include all targets.
    #[serde(default, deserialize_with = "deserialize_string")]
    target: Option<String>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked": Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    locking_mode: Option<String>,
}

impl CargoTreeRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");

        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }

        cmd.arg("tree");
        cmd.arg("--charset").arg("ascii"); // for better compatibility with LLMs

        // Package selection
        if let Some(packages) = &self.package {
            for pkg in packages {
                cmd.arg("--package").arg(pkg);
            }
        }

        if self.workspace.unwrap_or(false) {
            cmd.arg("--workspace");
        }

        if let Some(excludes) = &self.exclude {
            for exc in excludes {
                cmd.arg("--exclude").arg(exc);
            }
        }

        // Tree display options
        if let Some(edges) = &self.edges {
            cmd.arg("--edges").arg(edges.join(","));
        }

        if let Some(invert) = &self.invert {
            cmd.arg("--invert").arg(invert);
        }

        if let Some(prunes) = &self.prune {
            for p in prunes {
                cmd.arg("--prune").arg(p);
            }
        }

        if let Some(depth) = self.depth {
            cmd.arg("--depth").arg(depth.to_string());
        }

        if let Some(prefix) = &self.prefix {
            cmd.arg("--prefix").arg(prefix);
        }

        if self.no_dedupe.unwrap_or(false) {
            cmd.arg("--no-dedupe");
        }

        if self.duplicates.unwrap_or(false) {
            cmd.arg("--duplicates");
        }

        if let Some(format) = &self.format {
            cmd.arg("--format").arg(format);
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

        // Compilation options
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?;
        cmd.args(locking_flags);

        Ok(cmd)
    }
}

pub struct CargoTreeRmcpTool;

impl Tool for CargoTreeRmcpTool {
    const NAME: &'static str = "cargo-tree";
    const TITLE: &'static str = "cargo tree";
    const DESCRIPTION: &'static str = "Display a tree visualization of a dependency graph. Useful for understanding dependency relationships, finding duplicate dependencies, and debugging dependency resolution issues.";
    type RequestArgs = CargoTreeRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<crate::Response, ErrorData> {
        let cmd = request.build_cmd()?;
        let mut response: crate::Response = execute_command(cmd, Self::NAME)?.into();

        // Add helpful recommendations
        if request.depth.is_none() && request.duplicates.is_none() {
            response.add_recommendation(
                "Use depth parameter to limit output size for large dependency trees",
            );
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_minimal_request() {
        let input = json!({});
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize empty request");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(args, vec!["tree", "--charset", "ascii", "--locked"]);
    }

    #[test]
    fn test_with_package() {
        let input = json!({
            "package": ["my-crate"]
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with package");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec![
                "tree",
                "--charset",
                "ascii",
                "--package",
                "my-crate",
                "--locked"
            ]
        );
    }

    #[test]
    fn test_with_invert() {
        let input = json!({
            "invert": "tokio"
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with invert");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec![
                "tree",
                "--charset",
                "ascii",
                "--invert",
                "tokio",
                "--locked"
            ]
        );
    }

    #[test]
    fn test_with_depth() {
        let input = json!({
            "depth": 3
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with depth");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec!["tree", "--charset", "ascii", "--depth", "3", "--locked"]
        );
    }

    #[test]
    fn test_with_duplicates() {
        let input = json!({
            "duplicates": true
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with duplicates");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec!["tree", "--charset", "ascii", "--duplicates", "--locked"]
        );
    }

    #[test]
    fn test_with_edges() {
        let input = json!({
            "edges": ["normal", "build"]
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with edges");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec![
                "tree",
                "--charset",
                "ascii",
                "--edges",
                "normal,build",
                "--locked"
            ]
        );
    }

    #[test]
    fn test_with_toolchain() {
        let input = json!({
            "toolchain": "nightly"
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with toolchain");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec!["+nightly", "tree", "--charset", "ascii", "--locked"]
        );
    }

    #[test]
    fn test_with_features() {
        let input = json!({
            "features": ["serde", "tokio"],
            "all_features": false,
            "no_default_features": true
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with features");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec![
                "tree",
                "--charset",
                "ascii",
                "--features",
                "serde,tokio",
                "--no-default-features",
                "--locked"
            ]
        );
    }

    #[test]
    fn test_with_format() {
        let input = json!({
            "format": "{p} {l}"
        });
        let request: CargoTreeRequest =
            serde_json::from_value(input).expect("Should deserialize request with format");
        let cmd = request.build_cmd().expect("Should build command");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();

        assert_eq!(
            args,
            vec![
                "tree",
                "--charset",
                "ascii",
                "--format",
                "{p} {l}",
                "--locked"
            ]
        );
    }
}
