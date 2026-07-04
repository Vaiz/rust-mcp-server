use std::process::Command;

fn main() {
    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    if let Ok(hash) = hash {
        if let Ok(hash_str) = String::from_utf8(hash.stdout) {
            let trimmed_hash = hash_str.trim();
            println!("cargo:rustc-env=GIT_HASH={trimmed_hash}");
        } else {
            eprintln!("Failed to convert git hash output to string");
        }
    } else {
        eprintln!("Failed to execute git command");
    }

    // The `rmcp` dependency version is declared at the workspace level, so walk
    // up from this crate's directory to find the manifest that pins it.
    let version = find_rmcp_version().unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=RMCP_VERSION={version}");
}

fn find_rmcp_version() -> Option<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    let mut dir = std::path::Path::new(&manifest_dir);

    loop {
        let manifest_path = dir.join("Cargo.toml");
        println!("cargo:rerun-if-changed={}", manifest_path.display());
        if let Ok(manifest) = std::fs::read_to_string(&manifest_path)
            && let Some(version) = parse_rmcp_version(&manifest)
        {
            return Some(version);
        }

        dir = dir.parent()?;
    }
}

fn parse_rmcp_version(manifest: &str) -> Option<String> {
    manifest
        .lines()
        .map(str::trim_start)
        .find(|l| l.starts_with("rmcp = { version = \""))
        .and_then(|l| l.strip_prefix("rmcp = { version = \""))
        .and_then(|s| s.split_once('"').map(|(version, _)| version.to_string()))
}
