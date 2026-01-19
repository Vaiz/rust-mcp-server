pub mod cargo;
pub mod cargo_deny;
pub mod cargo_expand;
pub mod cargo_hack;
pub mod cargo_machete;
pub mod rustc;
pub mod rustup;

static WORKSPACE_ROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_workspace_root(root: String) {
    WORKSPACE_ROOT
        .set(root)
        .expect("Workspace root can only be set once");
}

pub fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = WORKSPACE_ROOT.get() {
        cmd.current_dir(root);
    }
}
