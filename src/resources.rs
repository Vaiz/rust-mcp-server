use phf::{Map, phf_map};
use rust_mcp_sdk::schema::{
    ListResourcesRequest, ListResourcesResult, ReadResourceRequest, ReadResourceResult, Resource,
    RpcError, TextResourceContents,
};
use tracing::{error, info, warn};

const CARGO_BOOK_BASE_URL: &str =
    "https://raw.githubusercontent.com/rust-lang/cargo/refs/tags/0.89.0/src/doc/src";
const CARGO_BOOK_SCHEME: &str = "cargo-book://";

static CARGO_BOOK_PAGES: Map<&'static str, &'static str> = phf_map! {
    "index.md" => "Introduction to Cargo",
    "getting-started/index.md" => "Getting Started with Cargo",
    "getting-started/installation.md" => "Installing Cargo",
    "getting-started/first-steps.md" => "First Steps with Cargo",
    "guide/index.md" => "Cargo Guide",
    "guide/why-cargo-exists.md" => "Why Cargo Exists",
    "guide/creating-a-new-project.md" => "Creating a New Package",
    "guide/working-on-an-existing-project.md" => "Working on an Existing Package",
    "guide/dependencies.md" => "Dependencies",
    "guide/project-layout.md" => "Package Layout",
    "guide/cargo-toml-vs-cargo-lock.md" => "Cargo.toml vs Cargo.lock",
    "guide/tests.md" => "Tests",
    "guide/continuous-integration.md" => "Continuous Integration",
    "guide/cargo-home.md" => "Cargo Home",
    "reference/index.md" => "Cargo Reference",
    "reference/manifest.md" => "The Manifest Format",
    "reference/cargo-targets.md" => "Cargo Targets",
    "reference/rust-version.md" => "Rust Version",
    "reference/workspaces.md" => "Workspaces",
    "reference/specifying-dependencies.md" => "Specifying Dependencies",
    "reference/overriding-dependencies.md" => "Overriding Dependencies",
    "reference/source-replacement.md" => "Source Replacement",
    "reference/resolver.md" => "Dependency Resolution",
    "reference/features.md" => "Features",
    "reference/features-examples.md" => "Features Examples",
    "reference/profiles.md" => "Profiles",
    "reference/config.md" => "Configuration",
    "reference/environment-variables.md" => "Environment Variables",
    "reference/build-scripts.md" => "Build Scripts",
    "reference/build-script-examples.md" => "Build Script Examples",
    "reference/build-cache.md" => "Build Cache",
    "reference/pkgid-spec.md" => "Package ID Specifications",
    "reference/external-tools.md" => "External Tools",
    "reference/registries.md" => "Registries",
    "reference/registry-authentication.md" => "Registry Authentication",
    "reference/credential-provider-protocol.md" => "Credential Provider Protocol",
    "reference/running-a-registry.md" => "Running a Registry",
    "reference/registry-index.md" => "Registry Index",
    "reference/registry-web-api.md" => "Registry Web API",
    "reference/semver.md" => "SemVer Compatibility",
    "reference/future-incompat-report.md" => "Future incompat report",
    "reference/timings.md" => "Reporting build timings",
    "reference/lints.md" => "Lints",
    "reference/unstable.md" => "Unstable Features",
    "reference/publishing.md" => "Publishing on crates.io",
    "commands/index.md" => "Cargo Commands",
    "commands/general-commands.md" => "General Commands",
    "commands/cargo.md" => "cargo",
    "commands/cargo-help.md" => "cargo help",
    "commands/cargo-version.md" => "cargo version",
    "commands/build-commands.md" => "Build Commands",
    "commands/cargo-bench.md" => "cargo bench",
    "commands/cargo-build.md" => "cargo build",
    "commands/cargo-check.md" => "cargo check",
    "commands/cargo-clean.md" => "cargo clean",
    "commands/cargo-clippy.md" => "cargo clippy",
    "commands/cargo-doc.md" => "cargo doc",
    "commands/cargo-fetch.md" => "cargo fetch",
    "commands/cargo-fix.md" => "cargo fix",
    "commands/cargo-fmt.md" => "cargo fmt",
    "commands/cargo-miri.md" => "cargo miri",
    "commands/cargo-report.md" => "cargo report",
    "commands/cargo-run.md" => "cargo run",
    "commands/cargo-rustc.md" => "cargo rustc",
    "commands/cargo-rustdoc.md" => "cargo rustdoc",
    "commands/cargo-test.md" => "cargo test",
    "commands/manifest-commands.md" => "Manifest Commands",
    "commands/cargo-add.md" => "cargo add",
    "commands/cargo-generate-lockfile.md" => "cargo generate-lockfile",
    "commands/cargo-info.md" => "cargo info",
    "commands/cargo-locate-project.md" => "cargo locate-project",
    "commands/cargo-metadata.md" => "cargo metadata",
    "commands/cargo-pkgid.md" => "cargo pkgid",
    "commands/cargo-remove.md" => "cargo remove",
    "commands/cargo-tree.md" => "cargo tree",
    "commands/cargo-update.md" => "cargo update",
    "commands/cargo-vendor.md" => "cargo vendor",
    "commands/package-commands.md" => "Package Commands",
    "commands/cargo-init.md" => "cargo init",
    "commands/cargo-install.md" => "cargo install",
    "commands/cargo-new.md" => "cargo new",
    "commands/cargo-search.md" => "cargo search",
    "commands/cargo-uninstall.md" => "cargo uninstall",
    "commands/publishing-commands.md" => "Publishing Commands",
    "commands/cargo-login.md" => "cargo login",
    "commands/cargo-logout.md" => "cargo logout",
    "commands/cargo-owner.md" => "cargo owner",
    "commands/cargo-package.md" => "cargo package",
    "commands/cargo-publish.md" => "cargo publish",
    "commands/cargo-yank.md" => "cargo yank",
    "commands/deprecated-and-removed.md" => "Deprecated and Removed Commands",
    "faq.md" => "FAQ",
    "CHANGELOG.md" => "Changelog",
    "appendix/glossary.md" => "Appendix: Glossary",
    "appendix/git-authentication.md" => "Appendix: Git Authentication",
};

