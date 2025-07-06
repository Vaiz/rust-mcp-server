use std::process::Command;

use crate::serde_utils::Tool;
use crate::{
    serde_utils::{PackageWithVersion, default_true, deserialize_string, deserialize_string_vec},
    tools::execute_command,
};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::{
    macros::mcp_tool,
    schema::{CallToolResult, schema_utils::CallToolError},
};

fn dependency_type_to_cli_flag(
    dependency_type: Option<&str>,
) -> Result<Option<&'static str>, CallToolError> {
    Ok(match dependency_type {
        None => None,
        Some("regular") => None,
        Some("dev") => Some("--dev"),
        Some("build") => Some("--build"),
        Some(dep) => {
            return Err(CallToolError(
                anyhow::anyhow!("Unknown dependency type: {dep}").into(),
            ));
        }
    })
}

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-add",
    description = "Adds a dependency to a Rust project using cargo add.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoAddTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Package with optional version (e.g., {"package": "serde", "version": "1.0.0"})
    #[serde(flatten)]
    pub package_spec: PackageWithVersion,

    /// Dependency type: "regular" (default), "dev", or "build"
    #[serde(default, deserialize_with = "deserialize_string")]
    pub dependency_type: Option<String>,

    /// Add as an optional dependency
    #[serde(default)]
    pub optional: bool,

    /// Disable the default features
    #[serde(default)]
    pub no_default_features: bool,

    /// Re-enable the default features
    #[serde(default)]
    pub default_features: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub features: Option<Vec<String>>,

    /// Rename the dependency
    #[serde(default, deserialize_with = "deserialize_string")]
    pub rename: Option<String>,

    /// Package to modify
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target_package: Option<String>,

    /// Filesystem path to local crate to add
    #[serde(default, deserialize_with = "deserialize_string")]
    pub path: Option<String>,

    /// Git repository location
    #[serde(default, deserialize_with = "deserialize_string")]
    pub git: Option<String>,

    /// Git branch to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub branch: Option<String>,

    /// Git tag to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub tag: Option<String>,

    /// Git reference to download the crate from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub rev: Option<String>,

    /// Package registry for this dependency
    #[serde(default, deserialize_with = "deserialize_string")]
    pub registry: Option<String>,

    /// Add as dependency to the given target platform
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target: Option<String>,

    /// Don't actually write the manifest
    #[serde(default)]
    pub dry_run: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[serde(default)]
    pub ignore_rust_version: bool,

    /// Assert that `Cargo.lock` will remain unchanged.
    #[serde(default)]
    pub locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    pub frozen: bool,

    /// Use verbose output
    #[serde(default)]
    pub verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl CargoAddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("add");

        cmd.arg(self.package_spec.to_spec());

        // Dependency type
        if let Some(flag) = dependency_type_to_cli_flag(self.dependency_type.as_deref())? {
            cmd.arg(flag);
        }

        if self.optional {
            cmd.arg("--optional");
        }

        // Feature selection
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }
        if self.default_features {
            cmd.arg("--default-features");
        }
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        // Package selection
        if let Some(target_package) = &self.target_package {
            cmd.arg("--package").arg(target_package);
        }

        // Source options
        if let Some(path) = &self.path {
            cmd.arg("--path").arg(path);
        }
        if let Some(git) = &self.git {
            cmd.arg("--git").arg(git);
        }
        if let Some(branch) = &self.branch {
            cmd.arg("--branch").arg(branch);
        }
        if let Some(tag) = &self.tag {
            cmd.arg("--tag").arg(tag);
        }
        if let Some(rev) = &self.rev {
            cmd.arg("--rev").arg(rev);
        }

        // Registry options
        if let Some(registry) = &self.registry {
            cmd.arg("--registry").arg(registry);
        }

        // Target platform
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Naming options
        if let Some(rename) = &self.rename {
            cmd.arg("--rename").arg(rename);
        }

        // Other options
        if self.dry_run {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }
        if self.ignore_rust_version {
            cmd.arg("--ignore-rust-version");
        }
        if self.locked {
            cmd.arg("--locked");
        }
        if self.offline {
            cmd.arg("--offline");
        }
        if self.frozen {
            cmd.arg("--frozen");
        }

        // Output options
        if self.verbose {
            cmd.arg("--verbose");
        }
        if self.quiet && !self.verbose {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

/// MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default
/// for better integration with automated tooling and to avoid blocking on missing lockfiles.
#[mcp_tool(
    name = "cargo-remove",
    description = "Remove dependencies from a Cargo.toml manifest file.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoRemoveTool {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// Dependencies to be removed.
    /// Examples:
    /// - Single dependency: ["regex"]
    /// - Multiple dependencies: ["tokio", "clap", "serde"]
    /// - Can be simple crate names as they appear in Cargo.toml
    pub dep_id: Vec<String>,

    /// Dependency type: "regular" (default), "dev", or "build"
    #[serde(default, deserialize_with = "deserialize_string")]
    pub dependency_type: Option<String>,

    /// Remove from target-dependencies
    #[serde(default, deserialize_with = "deserialize_string")]
    pub target: Option<String>,

    /// Package to remove from
    #[serde(default, deserialize_with = "deserialize_string")]
    pub package: Option<String>,

    /// Don't actually write the manifest
    #[serde(default)]
    pub dry_run: bool,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Assert that `Cargo.lock` will remain unchanged.
    #[serde(default)]
    pub locked: bool,

    /// Run without accessing the network
    #[serde(default)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[serde(default)]
    pub frozen: bool,

    /// Use verbose output
    #[serde(default)]
    pub verbose: bool,

    /// Do not print cargo log messages. By default is `true`.
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl CargoRemoveTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("remove");

        // Add dependency names
        for dep in &self.dep_id {
            cmd.arg(dep);
        }

        // Section options

        if let Some(flag) = dependency_type_to_cli_flag(self.dependency_type.as_deref())? {
            cmd.arg(flag);
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        // Package selection
        if let Some(package) = &self.package {
            cmd.arg("--package").arg(package);
        }

        // Other options
        if self.dry_run {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }
        if self.locked {
            cmd.arg("--locked");
        }
        if self.offline {
            cmd.arg("--offline");
        }
        if self.frozen {
            cmd.arg("--frozen");
        }

        // Output options
        if self.verbose {
            cmd.arg("--verbose");
        }
        if self.quiet && !self.verbose {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}

#[mcp_tool(
    name = "test-tool-1",
    description = "A test tool with an optional string parameter.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct TestTool1 {
    /// An optional string parameter for testing.
    #[serde(default, deserialize_with = "deserialize_string")]
    pub maybe_string: Option<String>,
}

impl TestTool1 {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let text = TextContent::new(
            format!("TestTool1 called with string: {:?}", self.maybe_string),
            None,
            None,
        );
        Ok(CallToolResult::text_content(vec![text.into()]))
    }
}

