use rust_mcp_sdk::schema::{
    ListResourcesRequest, ListResourcesResult, ReadResourceRequest, ReadResourceResult, Resource,
    RpcError, TextResourceContents,
};
use std::collections::HashMap;
use tracing::{error, info, warn};

const CARGO_BOOK_BASE_URL: &str =
    "https://raw.githubusercontent.com/rust-lang/cargo/refs/tags/0.89.0/src/doc/src";

/// Handler for MCP resources, currently supporting the Cargo Book
pub struct ResourceHandler {
    http_client: reqwest::Client,
    cargo_book_pages: HashMap<String, String>, // URI -> description mapping
}

impl ResourceHandler {
    pub fn new() -> Self {
        let mut cargo_book_pages = HashMap::new();

        // Populate the Cargo Book pages based on the SUMMARY.md structure
        Self::populate_cargo_book_pages(&mut cargo_book_pages);

        Self {
            http_client: reqwest::Client::new(),
            cargo_book_pages,
        }
    }

    /// Populate the Cargo Book pages mapping from the SUMMARY.md structure
    fn populate_cargo_book_pages(pages: &mut HashMap<String, String>) {
        // Introduction and Getting Started
        pages.insert(
            "cargo-book://index.md".to_string(),
            "Introduction to Cargo".to_string(),
        );
        pages.insert(
            "cargo-book://getting-started/index.md".to_string(),
            "Getting Started with Cargo".to_string(),
        );
        pages.insert(
            "cargo-book://getting-started/installation.md".to_string(),
            "Installing Cargo".to_string(),
        );
        pages.insert(
            "cargo-book://getting-started/first-steps.md".to_string(),
            "First Steps with Cargo".to_string(),
        );

        // Cargo Guide
        pages.insert(
            "cargo-book://guide/index.md".to_string(),
            "Cargo Guide".to_string(),
        );
        pages.insert(
            "cargo-book://guide/why-cargo-exists.md".to_string(),
            "Why Cargo Exists".to_string(),
        );
        pages.insert(
            "cargo-book://guide/creating-a-new-project.md".to_string(),
            "Creating a New Package".to_string(),
        );
        pages.insert(
            "cargo-book://guide/working-on-an-existing-project.md".to_string(),
            "Working on an Existing Package".to_string(),
        );
        pages.insert(
            "cargo-book://guide/dependencies.md".to_string(),
            "Dependencies".to_string(),
        );
        pages.insert(
            "cargo-book://guide/project-layout.md".to_string(),
            "Package Layout".to_string(),
        );
        pages.insert(
            "cargo-book://guide/cargo-toml-vs-cargo-lock.md".to_string(),
            "Cargo.toml vs Cargo.lock".to_string(),
        );
        pages.insert(
            "cargo-book://guide/tests.md".to_string(),
            "Tests".to_string(),
        );
        pages.insert(
            "cargo-book://guide/continuous-integration.md".to_string(),
            "Continuous Integration".to_string(),
        );
        pages.insert(
            "cargo-book://guide/cargo-home.md".to_string(),
            "Cargo Home".to_string(),
        );

        // Reference
        pages.insert(
            "cargo-book://reference/index.md".to_string(),
            "Cargo Reference".to_string(),
        );
        pages.insert(
            "cargo-book://reference/manifest.md".to_string(),
            "The Manifest Format".to_string(),
        );
        pages.insert(
            "cargo-book://reference/cargo-targets.md".to_string(),
            "Cargo Targets".to_string(),
        );
        pages.insert(
            "cargo-book://reference/rust-version.md".to_string(),
            "Rust Version".to_string(),
        );
        pages.insert(
            "cargo-book://reference/workspaces.md".to_string(),
            "Workspaces".to_string(),
        );
        pages.insert(
            "cargo-book://reference/specifying-dependencies.md".to_string(),
            "Specifying Dependencies".to_string(),
        );
        pages.insert(
            "cargo-book://reference/overriding-dependencies.md".to_string(),
            "Overriding Dependencies".to_string(),
        );
        pages.insert(
            "cargo-book://reference/source-replacement.md".to_string(),
            "Source Replacement".to_string(),
        );
        pages.insert(
            "cargo-book://reference/resolver.md".to_string(),
            "Dependency Resolution".to_string(),
        );
        pages.insert(
            "cargo-book://reference/features.md".to_string(),
            "Features".to_string(),
        );
        pages.insert(
            "cargo-book://reference/features-examples.md".to_string(),
            "Features Examples".to_string(),
        );
        pages.insert(
            "cargo-book://reference/profiles.md".to_string(),
            "Profiles".to_string(),
        );
        pages.insert(
            "cargo-book://reference/config.md".to_string(),
            "Configuration".to_string(),
        );
        pages.insert(
            "cargo-book://reference/environment-variables.md".to_string(),
            "Environment Variables".to_string(),
        );
        pages.insert(
            "cargo-book://reference/build-scripts.md".to_string(),
            "Build Scripts".to_string(),
        );
        pages.insert(
            "cargo-book://reference/build-script-examples.md".to_string(),
            "Build Script Examples".to_string(),
        );
        pages.insert(
            "cargo-book://reference/build-cache.md".to_string(),
            "Build Cache".to_string(),
        );
        pages.insert(
            "cargo-book://reference/pkgid-spec.md".to_string(),
            "Package ID Specifications".to_string(),
        );
        pages.insert(
            "cargo-book://reference/external-tools.md".to_string(),
            "External Tools".to_string(),
        );
        pages.insert(
            "cargo-book://reference/registries.md".to_string(),
            "Registries".to_string(),
        );
        pages.insert(
            "cargo-book://reference/registry-authentication.md".to_string(),
            "Registry Authentication".to_string(),
        );
        pages.insert(
            "cargo-book://reference/credential-provider-protocol.md".to_string(),
            "Credential Provider Protocol".to_string(),
        );
        pages.insert(
            "cargo-book://reference/running-a-registry.md".to_string(),
            "Running a Registry".to_string(),
        );
        pages.insert(
            "cargo-book://reference/registry-index.md".to_string(),
            "Registry Index".to_string(),
        );
        pages.insert(
            "cargo-book://reference/registry-web-api.md".to_string(),
            "Registry Web API".to_string(),
        );
        pages.insert(
            "cargo-book://reference/semver.md".to_string(),
            "SemVer Compatibility".to_string(),
        );
        pages.insert(
            "cargo-book://reference/future-incompat-report.md".to_string(),
            "Future incompat report".to_string(),
        );
        pages.insert(
            "cargo-book://reference/timings.md".to_string(),
            "Reporting build timings".to_string(),
        );
        pages.insert(
            "cargo-book://reference/lints.md".to_string(),
            "Lints".to_string(),
        );
        pages.insert(
            "cargo-book://reference/unstable.md".to_string(),
            "Unstable Features".to_string(),
        );
        pages.insert(
            "cargo-book://reference/publishing.md".to_string(),
            "Publishing on crates.io".to_string(),
        );

        // Commands
        pages.insert(
            "cargo-book://commands/index.md".to_string(),
            "Cargo Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/general-commands.md".to_string(),
            "General Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo.md".to_string(),
            "cargo".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-help.md".to_string(),
            "cargo help".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-version.md".to_string(),
            "cargo version".to_string(),
        );

        // Build Commands
        pages.insert(
            "cargo-book://commands/build-commands.md".to_string(),
            "Build Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-bench.md".to_string(),
            "cargo bench".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-build.md".to_string(),
            "cargo build".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-check.md".to_string(),
            "cargo check".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-clean.md".to_string(),
            "cargo clean".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-clippy.md".to_string(),
            "cargo clippy".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-doc.md".to_string(),
            "cargo doc".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-fetch.md".to_string(),
            "cargo fetch".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-fix.md".to_string(),
            "cargo fix".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-fmt.md".to_string(),
            "cargo fmt".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-miri.md".to_string(),
            "cargo miri".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-report.md".to_string(),
            "cargo report".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-run.md".to_string(),
            "cargo run".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-rustc.md".to_string(),
            "cargo rustc".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-rustdoc.md".to_string(),
            "cargo rustdoc".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-test.md".to_string(),
            "cargo test".to_string(),
        );

        // Manifest Commands
        pages.insert(
            "cargo-book://commands/manifest-commands.md".to_string(),
            "Manifest Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-add.md".to_string(),
            "cargo add".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-generate-lockfile.md".to_string(),
            "cargo generate-lockfile".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-info.md".to_string(),
            "cargo info".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-locate-project.md".to_string(),
            "cargo locate-project".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-metadata.md".to_string(),
            "cargo metadata".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-pkgid.md".to_string(),
            "cargo pkgid".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-remove.md".to_string(),
            "cargo remove".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-tree.md".to_string(),
            "cargo tree".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-update.md".to_string(),
            "cargo update".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-vendor.md".to_string(),
            "cargo vendor".to_string(),
        );

        // Package Commands
        pages.insert(
            "cargo-book://commands/package-commands.md".to_string(),
            "Package Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-init.md".to_string(),
            "cargo init".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-install.md".to_string(),
            "cargo install".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-new.md".to_string(),
            "cargo new".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-search.md".to_string(),
            "cargo search".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-uninstall.md".to_string(),
            "cargo uninstall".to_string(),
        );

        // Publishing Commands
        pages.insert(
            "cargo-book://commands/publishing-commands.md".to_string(),
            "Publishing Commands".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-login.md".to_string(),
            "cargo login".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-logout.md".to_string(),
            "cargo logout".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-owner.md".to_string(),
            "cargo owner".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-package.md".to_string(),
            "cargo package".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-publish.md".to_string(),
            "cargo publish".to_string(),
        );
        pages.insert(
            "cargo-book://commands/cargo-yank.md".to_string(),
            "cargo yank".to_string(),
        );
        pages.insert(
            "cargo-book://commands/deprecated-and-removed.md".to_string(),
            "Deprecated and Removed Commands".to_string(),
        );

        // Additional sections
        pages.insert("cargo-book://faq.md".to_string(), "FAQ".to_string());
        pages.insert(
            "cargo-book://CHANGELOG.md".to_string(),
            "Changelog".to_string(),
        );
        pages.insert(
            "cargo-book://appendix/glossary.md".to_string(),
            "Appendix: Glossary".to_string(),
        );
        pages.insert(
            "cargo-book://appendix/git-authentication.md".to_string(),
            "Appendix: Git Authentication".to_string(),
        );
    }

    /// Handle list resources request
    pub async fn handle_list_resources_request(
        &self,
        _request: ListResourcesRequest,
    ) -> Result<ListResourcesResult, RpcError> {
        info!("Listing available Cargo Book resources");

        let mut resources = Vec::new();

        // Create resources from our cargo book pages
        for (uri, description) in &self.cargo_book_pages {
            resources.push(Resource {
                uri: uri.clone(),
                name: description.clone(),
                description: Some(format!("Cargo Book: {}", description)),
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

    /// Handle read resource request
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
            Err(RpcError::invalid_params().with_message(format!("Unknown resource URI: {}", uri)))
        }
    }

    /// Read a Cargo Book page by fetching it from GitHub
    async fn read_cargo_book_page(&self, page_path: &str) -> Result<ReadResourceResult, RpcError> {
        let url = format!("{}/{}", CARGO_BOOK_BASE_URL, page_path);
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
                                        uri: format!("cargo-book://{}", page_path),
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
                                .with_message(format!("Failed to read response: {}", e)))
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
                Err(RpcError::internal_error().with_message(format!("Failed to fetch page: {}", e)))
            }
        }
    }
}
