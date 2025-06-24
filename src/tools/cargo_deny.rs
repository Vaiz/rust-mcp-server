use std::process::Command;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use crate::tools::execute_command;

#[mcp_tool(
    name = "cargo-deny-check",
    description = "Checks a project's crate graph for security advisories, license compliance, banned crates.",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoDenyCheckTool {
    /// The check(s) to perform. Options: advisories, ban, bans, license, licenses, sources, all
    which: Option<Vec<String>>,

    /// Path to the config to use. Defaults to <cwd>/deny.toml if not specified
    config: Option<String>,

    /// Path to graph output root directory for dotviz graph creation
    graph: Option<String>,

    /// Hides the inclusion graph when printing out info for a crate
    #[serde(default)]
    hide_inclusion_graph: bool,

    /// Disable fetching of the advisory database
    #[serde(default)]
    disable_fetch: bool,

    /// If set, excludes all dev-dependencies, not just ones for non-workspace crates
    #[serde(default)]
    exclude_dev: bool,

    /// Show stats for all the checks, regardless of the log-level
    #[serde(default)]
    show_stats: bool,

    /// Set lint warnings
    warn: Option<Vec<String>>,

    /// Set lint allowed
    allow: Option<Vec<String>>,

    /// Set lint denied
    deny: Option<Vec<String>>,

    /// Specifies the depth at which feature edges are added in inclusion graphs
    feature_depth: Option<u32>,

    /// The log level for messages (off, error, warn, info, debug, trace)
    log_level: Option<String>,

    /// Specify the format of cargo-deny's output (human, json)
    format: Option<String>,

    /// The path of a Cargo.toml to use as the context for the operation
    manifest_path: Option<String>,

    /// If passed, all workspace packages are used as roots for the crate graph
    #[serde(default)]
    workspace: bool,

    /// One or more crates to exclude from the crate graph that is used
    exclude: Option<Vec<String>>,

    /// One or more platforms to filter crates by
    target: Option<Vec<String>>,

    /// Activate all available features
    #[serde(default)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[serde(default)]
    no_default_features: bool,

    /// Space or comma separated list of features to activate
    features: Option<Vec<String>>,

    /// Run without accessing the network
    #[serde(default)]
    offline: bool,

    /// Assert that `Cargo.lock` will remain unchanged
    #[serde(default)]
    locked: bool,
}

impl CargoDenyCheckTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny");
        cmd.arg("--locked");

        if let Some(log_level) = &self.log_level {
            cmd.arg("--log-level").arg(log_level);
        }

        if let Some(format) = &self.format {
            cmd.arg("--format").arg(format);
        }

        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }

        if self.workspace {
            cmd.arg("--workspace");
        }

        if let Some(exclude) = &self.exclude {
            for item in exclude {
                cmd.arg("--exclude").arg(item);
            }
        }

        if let Some(target) = &self.target {
            for item in target {
                cmd.arg("--target").arg(item);
            }
        }

        if self.all_features {
            cmd.arg("--all-features");
        }

        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        if self.offline {
            cmd.arg("--offline");
        }

        cmd.arg("check");

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(graph) = &self.graph {
            cmd.arg("--graph").arg(graph);
        }

        if self.hide_inclusion_graph {
            cmd.arg("--hide-inclusion-graph");
        }

        if self.disable_fetch {
            cmd.arg("--disable-fetch");
        }

        if self.exclude_dev {
            cmd.arg("--exclude-dev");
        }

        if self.show_stats {
            cmd.arg("--show-stats");
        }

        if let Some(warn) = &self.warn {
            for item in warn {
                cmd.arg("-W").arg(item);
            }
        }

        if let Some(allow) = &self.allow {
            for item in allow {
                cmd.arg("-A").arg(item);
            }
        }

        if let Some(deny) = &self.deny {
            for item in deny {
                cmd.arg("-D").arg(item);
            }
        }

        if let Some(feature_depth) = &self.feature_depth {
            cmd.arg("--feature-depth").arg(feature_depth.to_string());
        }

        if let Some(which) = &self.which {
            for check in which {
                cmd.arg(check);
            }
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-deny-init",
    description = "Creates a cargo-deny config from a template",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoDenyInitTool {
    /// The path to create. Defaults to <cwd>/deny.toml
    config: Option<String>,
}

impl CargoDenyInitTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny").arg("init");

        if let Some(config) = &self.config {
            cmd.arg(config);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-deny-list",
    description = "Outputs a listing of all licenses and the crates that use them",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoDenyListTool {
    /// Path to the config to use. Defaults to a deny.toml in the same folder as the manifest path
    config: Option<String>,

    /// Minimum confidence threshold for license text (0.0 - 1.0, default: 0.8)
    threshold: Option<f64>,

    /// The format of the output (human, json, tsv)
    format: Option<String>,

    /// The layout for the output, does not apply to TSV (crate, license)
    layout: Option<String>,
}

impl CargoDenyListTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("deny").arg("list");

        if let Some(config) = &self.config {
            cmd.arg("--config").arg(config);
        }

        if let Some(threshold) = &self.threshold {
            cmd.arg("--threshold").arg(threshold.to_string());
        }

        if let Some(format) = &self.format {
            cmd.arg("--format").arg(format);
        }

        if let Some(layout) = &self.layout {
            cmd.arg("--layout").arg(layout);
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "cargo-deny-install",
    description = "Installs cargo-deny tool for dependency graph analysis and security checks",
    openWorldHint = false,
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CargoDenyInstallTool {}

impl CargoDenyInstallTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("cargo-deny");

        execute_command(cmd)
    }
}
