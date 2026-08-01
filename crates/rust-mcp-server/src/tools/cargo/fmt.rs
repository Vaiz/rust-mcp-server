//! Cargo-fmt compatibility fixes:
//!
//! - rust-mcp-server#136: `cargo fmt --manifest-path` fails for virtual manifests unless
//!   `--all` or an explicit package is selected. Add `--all` automatically when the caller
//!   leaves `all` unset.
//! - rust-lang/rustfmt#6934: large Windows workspaces can exceed the OS command-line limit.
//!   Retry eligible workspace or explicit-package requests one package at a time. Never apply
//!   this retry to `--all`, because it also covers local path dependencies outside the workspace.

use std::{path::Path, process::Command};

use crate::{
    Response, Tool, command_cwd, execute_command,
    serde_utils::{deserialize_string, deserialize_string_vec, output_verbosity_to_cli_flags},
};
use rmcp::ErrorData;
use serde::Deserialize;

#[derive(Debug, ::serde::Deserialize, schemars::JsonSchema)]
pub struct CargoFmtRequest {
    /// The toolchain to use, e.g., "stable" or "nightly".
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,

    /// The name of the package(s) to format. If not specified, formats the current package.
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    package: Option<Vec<String>>,

    /// Format all packages, and also their local path-based dependencies.
    /// When unset, `--all` is added automatically for virtual workspace manifests.
    #[serde(default)]
    all: Option<bool>,

    /// Run rustfmt in check mode (don't write changes, just check if formatting is needed)
    #[serde(default)]
    check: bool,

    /// Specify path to Cargo.toml
    #[serde(default, deserialize_with = "deserialize_string")]
    manifest_path: Option<String>,

    /// Specify message-format: short|json|human
    #[serde(default, deserialize_with = "deserialize_string")]
    message_format: Option<String>,

    /// Output verbosity level.
    ///
    /// Valid options:
    /// - "quiet" (default): Show only the essential command output
    /// - "normal": Show standard output (no additional flags)
    /// - "verbose": Show detailed output including build information
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
}

impl CargoFmtRequest {
    pub fn build_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = self.base_cmd()?;

        if let Some(packages) = &self.package {
            for package in packages {
                cmd.arg("--package").arg(package);
            }
        }

        if self.use_all() {
            cmd.arg("--all");
        }

        Ok(cmd)
    }

    /// Builds a `cargo fmt` command scoped to a single package.
    fn build_package_cmd(&self, package: &str) -> Result<Command, ErrorData> {
        let mut cmd = self.base_cmd()?;
        cmd.arg("--package").arg(package);
        Ok(cmd)
    }

    /// `cargo [+toolchain] fmt` with formatting, manifest and output options.
    fn base_cmd(&self) -> Result<Command, ErrorData> {
        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.arg("fmt");
        if self.check {
            cmd.arg("--check");
        }
        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(message_format) = &self.message_format {
            cmd.arg("--message-format").arg(message_format);
        }
        cmd.args(output_verbosity_to_cli_flags(
            self.output_verbosity.as_deref(),
        )?);
        Ok(cmd)
    }

    fn use_all(&self) -> bool {
        self.all.unwrap_or_else(|| {
            self.package.as_ref().is_none_or(Vec::is_empty)
                && self
                    .manifest_path
                    .as_deref()
                    .is_some_and(is_virtual_manifest)
        })
    }
}

#[cfg(not(windows))]
fn cmd_line_too_long(_stderr: &str) -> bool {
    false
}

/// Returns `true` when `stderr` shows the rust-lang/rustfmt#6934 failure.
#[cfg(windows)]
fn cmd_line_too_long(stderr: &str) -> bool {
    stderr.contains("os error 206") || stderr.contains("filename or extension is too long")
}

fn is_virtual_manifest(manifest_path: &str) -> bool {
    let Ok(contents) = std::fs::read_to_string(manifest_path) else {
        return false;
    };
    manifest_contents_are_virtual(&contents)
}

fn manifest_contents_are_virtual(contents: &str) -> bool {
    let mut has_workspace = false;
    let mut has_package = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("[workspace]") || line.starts_with("[workspace.") {
            has_workspace = true;
        } else if line.starts_with("[package]") || line.starts_with("[package.") {
            has_package = true;
        }
    }
    has_workspace && !has_package
}

