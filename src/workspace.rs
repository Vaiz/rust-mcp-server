use rmcp::{model::Root, service::NotificationContext};

use crate::globals;

/// Decides whether automatic workspace detection is needed and, if so,
/// tries to find a Cargo project through the MCP client's roots capability.
///
/// Decision tree:
/// 1. CWD contains `Cargo.toml`      -> already in a Cargo project, nothing to do.
/// 2. Client supports roots          -> spawn a task that iterates the roots and
///    sets the first one that contains a `Cargo.toml` as the workspace root.
pub fn detect_rust_workspace(context: NotificationContext<rmcp::RoleServer>) {
    let cwd = std::env::current_dir().ok();
    tracing::info!("Checking current working directory for Cargo project: {cwd:?}");
    if let Some(cwd_path) = cwd {
        if cwd_path.join("Cargo.toml").exists() {
            tracing::info!(
                "Cargo.toml found in CWD ({}), using it as workspace root (no auto-detection needed)",
                cwd_path.display()
            );
            return;
        }
        tracing::info!(
            "No Cargo.toml in CWD ({}), will attempt workspace detection via client roots",
            cwd_path.display()
        );
    }

    // Step 3: ask the client for its workspace roots
    let supports_roots = context
        .peer
        .peer_info()
        .and_then(|info| info.capabilities.roots.as_ref())
        .is_some();
    tracing::info!("Checking client roots capability: supports_roots={supports_roots}");

    if !supports_roots {
        tracing::warn!(
            "Client does not support roots capability; workspace auto-detection is not possible"
        );
        return;
    }

    // Spawn onto a separate task to avoid blocking the notification handler,
    // which would deadlock if the client waits for the server to finish
    // processing this notification before responding to roots/list.
    tokio::spawn(async move {
        tracing::info!("Requesting workspace roots from client");
        let result = match context.peer.list_roots().await {
            Ok(result) => result,
            Err(e) => {
                tracing::warn!("Failed to fetch client roots: {e}");
                return;
            }
        };

        tracing::info!(
            "Received {} root(s) from client: {:?}",
            result.roots.len(),
            result.roots
        );
        for Root { uri, .. } in result.roots {
            tracing::info!("Checking root for Cargo project: {uri}");
            let Some(path) = file_uri_to_path(&uri) else {
                tracing::warn!("Could not convert root URI to a filesystem path: {uri}");
                continue;
            };
            if path.join("Cargo.toml").exists() {
                let path_str = path.to_str().expect("Invalid UTF-8 in path").to_owned();
                tracing::info!("Found Cargo project in root, setting as workspace: {path_str}");
                globals::try_set_workspace_root(path_str);
                return;
            }
            tracing::debug!("No Cargo.toml found in root: {}", path.display());
        }
        tracing::warn!("No Cargo project found in any client root; workspace unset");
    });
}

/// Convert a `file://` URI to a local filesystem path.
///
/// Handles:
/// - `file:///path/to/dir` (Unix)
/// - `file:///C:/path/to/dir` (Windows, leading slash before drive letter stripped)
/// - `file://localhost/path` (optional localhost authority)
fn file_uri_to_path(uri: &str) -> Option<std::path::PathBuf> {
    let after_scheme = uri.strip_prefix("file://")?;
    // Strip optional "localhost" authority (file://localhost/path)
    let path_str = after_scheme
        .strip_prefix("localhost")
        .unwrap_or(after_scheme);
    let path_str = strip_windows_drive_slash(path_str);
    Some(std::path::PathBuf::from(path_str))
}

#[cfg(windows)]
fn strip_windows_drive_slash(path: &str) -> &str {
    let bytes = path.as_bytes();
    if bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':' {
        &path[1..]
    } else {
        path
    }
}

#[cfg(not(windows))]
fn strip_windows_drive_slash(path: &str) -> &str {
    path
}

pub fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = globals::get_workspace_root() {
        cmd.current_dir(root);
    }
}
