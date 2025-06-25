# Update Toolset and Fix Rust/Clippy Warnings via MCP

You are an AI agent operating as an MCP (Model Context Protocol) tool integrator for a Rust project. Your task is to ensure the toolset is up-to-date and that the codebase is free of new Rust and Clippy warnings.

## Instructions

1. **Discover MCP tools**
   - Use the MCP protocol to discover and list all available tools.

2. **Update Toolset**
   - Update Rust toolchain using `#rustup-update` tool.

3. **Fix warnings using clippy**
   - Try to fix warnings automatically using `#cargo-clippy` with `fix` flag, be aware that clippy cannot fix all the warnings

3. **Check and Fix Remaining Warnings**
   - Run the following tools via MCP:
     - `#cargo-check` to check for compiler warnings and errors
     - `#cargo-clippy` to lint the codebase for best practices
   - Collect all new warnings and errors reported by the Rust compiler and Clippy linter.
   - Refactor the codebase to resolve these warnings, following Rust best practices and idioms.

4. **Format code**
   - Reformat code using `#cargo-fmt`.

5. **Verification**
   - Run `#cargo-check`, `#cargo-build`, and `#cargo-clippy` to confirm all warnings are resolved.
   - Summarize the actions taken and any remaining issues, if applicable.

**Note:** All actions must be performed via the MCP protocol, using JSON-RPC over stdio, and should comply with the project’s async/await and error handling guidelines.
