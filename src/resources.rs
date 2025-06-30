use phf::{Map, phf_map};
use rust_mcp_sdk::schema::{
    ListResourcesRequest, ListResourcesResult, ReadResourceRequest, ReadResourceResult, Resource,
    RpcError, TextResourceContents,
};
use tracing::{error, info, warn};

const CARGO_BOOK_BASE_URL: &str =
    "https://raw.githubusercontent.com/rust-lang/cargo/refs/tags/0.89.0/src/doc/src";

static CARGO_BOOK_PAGES: Map<&'static str, &'static str> = phf_map! {
    "cargo-book://index.md" => "Introduction to Cargo",
    "cargo-book://getting-started/index.md" => "Getting Started with Cargo",
    "cargo-book://getting-started/installation.md" => "Installing Cargo",
    "cargo-book://getting-started/first-steps.md" => "First Steps with Cargo",
    "cargo-book://guide/index.md" => "Cargo Guide",
    "cargo-book://guide/why-cargo-exists.md" => "Why Cargo Exists",
    "cargo-book://guide/creating-a-new-project.md" => "Creating a New Package",
    "cargo-book://guide/working-on-an-existing-project.md" => "Working on an Existing Package",
    "cargo-book://guide/dependencies.md" => "Dependencies",
    "cargo-book://guide/project-layout.md" => "Package Layout",
    "cargo-book://guide/cargo-toml-vs-cargo-lock.md" => "Cargo.toml vs Cargo.lock",
    "cargo-book://guide/tests.md" => "Tests",
    "cargo-book://guide/continuous-integration.md" => "Continuous Integration",
    "cargo-book://guide/cargo-home.md" => "Cargo Home",
    "cargo-book://reference/index.md" => "Cargo Reference",
    "cargo-book://reference/manifest.md" => "The Manifest Format",
    "cargo-book://reference/cargo-targets.md" => "Cargo Targets",
    "cargo-book://reference/rust-version.md" => "Rust Version",
    "cargo-book://reference/workspaces.md" => "Workspaces",
    "cargo-book://reference/specifying-dependencies.md" => "Specifying Dependencies",
    "cargo-book://reference/overriding-dependencies.md" => "Overriding Dependencies",
    "cargo-book://reference/source-replacement.md" => "Source Replacement",
    "cargo-book://reference/resolver.md" => "Dependency Resolution",
    "cargo-book://reference/features.md" => "Features",
    "cargo-book://reference/features-examples.md" => "Features Examples",
    "cargo-book://reference/profiles.md" => "Profiles",
    "cargo-book://reference/config.md" => "Configuration",
    "cargo-book://reference/environment-variables.md" => "Environment Variables",
    "cargo-book://reference/build-scripts.md" => "Build Scripts",
    "cargo-book://reference/build-script-examples.md" => "Build Script Examples",
    "cargo-book://reference/build-cache.md" => "Build Cache",
    "cargo-book://reference/pkgid-spec.md" => "Package ID Specifications",
    "cargo-book://reference/external-tools.md" => "External Tools",
    "cargo-book://reference/registries.md" => "Registries",
    "cargo-book://reference/registry-authentication.md" => "Registry Authentication",
    "cargo-book://reference/credential-provider-protocol.md" => "Credential Provider Protocol",
    "cargo-book://reference/running-a-registry.md" => "Running a Registry",
    "cargo-book://reference/registry-index.md" => "Registry Index",
    "cargo-book://reference/registry-web-api.md" => "Registry Web API",
    "cargo-book://reference/semver.md" => "SemVer Compatibility",
    "cargo-book://reference/future-incompat-report.md" => "Future incompat report",
    "cargo-book://reference/timings.md" => "Reporting build timings",
    "cargo-book://reference/lints.md" => "Lints",
    "cargo-book://reference/unstable.md" => "Unstable Features",
    "cargo-book://reference/publishing.md" => "Publishing on crates.io",
    "cargo-book://commands/index.md" => "Cargo Commands",
    "cargo-book://commands/general-commands.md" => "General Commands",
    "cargo-book://commands/cargo.md" => "cargo",
    "cargo-book://commands/cargo-help.md" => "cargo help",
    "cargo-book://commands/cargo-version.md" => "cargo version",
    "cargo-book://commands/build-commands.md" => "Build Commands",
    "cargo-book://commands/cargo-bench.md" => "cargo bench",
    "cargo-book://commands/cargo-build.md" => "cargo build",
    "cargo-book://commands/cargo-check.md" => "cargo check",
    "cargo-book://commands/cargo-clean.md" => "cargo clean",
    "cargo-book://commands/cargo-clippy.md" => "cargo clippy",
    "cargo-book://commands/cargo-doc.md" => "cargo doc",
    "cargo-book://commands/cargo-fetch.md" => "cargo fetch",
    "cargo-book://commands/cargo-fix.md" => "cargo fix",
    "cargo-book://commands/cargo-fmt.md" => "cargo fmt",
    "cargo-book://commands/cargo-miri.md" => "cargo miri",
    "cargo-book://commands/cargo-report.md" => "cargo report",
    "cargo-book://commands/cargo-run.md" => "cargo run",
    "cargo-book://commands/cargo-rustc.md" => "cargo rustc",
    "cargo-book://commands/cargo-rustdoc.md" => "cargo rustdoc",
    "cargo-book://commands/cargo-test.md" => "cargo test",
    "cargo-book://commands/manifest-commands.md" => "Manifest Commands",
    "cargo-book://commands/cargo-add.md" => "cargo add",
    "cargo-book://commands/cargo-generate-lockfile.md" => "cargo generate-lockfile",
    "cargo-book://commands/cargo-info.md" => "cargo info",
    "cargo-book://commands/cargo-locate-project.md" => "cargo locate-project",
    "cargo-book://commands/cargo-metadata.md" => "cargo metadata",
    "cargo-book://commands/cargo-pkgid.md" => "cargo pkgid",
    "cargo-book://commands/cargo-remove.md" => "cargo remove",
    "cargo-book://commands/cargo-tree.md" => "cargo tree",
    "cargo-book://commands/cargo-update.md" => "cargo update",
    "cargo-book://commands/cargo-vendor.md" => "cargo vendor",
    "cargo-book://commands/package-commands.md" => "Package Commands",
    "cargo-book://commands/cargo-init.md" => "cargo init",
    "cargo-book://commands/cargo-install.md" => "cargo install",
    "cargo-book://commands/cargo-new.md" => "cargo new",
    "cargo-book://commands/cargo-search.md" => "cargo search",
    "cargo-book://commands/cargo-uninstall.md" => "cargo uninstall",
    "cargo-book://commands/publishing-commands.md" => "Publishing Commands",
    "cargo-book://commands/cargo-login.md" => "cargo login",
    "cargo-book://commands/cargo-logout.md" => "cargo logout",
    "cargo-book://commands/cargo-owner.md" => "cargo owner",
    "cargo-book://commands/cargo-package.md" => "cargo package",
    "cargo-book://commands/cargo-publish.md" => "cargo publish",
    "cargo-book://commands/cargo-yank.md" => "cargo yank",
    "cargo-book://commands/deprecated-and-removed.md" => "Deprecated and Removed Commands",
    "cargo-book://faq.md" => "FAQ",
    "cargo-book://CHANGELOG.md" => "Changelog",
    "cargo-book://appendix/glossary.md" => "Appendix: Glossary",
    "cargo-book://appendix/git-authentication.md" => "Appendix: Git Authentication",
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

        for (uri, description) in &CARGO_BOOK_PAGES {
            resources.push(Resource {
                uri: uri.to_string(),
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

        if let Some(stripped) = uri.strip_prefix("cargo-book://") {
            self.read_cargo_book_page(stripped).await
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
                                        uri: format!("cargo-book://{page_path}"),
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
