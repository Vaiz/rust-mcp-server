use std::collections::HashMap;
use std::process::Command;

use crate::{Tool, command::execute_command, serde_utils::deserialize_string};
use rmcp::{
    ErrorData,
    model::{AnnotateAble, Annotations, CallToolResult, RawContent, Role},
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

pub struct CargoWorkspaceInfoRmcpTool;

impl Tool for CargoWorkspaceInfoRmcpTool {
    const NAME: &'static str = "workspace-info";
    const TITLE: &'static str = "workspace info";
    const DESCRIPTION: &'static str = "Get information about crates in the current workspace, including package names, target \
         types, manifest paths, descriptions, features, and optionally dependencies. This is a cut down version of cargo metadata with the goal of saving tokens.";
    type RequestArgs = CargoWorkspaceInfoRequest;

    fn call_rmcp_tool(
        &self,
        request: Self::RequestArgs,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let cmd = request.build_cmd()?;
        let mut output = execute_command(cmd, Self::NAME)?;

        if !output.success() {
            return Ok(output.into());
        }

        let Some(stdout) = output.stdout.take() else {
            return Err(ErrorData::internal_error(
                "cargo metadata command produced no output".to_owned(),
                None,
            ));
        };

        let metadata: CargoMetadata = serde_json::from_str(&stdout.0).map_err(|e| {
            ErrorData::internal_error(format!("failed to parse cargo metadata JSON: {e}"), None)
        })?;

        let include_deps = request.include_dependencies.unwrap_or(false);
        let mut packages: Vec<PackageInfo> = vec![];

        for package in metadata.packages {
            let mut target_types = Vec::new();
            let mut seen = std::collections::HashSet::new();
            for target in &package.targets {
                for kind in &target.kind {
                    if !seen.contains(kind) {
                        target_types.push(kind.clone());
                        seen.insert(kind);
                    }
                }
            }
            target_types.sort();

            let dependencies = include_deps.then(|| {
                package
                    .dependencies
                    .into_iter()
                    .map(DependencyInfo::from)
                    .collect()
            });

            packages.push(PackageInfo {
                name: package.name,
                description: package.description,
                manifest_path: package.manifest_path,
                target_types,
                features: package.features,
                dependencies,
            });
        }

        let mut call_tool_result: CallToolResult = output.into();
        let workspace_info = WorkspaceInfo { packages };
        let workspace_info = RawContent::json(workspace_info)?.annotate(Annotations {
            audience: Some(vec![Role::User, Role::Assistant]),
            last_modified: None,
            priority: Some(1.),
        });

        call_tool_result.content.push(workspace_info);
        Ok(call_tool_result)
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

impl From<Dependency> for DependencyInfo {
    fn from(dep: Dependency) -> Self {
        DependencyInfo {
            name: dep.name,
            version: dep.req,
        }
    }
}
