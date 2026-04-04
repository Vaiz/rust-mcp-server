use rmcp::{model::Root, service::NotificationContext};

use crate::globals;

/// Applies the workspace root to a command if it is set
pub fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = globals::get_workspace_root() {
        cmd.current_dir(root);
    }
}

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

    // Spawn onto a separate task to avoid blocking the notification handler,
    // which would deadlock if the client waits for the server to finish
    // processing this notification before responding to roots/list.
    let fut = async move {
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
            let Some(path) = file_uri_to_path(&uri) else {
                tracing::warn!("Could not convert root URI to a filesystem path: {uri}");
                continue;
            };
            tracing::info!(
                "Checking root for Cargo project: {uri} -> {}",
                path.display()
            );
            if path.join("Cargo.toml").exists() {
                tracing::info!(
                    "Found Cargo project in root, setting as workspace: {}",
                    path.display()
                );
                globals::try_set_workspace_root(path);
                return;
            }
            tracing::debug!("No Cargo.toml found in root: {}", path.display());
        }
        tracing::warn!("No Cargo project found in any client root; workspace unset");
    };

    tokio::spawn(async move {
        let _ = tokio::time::timeout(std::time::Duration::from_secs(10), fut)
            .await
            .inspect_err(|_| {
                tracing::warn!("Workspace detection timed out after 10 seconds");
            });
    });
}

/// Convert a `file://` URI to a local filesystem path.
///
/// Handles:
/// - `file:///path/to/dir` (Unix)
/// - `file:///C:/path/to/dir` (Windows, leading slash before drive letter stripped)
/// - `file:///d%3A/path` (Windows, percent-encoded colon in drive letter)
/// - `file://localhost/path` (optional localhost authority)
fn file_uri_to_path(uri: &str) -> Option<std::path::PathBuf> {
    let path = uri.strip_prefix("file://")?;
    let path = path.strip_prefix("localhost").unwrap_or(path);
    let decoded = percent_encoding::percent_decode_str(path)
        .decode_utf8()
        .ok()?;
    // On Windows, strip the leading slash before the drive letter (/C:/ -> C:/)
    #[cfg(windows)]
    let decoded = {
        let b = decoded.as_bytes();
        // `/C:`
        if b.len() >= 3 && b[0] == b'/' && b[1].is_ascii_alphabetic() && b[2] == b':' {
            &decoded[1..]
        } else {
            &decoded
        }
    };
    Some(std::path::PathBuf::from(decoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_file_uri_returns_none() {
        assert!(file_uri_to_path("https://example.com/path").is_none());
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_drive_letter() {
        let path = file_uri_to_path("file:///C:/Users/user/project").unwrap();
        assert_eq!(path, std::path::PathBuf::from("C:\\Users\\user\\project"));
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_percent_encoded_colon() {
        let path = file_uri_to_path("file:///d%3A/projects/myapp").unwrap();
        assert_eq!(path, std::path::PathBuf::from("d:\\projects\\myapp"));
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_percent_encoded_colon_uppercase() {
        let path = file_uri_to_path("file:///D%3A/projects/myapp").unwrap();
        assert_eq!(path, std::path::PathBuf::from("D:\\projects\\myapp"));
    }

    #[cfg(not(windows))]
    #[test]
    fn test_path_with_spaces() {
        // Spaces encoded as %20
        let path = file_uri_to_path("file:///path%20with%20spaces").unwrap();
        assert_eq!(path, std::path::PathBuf::from("/path with spaces"));
    }

    #[cfg(not(windows))]
    #[test]
    fn test_unix_path() {
        let path = file_uri_to_path("file:///home/user/project").unwrap();
        assert_eq!(path, std::path::PathBuf::from("/home/user/project"));
    }

    #[cfg(not(windows))]
    #[test]
    fn test_localhost_authority() {
        let path = file_uri_to_path("file://localhost/home/user/project").unwrap();
        assert_eq!(path, std::path::PathBuf::from("/home/user/project"));
    }
}