#[derive(Deserialize)]
struct CargoMetadata {
    workspace_root: std::path::PathBuf,
    packages: Vec<MetadataPackage>,
}

#[derive(Deserialize)]
struct MetadataPackage {
    name: String,
    manifest_path: std::path::PathBuf,
}

fn same_path(left: &Path, right: &Path) -> bool {
    match (left.canonicalize(), right.canonicalize()) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

fn packages_for_cwd(metadata: CargoMetadata, cwd: &Path) -> Vec<String> {
    if same_path(&metadata.workspace_root, cwd) {
        return metadata
            .packages
            .into_iter()
            .map(|package| package.name)
            .collect();
    }

    metadata
        .packages
        .into_iter()
        .filter(|package| {
            package
                .manifest_path
                .parent()
                .is_some_and(|path| same_path(path, cwd))
        })
        .map(|package| package.name)
        .collect()
}

pub struct CargoFmtRmcpTool;

impl CargoFmtRmcpTool {
    fn retry_packages(
        request: &CargoFmtRequest,
        cwd: Option<&Path>,
    ) -> Result<Option<Vec<String>>, ErrorData> {
        if request.use_all() {
            return Ok(None);
        }

        if let Some(packages) = request
            .package
            .as_ref()
            .filter(|packages| !packages.is_empty())
        {
            return Ok((packages.len() > 1).then(|| packages.clone()));
        }

        let mut cmd = Command::new("cargo");
        if let Some(toolchain) = &request.toolchain {
            cmd.arg(format!("+{toolchain}"));
        }
        cmd.args(["metadata", "--no-deps", "--format-version", "1"]);
        if let Some(manifest_path) = &request.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path);
        }
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd
            .output()
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        if !output.status.success() {
            tracing::warn!(
                "Could not inspect workspace for cargo fmt retry: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Ok(None);
        }

        let metadata: CargoMetadata = match serde_json::from_slice(&output.stdout) {
            Ok(metadata) => metadata,
            Err(error) => {
                tracing::warn!("Could not parse cargo metadata for cargo fmt retry: {error}");
                return Ok(None);
            }
        };
        let effective_cwd = cwd
            .map(Path::to_path_buf)
            .or_else(|| std::env::current_dir().ok());
        let Some(effective_cwd) = effective_cwd else {
            return Ok(None);
        };
        let packages = packages_for_cwd(metadata, &effective_cwd);
        Ok((packages.len() > 1).then_some(packages))
    }

    fn format_per_package(
        request: &CargoFmtRequest,
        packages: &[String],
        cwd: Option<&Path>,
    ) -> Result<Response, ErrorData> {
        tracing::warn!(
            "cargo fmt hit rust-lang/rustfmt#6934; retrying {} packages individually",
            packages.len()
        );

        let mut outputs = Vec::with_capacity(packages.len());
        for package in packages {
            let output = execute_command(
                request.build_package_cmd(package)?,
                CargoFmtRmcpTool::NAME,
                cwd,
            )?;
            let failed = !output.success();
            outputs.push(output);
            if failed {
                break;
            }
        }

        let executed = outputs.len();
        let failed = outputs.last().is_some_and(|output| !output.success());
        let mut response = Response::from_outputs(outputs);
        response.add_recommendation(format!(
            "cargo fmt hit rust-lang/rustfmt#6934; retried {executed} of {} packages individually",
            packages.len()
        ));
        if failed && request.check {
            response.add_recommendation(format!(
                "Run #{} with `check: false` to automatically format the code",
                CargoFmtRmcpTool::NAME
            ));
        }
        Ok(response)
    }
}

impl Tool for CargoFmtRmcpTool {
    const NAME: &'static str = "cargo-fmt";
    const TITLE: &'static str = "Format Rust code";
    const DESCRIPTION: &'static str =
        "Formats Rust code using rustfmt. Usually, run without any additional arguments.";
    type RequestArgs = CargoFmtRequest;

