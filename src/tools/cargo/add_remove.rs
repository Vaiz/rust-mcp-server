use std::process::Command;

use crate::{
    Response, Tool, execute_command,
    serde_utils::{
        PackageWithVersion, deserialize_string, deserialize_string_vec, locking_mode_to_cli_flags,
        output_verbosity_to_cli_flags,
    },
    tools::effective_registry,
};
use rmcp::ErrorData;

fn dependency_type_to_cli_flag(
    dependency_type: Option<&str>,
) -> Result<Option<&'static str>, ErrorData> {
    Ok(match dependency_type {
        None => None,
        Some("regular") => None,
        Some("dev") => Some("--dev"),
        Some("build") => Some("--build"),
        Some(dep) => {
            return Err(ErrorData::invalid_params(
                format!("Unknown dependency type: {dep}"),
                None,
            ));
        }
    })
}

/// Adds a dependency to a Rust project using cargo add.
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoAddRequest {
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
    pub no_default_features: Option<bool>,

    /// Re-enable the default features
    #[serde(default)]
    pub default_features: bool,

    /// Space or comma separated list of features to activate
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub features: Option<Vec<String>>,

    /// Rename the dependency
    #[serde(default, deserialize_with = "deserialize_string")]
    pub rename: Option<String>,

    /// Package to modify, must be specified
    pub target_package: String,

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
    pub dry_run: Option<bool>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[serde(default)]
    pub ignore_rust_version: Option<bool>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked": Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked" (default): Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    pub locking_mode: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    pub output_verbosity: Option<String>,
}

impl CargoAddRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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
        if self.no_default_features.unwrap_or(false) {
            cmd.arg("--no-default-features");
        }
        if self.default_features {
            cmd.arg("--default-features");
        }
        if let Some(features) = &self.features {
            cmd.arg("--features").arg(features.join(","));
        }

        // Package selection
        cmd.arg("--package").arg(&self.target_package);

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
        if let Some(registry) = effective_registry(self.registry.as_deref()) {
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
        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }
        if self.ignore_rust_version.unwrap_or(false) {
            cmd.arg("--ignore-rust-version");
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "unlocked")?;
        for flag in locking_flags {
            cmd.arg(flag);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        Ok(cmd)
    }
}

pub struct CargoAddRmcpTool;

impl Tool for CargoAddRmcpTool {
    const NAME: &'static str = "cargo-add";
    const TITLE: &'static str = "Add Rust dependency";
    const DESCRIPTION: &'static str = "Adds a dependency to a Rust project using cargo add.";
    type RequestArgs = CargoAddRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

/// Remove dependencies from a Cargo.toml manifest file.
#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoRemoveRequest {
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

    /// Package to remove from, must be specified
    pub target_package: String,

    /// Don't actually write the manifest
    #[serde(default)]
    pub dry_run: Option<bool>,

    /// Path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    pub manifest_path: Option<String>,

    /// Path to Cargo.lock (unstable)
    #[serde(default, deserialize_with = "deserialize_string")]
    pub lockfile_path: Option<String>,

    /// Locking mode for dependency resolution.
    ///
    /// Valid options:
    /// - "locked": Assert that `Cargo.lock` will remain unchanged
    /// - "unlocked" (default): Allow `Cargo.lock` to be updated
    /// - "offline": Run without accessing the network
    /// - "frozen": Equivalent to specifying both --locked and --offline
    #[serde(default, deserialize_with = "deserialize_string")]
    pub locking_mode: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    pub output_verbosity: Option<String>,
}

impl CargoRemoveRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
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
        cmd.arg("--package").arg(&self.target_package);

        // Other options
        if self.dry_run.unwrap_or(false) {
            cmd.arg("--dry-run");
        }

        // Manifest options
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(lockfile_path) = &self.lockfile_path {
            cmd.arg("--lockfile-path").arg(lockfile_path);
        }

        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref(), "unlocked")?;
        for flag in locking_flags {
            cmd.arg(flag);
        }

        // Output options
        let output_flags = output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?;
        cmd.args(output_flags);

        Ok(cmd)
    }
}

pub struct CargoRemoveRmcpTool;

impl Tool for CargoRemoveRmcpTool {
    const NAME: &'static str = "cargo-remove";
    const TITLE: &'static str = "Remove Rust dependency";
    const DESCRIPTION: &'static str = "Remove dependencies from a Cargo.toml manifest file.";
    type RequestArgs = CargoRemoveRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<Response, ErrorData> {
        execute_command(request.build_cmd()?, Self::NAME).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::tool::DynTool;

    use super::*;

    #[test]
    fn test_dependency_type_helper() {
        // Test CLI flags
        assert_eq!(dependency_type_to_cli_flag(Some("regular")).unwrap(), None);
        assert_eq!(
            dependency_type_to_cli_flag(Some("dev")).unwrap(),
            Some("--dev")
        );
        assert_eq!(
            dependency_type_to_cli_flag(Some("build")).unwrap(),
            Some("--build")
        );
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
  "description": "Adds a dependency to a Rust project using cargo add.",
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
      "default": null,
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
    "git": {
      "default": null,
      "description": "Git repository location",
      "type": "string"
    },
    "ignore_rust_version": {
      "default": null,
      "description": "Ignore `rust-version` specification in packages",
      "type": "boolean"
    },
    "locking_mode": {
      "default": null,
      "description": "Locking mode for dependency resolution.\n\nValid options:\n- \"locked\": Assert that `Cargo.lock` will remain unchanged\n- \"unlocked\" (default): Allow `Cargo.lock` to be updated\n- \"offline\": Run without accessing the network\n- \"frozen\": Equivalent to specifying both --locked and --offline",
      "type": "string"
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
      "default": null,
      "description": "Disable the default features",
      "type": "boolean"
    },
    "optional": {
      "default": false,
      "description": "Add as an optional dependency",
      "type": "boolean"
    },
    "output_verbosity": {
      "default": null,
      "description": "Output verbosity level.\n\nValid options:\n- \"quiet\" (default): Show only the essential command output\n- \"normal\": Show standard output (no additional flags)\n- \"verbose\": Show detailed output including build information",
      "type": "string"
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
      "description": "Package to modify, must be specified",
      "type": "string"
    },
    "toolchain": {
      "default": null,
      "description": "The toolchain to use, e.g., \"stable\" or \"nightly\".",
      "type": "string"
    },
    "version": {
      "default": null,
      "description": "Optional version specification",
      "type": "string"
    }
  },
  "required": [
    "package",
    "target_package"
  ],
  "title": "CargoAddRequest",
  "type": "object"
}"##;
        let schema = serde_json::Value::from(CargoAddRmcpTool {}.json_schema());
        println!(
            "CargoAddRequest schema: {}",
            serde_json::to_string_pretty(&schema).unwrap()
        );

        let expected_schema: serde_json::Value = serde_json::from_str(EXPECTED_SCHEMA).unwrap();
        assert_eq!(
            schema, expected_schema,
            "CargoAddRequest schema should match expected structure"
        );
    }
}
