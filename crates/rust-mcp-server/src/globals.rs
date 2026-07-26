use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

/// The workspace root together with how it was determined.
#[derive(Debug)]
pub(crate) enum WorkspaceRoot {
    /// Set explicitly (e.g. via the CLI). Takes precedence over any per-command
    /// manifest directory.
    Explicit(PathBuf),
    /// Auto-detected from the client's roots.
    Detected(PathBuf),
}

impl WorkspaceRoot {
    pub(crate) fn path(&self) -> &Path {
        match self {
            Self::Explicit(path) | Self::Detected(path) => path,
        }
    }
}

static WORKSPACE_ROOT: OnceLock<WorkspaceRoot> = OnceLock::new();
static DEFAULT_REGISTRY: OnceLock<String> = OnceLock::new();

/// Sets the workspace root explicitly (e.g. from the CLI). An explicitly set
/// root takes precedence over any per-command manifest directory.
pub fn set_workspace_root(root: impl Into<PathBuf>) {
    WORKSPACE_ROOT
        .set(WorkspaceRoot::Explicit(root.into()))
        .expect("Workspace root can only be set once");
}

/// Attempts to set the workspace root via auto-detection. Returns `true` if set
/// successfully, `false` if it was already set (e.g. via CLI argument).
pub fn try_set_workspace_root(root: impl Into<PathBuf>) -> bool {
    WORKSPACE_ROOT
        .set(WorkspaceRoot::Detected(root.into()))
        .is_ok()
}

pub fn get_workspace_root() -> Option<&'static WorkspaceRoot> {
    WORKSPACE_ROOT.get()
}

pub fn set_default_registry(registry: String) {
    DEFAULT_REGISTRY
        .set(registry)
        .expect("Default registry can only be set once");
}

pub fn get_default_registry() -> Option<&'static str> {
    DEFAULT_REGISTRY.get().map(|s| s.as_str())
}