    fn call_rmcp_tool(&self, request: Self::RequestArgs) -> Result<Response, ErrorData> {
        let cwd = command_cwd(request.manifest_path.as_deref());
        let output = execute_command(request.build_cmd()?, Self::NAME, cwd)?;

        // Work around rust-lang/rustfmt#6934 only when that specific failure hit.
        if !output.success()
            && output
                .stderr
                .as_ref()
                .is_some_and(|e| cmd_line_too_long(&e.0))
            && let Some(packages) = Self::retry_packages(&request, cwd)?
        {
            return Self::format_per_package(&request, &packages, cwd);
        }

        let failed = !output.success();
        let mut response: Response = output.into();

        if failed && request.check {
            response.add_recommendation(format!(
                "Run #{} with `check: false` to automatically format the code",
                Self::NAME
            ));
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_change_default_scope_for_manifest_path() {
        let request: CargoFmtRequest = serde_json::from_value(serde_json::json!({
            "manifest_path": "/workspace/Cargo.toml"
        }))
        .unwrap();
        let cmd = request.build_cmd().unwrap();
        let args = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            args,
            ["fmt", "--manifest-path", "/workspace/Cargo.toml", "--quiet"]
        );
    }

    #[test]
    fn adds_all_for_virtual_manifest() {
        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("Cargo.toml");
        let request: CargoFmtRequest = serde_json::from_value(serde_json::json!({
            "manifest_path": manifest_path
        }))
        .unwrap();
        let cmd = request.build_cmd().unwrap();
        let args = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert!(args.iter().any(|arg| arg == "--all"));
        assert_eq!(
            CargoFmtRmcpTool::retry_packages(&request, None).unwrap(),
            None
        );
    }

    #[test]
    fn detects_virtual_manifest_contents() {
        assert!(manifest_contents_are_virtual(
            "[workspace]\nmembers = [\"crates/*\"]\n"
        ));
        assert!(manifest_contents_are_virtual("[workspace.package]\n"));
        assert!(!manifest_contents_are_virtual(
            "[package]\nname = \"foo\"\n"
        ));
        assert!(!manifest_contents_are_virtual(
            "[workspace]\n\n[package.metadata]\nkey = \"value\"\n"
        ));
    }

    #[cfg(windows)]
    #[test]
    fn detects_command_line_too_long() {
        // Windows surfaces rust-lang/rustfmt#6934 as os error 206.
        assert!(cmd_line_too_long(
            "error: The filename or extension is too long. (os error 206)"
        ));
        assert!(cmd_line_too_long("The filename or extension is too long."));
        assert!(!cmd_line_too_long(
            "error[internal]: left behind trailing whitespace"
        ));
    }

    #[cfg(not(windows))]
    #[test]
    fn ignores_command_line_too_long() {
        assert!(!cmd_line_too_long(
            "error: The filename or extension is too long. (os error 206)"
        ));
        assert!(!cmd_line_too_long(
            "error: Argument list too long (os error 7)"
        ));
    }

    #[test]
    fn selects_all_packages_at_workspace_root() {
        let metadata = CargoMetadata {
            workspace_root: "/workspace".into(),
            packages: vec![
                MetadataPackage {
                    name: "first".into(),
                    manifest_path: "/workspace/first/Cargo.toml".into(),
                },
                MetadataPackage {
                    name: "second".into(),
                    manifest_path: "/workspace/second/Cargo.toml".into(),
                },
            ],
        };

        assert_eq!(
            packages_for_cwd(metadata, Path::new("/workspace")),
            ["first", "second"]
        );
    }

    #[test]
    fn selects_only_current_workspace_member() {
        let metadata = CargoMetadata {
            workspace_root: "/workspace".into(),
            packages: vec![
                MetadataPackage {
                    name: "first".into(),
                    manifest_path: "/workspace/first/Cargo.toml".into(),
                },
                MetadataPackage {
                    name: "second".into(),
                    manifest_path: "/workspace/second/Cargo.toml".into(),
                },
            ],
        };

        assert_eq!(
            packages_for_cwd(metadata, Path::new("/workspace/first")),
            ["first"]
        );
    }

    #[test]
    fn retries_explicit_packages_without_discovery() {
        let request: CargoFmtRequest = serde_json::from_value(serde_json::json!({
            "package": ["first", "second"]
        }))
        .unwrap();

        assert_eq!(
            CargoFmtRmcpTool::retry_packages(&request, None).unwrap(),
            Some(vec!["first".into(), "second".into()])
        );
    }

    #[test]
    fn does_not_narrow_all_to_workspace_packages() {
        let request: CargoFmtRequest =
            serde_json::from_value(serde_json::json!({ "all": true })).unwrap();

        assert_eq!(
            CargoFmtRmcpTool::retry_packages(&request, None).unwrap(),
            None
        );
    }
}
