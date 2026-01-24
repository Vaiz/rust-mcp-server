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

    let manifest = include_str!(env!("CARGO_MANIFEST_PATH"));
    let version = manifest
        .lines()
        .find(|l| l.starts_with("rmcp = { version = \""))
        .and_then(|l| l.strip_prefix("rmcp = { version = \""))
        .and_then(|s| s.split_once('"').map(|(version, _)| version))
        .unwrap_or("unknown");
    println!("cargo:rustc-env=RMCP_VERSION={version}");
}