pub struct ResourceHandler {
    http_client: reqwest::Client,
}

impl ResourceHandler {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn handle_list_resources_request(
        &self,
        _request: ListResourcesRequest,
    ) -> Result<ListResourcesResult, RpcError> {
        info!("Listing available Cargo Book resources");

        let mut resources = Vec::new();

        for (path, description) in &CARGO_BOOK_PAGES {
            resources.push(Resource {
                uri: format!("{CARGO_BOOK_SCHEME}{path}"),
                name: description.to_string(),
                description: None,
                mime_type: Some("text/markdown".to_string()),
                size: None,
                annotations: None,
            });
        }

        // Sort resources by URI for consistent ordering
        resources.sort_by(|a, b| a.uri.cmp(&b.uri));

        Ok(ListResourcesResult {
            meta: None,
            next_cursor: None,
            resources,
        })
    }

    pub async fn handle_read_resource_request(
        &self,
        request: ReadResourceRequest,
    ) -> Result<ReadResourceResult, RpcError> {
        let uri = &request.params.uri;
        info!(uri = %uri, "Reading resource");

        if let Some(page_path) = uri.strip_prefix(CARGO_BOOK_SCHEME) {
            // Validate that the page path exists in our map
            if CARGO_BOOK_PAGES.contains_key(page_path) {
                self.read_cargo_book_page(page_path).await
            } else {
                warn!(uri = %uri, page_path = %page_path, "Unknown Cargo Book page path");
                Err(RpcError::invalid_params()
                    .with_message(format!("Unknown Cargo Book page: {page_path}")))
            }
        } else {
            warn!(uri = %uri, "Unknown resource URI scheme");
            Err(RpcError::invalid_params().with_message(format!("Unknown resource URI: {uri}")))
        }
    }

    async fn read_cargo_book_page(&self, page_path: &str) -> Result<ReadResourceResult, RpcError> {
        let url = format!("{CARGO_BOOK_BASE_URL}/{page_path}");
        info!(url = %url, "Fetching Cargo Book page");

        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(content) => {
                            info!(page_path = %page_path, content_length = content.len(), "Successfully fetched Cargo Book page");
                            Ok(ReadResourceResult {
                                contents: vec![
                                    TextResourceContents {
                                        uri: format!("{CARGO_BOOK_SCHEME}{page_path}"),
                                        mime_type: Some("text/markdown".to_string()),
                                        text: content,
                                    }
                                    .into(),
                                ],
                                meta: None,
                            })
                        }
                        Err(e) => {
                            error!(page_path = %page_path, error = %e, "Failed to read response text");
                            Err(RpcError::internal_error()
                                .with_message(format!("Failed to read response: {e}")))
                        }
                    }
                } else {
                    warn!(page_path = %page_path, status = %response.status(), "HTTP request failed");
                    Err(RpcError::internal_error().with_message(format!(
                        "HTTP {} for page: {}",
                        response.status(),
                        page_path
                    )))
                }
            }
            Err(e) => {
                error!(page_path = %page_path, error = %e, "Failed to fetch Cargo Book page");
                Err(RpcError::internal_error().with_message(format!("Failed to fetch page: {e}")))
            }
        }
    }
}