#[mcp_tool(
    name = "test-tool-2",
    description = "A test tool with a dependency type parameter.",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct TestTool2 {
    /// The dependency type to test: "regular", "dev", or "build"
    #[serde(default, deserialize_with = "deserialize_string")]
    pub dependency_type: Option<String>,
}

impl TestTool2 {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let dep_type = self.dependency_type.as_deref().unwrap_or("regular");
        let text = TextContent::new(
            format!("TestTool2 called with dependency type: {}", dep_type),
            None,
            None,
        );
        Ok(CallToolResult::text_content(vec![text.into()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_type_helper() {
        // Test CLI flags
        assert_eq!(dependency_type_to_cli_flag(Some("regular")).unwrap(), None);
        assert_eq!(dependency_type_to_cli_flag(Some("dev")).unwrap(), Some("--dev"));
        assert_eq!(dependency_type_to_cli_flag(Some("build")).unwrap(), Some("--build"));
        assert!(dependency_type_to_cli_flag(Some("unknown")).is_err());
    }

    #[test]
    fn test_dependency_type_serde() {
        // Test string parsing for dependency types
        assert_eq!(
            serde_json::from_str::<String>("\"regular\"").unwrap(),
            "regular"
        );
        assert_eq!(serde_json::from_str::<String>("\"dev\"").unwrap(), "dev");
        assert_eq!(
            serde_json::from_str::<String>("\"build\"").unwrap(),
            "build"
        );
    }

    #[test]
    fn test_cargo_add_schema() {
        const EXPECTED_SCHEMA: &str = r##"
        {
  "description": "MCP defaults differ from cargo defaults: `quiet` and `locked` are `true` by default\nfor better integration with automated tooling and to avoid blocking on missing lockfiles.",
  "properties": {
    "branch": {
      "default": null,
      "description": "Git branch to download the crate from",
      "type": "string"
    },
    "default_features": {
      "default": false,
      "description": "Re-enable the default features",
      "type": "boolean"
    },
    "dependency_type": {
      "default": null,
      "description": "Dependency type: \"regular\" (default), \"dev\", or \"build\"",
      "type": "string"
    },
    "dry_run": {
      "default": false,
      "description": "Don't actually write the manifest",
      "type": "boolean"
    },
    "features": {
      "default": null,
      "description": "Space or comma separated list of features to activate",
      "items": {
        "type": "string"
      },
      "type": "array"
    },
    "frozen": {
      "default": false,
      "description": "Equivalent to specifying both --locked and --offline",
      "type": "boolean"
    },
    "git": {
      "default": null,
      "description": "Git repository location",
      "type": "string"
    },
    "ignore_rust_version": {
      "default": false,
      "description": "Ignore `rust-version` specification in packages",
      "type": "boolean"
    },
    "locked": {
      "default": false,
      "description": "Assert that `Cargo.lock` will remain unchanged.",
      "type": "boolean"
    },
    "lockfile_path": {
      "default": null,
      "description": "Path to Cargo.lock (unstable)",
      "type": "string"
    },
    "manifest_path": {
      "default": null,
      "description": "Path to Cargo.toml",
      "type": "string"
    },
    "no_default_features": {
      "default": false,
      "description": "Disable the default features",
      "type": "boolean"
    },
    "offline": {
      "default": false,
      "description": "Run without accessing the network",
      "type": "boolean"
    },
    "optional": {
      "default": false,
      "description": "Add as an optional dependency",
      "type": "boolean"
    },
    "package": {
      "description": "The package name",
      "type": "string"
    },
    "path": {
      "default": null,
      "description": "Filesystem path to local crate to add",
      "type": "string"
    },
    "quiet": {
      "default": true,
      "description": "Do not print cargo log messages. By default is `true`.",
      "type": "boolean"
    },
    "registry": {
      "default": null,
      "description": "Package registry for this dependency",
      "type": "string"
    },
    "rename": {
      "default": null,
      "description": "Rename the dependency",
      "type": "string"
    },
    "rev": {
      "default": null,
      "description": "Git reference to download the crate from",
      "type": "string"
    },
    "tag": {
      "default": null,
      "description": "Git tag to download the crate from",
      "type": "string"
    },
    "target": {
      "default": null,
      "description": "Add as dependency to the given target platform",
      "type": "string"
    },
    "target_package": {
      "default": null,
      "description": "Package to modify",
      "type": "string"
    },
    "toolchain": {
      "default": null,
      "description": "The toolchain to use, e.g., \"stable\" or \"nightly\".",
      "type": "string"
    },
    "verbose": {
      "default": false,
      "description": "Use verbose output",
      "type": "boolean"
    },
    "version": {
      "default": null,
      "description": "Optional version specification",
      "type": "string"
    }
  },
  "required": [
    "package"
  ],
  "title": "CargoAddTool",
  "type": "object"
}"##;
        let schema = serde_json::Value::from(CargoAddTool::json_schema());
        println!(
            "CargoAddTool schema: {}",
            serde_json::to_string_pretty(&schema).unwrap()
        );

        let expected_schema: serde_json::Value = serde_json::from_str(EXPECTED_SCHEMA).unwrap();

        // Traverse both schemas to find the difference
        fn find_diff(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
            match (a, b) {
                (serde_json::Value::Object(map_a), serde_json::Value::Object(map_b)) => {
                    for (k, v_a) in map_a {
                        let new_path = if path.is_empty() {
                            k.clone()
                        } else {
                            format!("{}/{}", path, k)
                        };
                        match map_b.get(k) {
                            Some(v_b) => {
                                if let Some(diff) = find_diff(v_a, v_b, &new_path) {
                                    return Some(diff);
                                }
                            }
                            None => {
                                return Some(format!(
                                    "Key '{}' missing in expected at path '{}'",
                                    k, new_path
                                ));
                            }
                        }
                    }
                    for k in map_b.keys() {
                        if !map_a.contains_key(k) {
                            let new_path = if path.is_empty() {
                                k.clone()
                            } else {
                                format!("{}/{}", path, k)
                            };
                            return Some(format!(
                                "Extra key '{}' in expected at path '{}'",
                                k, new_path
                            ));
                        }
                    }
                    None
                }
                (serde_json::Value::Array(arr_a), serde_json::Value::Array(arr_b)) => {
                    if arr_a.len() != arr_b.len() {
                        return Some(format!(
                            "Array length mismatch at '{}': {} vs {}",
                            path,
                            arr_a.len(),
                            arr_b.len()
                        ));
                    }
                    for (i, (v_a, v_b)) in arr_a.iter().zip(arr_b.iter()).enumerate() {
                        let new_path = format!("{}[{}]", path, i);
                        if let Some(diff) = find_diff(v_a, v_b, &new_path) {
                            return Some(diff);
                        }
                    }
                    None
                }
                _ => {
                    if a != b {
                        Some(format!(
                            "Value mismatch at '{}': left = {}, right = {}",
                            path, a, b
                        ))
                    } else {
                        None
                    }
                }
            }
        }

        if let Some(diff) = find_diff(&schema, &expected_schema, "") {
            panic!("Schema difference found: {}", diff);
        }

        assert_eq!(
            schema, expected_schema,
            "CargoAddTool schema should match expected structure"
        );
    }
}
