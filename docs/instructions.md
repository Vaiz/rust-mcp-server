# Model Context Protocol (MCP) Rust Server Instructions

This server implements the Model Context Protocol (MCP) for Rust projects, providing a set of tools and resources to help LLMs and clients interact with Rust codebases. The server communicates using JSON-RPC over stdio and supports asynchronous operations via the Tokio runtime.

## Example Scenarios

### 1. Verifying Rust Code Without Making Changes
Steps:
1. Run `cargo-build` to ensure the project builds.
2. Run `cargo-clippy` (without the `fix` flag) to check for code issues.
3. Run `cargo-fmt` with the `check` flag to verify formatting.
4. Run `cargo-machete` to check for unused dependencies.
5. Run `cargo-deny` to check for security and license issues.

### 2. Fixing Various Code Issues (Commit Your Code First)
Steps:
1. Run `cargo-check` to ensure code compiles.
2. Run `cargo-fmt` to fix formatting issues.
3. Run `cargo-clippy` with the `fix` flag to automatically fix code issues.
4. Run `cargo-machete` with the `fix` flag to remove unused dependencies.

### 3. Verifying Rust Code After Changes
Steps:
1. Run `cargo-fmt` to fix formatting issues.
2. Run `cargo-check` and `cargo-build` to ensure code compiles.
3. Run `cargo-clippy` (without the `fix` flag) to check for code issues.
4. Run `cargo-machete` to check for unused dependencies.
5. Run `cargo-deny` to check for license and security issues.

### 4. Adding a New Dependency Using `cargo-add`
Steps:
1. Use `cargo-add <crate-name>` to add a new dependency to your `Cargo.toml`.
2. Run `cargo-build` to ensure the project builds with the new dependency.

### 5. Loading Crate Metadata Using `cargo-metadata`
Steps:
1. Run `cargo-metadata` to retrieve detailed information about the project's dependency graph, workspace members, and crate metadata.
2. Use the output to analyze dependencies, resolve workspace structure, or integrate with other tools that require project metadata.