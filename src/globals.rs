use std::sync::OnceLock;

static WORKSPACE_ROOT: OnceLock<String> = OnceLock::new();
static DEFAULT_REGISTRY: OnceLock<String> = OnceLock::new();

pub fn set_workspace_root(root: String) {
    WORKSPACE_ROOT
        .set(root)
        .expect("Workspace root can only be set once");
}

/// Attempts to set the workspace root. Returns `true` if set successfully,
/// `false` if it was already set (e.g. via CLI argument).
pub fn try_set_workspace_root(root: String) -> bool {
    WORKSPACE_ROOT.set(root).is_ok()
}

pub fn get_workspace_root() -> Option<&'static str> {
    WORKSPACE_ROOT.get().map(|s| s.as_str())
}

pub fn set_default_registry(registry: String) {
    DEFAULT_REGISTRY
        .set(registry)
        .expect("Default registry can only be set once");
}

pub fn get_default_registry() -> Option<&'static str> {
    DEFAULT_REGISTRY.get().map(|s| s.as_str())
}
