use std::sync::OnceLock;

static WORKSPACE_ROOT: OnceLock<String> = OnceLock::new();
static DEFAULT_REGISTRY: OnceLock<String> = OnceLock::new();

pub fn set_workspace_root(root: String) {
    WORKSPACE_ROOT
        .set(root)
        .expect("Workspace root can only be set once");
}

pub fn get_workspace_root() -> Option<&'static str> {
    WORKSPACE_ROOT.get().map(|s| s.as_str())
}

pub fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = WORKSPACE_ROOT.get() {
        cmd.current_dir(root);
    }
}

pub fn set_default_registry(registry: String) {
    DEFAULT_REGISTRY
        .set(registry)
        .expect("Default registry can only be set once");
}

pub fn get_default_registry() -> Option<&'static str> {
    DEFAULT_REGISTRY.get().map(|s| s.as_str())
}
