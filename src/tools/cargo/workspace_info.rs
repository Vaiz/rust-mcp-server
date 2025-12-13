use std::collections::HashMap;
use std::process::Command;

use crate::{Tool, serde_utils::deserialize_string};
use rmcp::{
    ErrorData,
    model::{AnnotateAble, Annotations, RawContent, Role},
};
use serde::Deserialize;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
#[schemars(title = "CargoWorkspaceInfoRequest")]
pub struct CargoWorkspaceInfoRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Include direct dependencies (name and version) for each package
    #[serde(default)]
    include_dependencies: Option<bool>,
}

impl CargoWorkspaceInfoRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("metadata");
        cmd.arg("--format-version").arg("1");
        cmd.arg("--no-deps");

        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        Ok(cmd)
    }
}

// Structures for parsing cargo metadata output
#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    #[serde(default)]
    description: Option<String>,
    manifest_path: String,
    targets: Vec<Target>,
    #[serde(default)]
    features: HashMap<String, Vec<String>>,
    #[serde(default)]
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Target {
    kind: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    req: String,
}

// Simplified output structures
#[derive(Debug, ::serde::Serialize)]
struct WorkspaceInfo {
    packages: Vec<PackageInfo>,
}

#[derive(Debug, ::serde::Serialize)]
struct PackageInfo {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    manifest_path: String,
    target_types: Vec<String>,
    features: HashMap<String, Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<DependencyInfo>>,
}

#[derive(Debug, ::serde::Serialize)]
struct DependencyInfo {
    name: String,
    version: String,
}

pub struct CargoWorkspaceInfoRmcpTool;

impl Tool for CargoWorkspaceInfoRmcpTool {
    const NAME: &'static str = "workspace-info";
    const TITLE: &'static str = "workspace info";
    const DESCRIPTION: &'static str = "Get information about crates in the current workspace, including package names, target \
         types, manifest paths, descriptions, features, and optionally dependencies.";
    type RequestArgs = CargoWorkspaceInfoRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let mut cmd = request.build_cmd()?;

        // Execute command and get output
        let output = cmd.output().map_err(|e| {
            ErrorData::internal_error(format!("Failed to execute cargo metadata: {}", e), None)
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ErrorData::internal_error(
                format!("cargo metadata failed: {}", stderr),
                None,
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let metadata: CargoMetadata = serde_json::from_str(&stdout).map_err(|e| {
            ErrorData::internal_error(format!("Failed to parse cargo metadata JSON: {}", e), None)
        })?;

        let include_deps = request.include_dependencies.unwrap_or(false);

        let packages: Vec<PackageInfo> = metadata
            .packages
            .into_iter()
            .map(|pkg| {
                // Extract unique target types (lib, bin, etc.)
                let mut target_types = Vec::new();
                let mut seen = std::collections::HashSet::new();

                for target in &pkg.targets {
                    for kind in &target.kind {
                        // Only include lib and bin, skip custom-build
                        if (kind == "lib" || kind == "bin") && !seen.contains(kind) {
                            target_types.push(kind.clone());
                            seen.insert(kind.clone());
                        }
                    }
                }
                target_types.sort();

                let dependencies = if include_deps {
                    Some(
                        pkg.dependencies
                            .into_iter()
                            .map(|dep| DependencyInfo {
                                name: dep.name,
                                version: dep.req,
                            })
                            .collect(),
                    )
                } else {
                    None
                };

                PackageInfo {
                    name: pkg.name,
                    description: pkg.description,
                    manifest_path: pkg.manifest_path,
                    target_types,
                    features: pkg.features,
                    dependencies,
                }
            })
            .collect();

        let workspace_info = WorkspaceInfo { packages };
        Ok(rmcp::model::CallToolResult::success(vec![
            RawContent::json(workspace_info)?.annotate(Annotations {
                audience: Some(vec![Role::User, Role::Assistant]),
                last_modified: None,
                priority: Some(1.),
            }),
        ]))
    }
}
