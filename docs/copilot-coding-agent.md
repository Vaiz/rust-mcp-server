# Using Rust MCP Server with GitHub Copilot Coding Agent

GitHub recently announced a powerful new feature that allows GitHub Copilot to work autonomously in the background to complete development tasks, functioning like an independent developer. This experience can be significantly enhanced by integrating the Rust MCP Server into your workflow.

For more information about GitHub Copilot's coding agent capabilities, see the [official documentation](https://docs.github.com/en/enterprise-cloud@latest/copilot/concepts/about-assigning-tasks-to-copilot).

## Repository Setup Guide

### 1. Environment Setup

Create a `.github/workflows/copilot-setup-steps.yml` file to pre-install all required dependencies. This ensures the Copilot agent has access to the necessary tools.

For additional configuration details, refer to the [MCP extension documentation](https://docs.github.com/en/enterprise-cloud@latest/copilot/how-tos/agents/copilot-coding-agent/extending-copilot-coding-agent-with-mcp#example-azure).

```yaml
on:
  workflow_dispatch:

permissions:
  id-token: write
  contents: write

jobs:
  copilot-setup-steps:
    runs-on: ubuntu-latest
    permissions:
      id-token: write
      contents: write
    environment: copilot
    
    steps:
    - name: Install nightly rustfmt
      run: rustup component add --toolchain nightly rustfmt
      
    - name: Cache Cargo dependencies
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
        key: copilot-cargo
          
    - name: Install cargo-quickinstall
      run: cargo install cargo-quickinstall    
    
    - name: Install Rust MCP Server
      run: cargo install --git https://github.com/Vaiz/rust-mcp-server.git --tag stable
      
    - name: Install cargo-machete
      run: cargo quickinstall cargo-machete
      
    - name: Install cargo-deny
      run: cargo quickinstall cargo-deny

    - name: Install cargo-hack
      run: cargo quickinstall cargo-hack

```

### 2. MCP Server Configuration

Navigate to **Settings** → **Copilot** → **Coding Agent** and add the following configuration. 
> **Note**: You'll need to determine the correct path where your repository is checked out, as this varies between repositories. Usually, the location is `/home/runner/work/{repo_name}/{repo_name}/` (note that `{repo_name}` is repeated twice).

```json
{
  "mcpServers": {
    "Rust": {
      "command": "rust-mcp-server",
      "args": ["--workspace", "path/to/root/of/your/repo"],
      "tools": ["*"],
      "type": "local"
    }
  }
}
```

### 3. Copilot Instructions Configuration

Create or update `.github/copilot-instructions.md` with the following guidelines to ensure Copilot uses the Rust MCP Server effectively:

```markdown
## AI Agent Guidelines

### 1. Always Use Rust MCP Tools

- **DO**: Use `Rust-cargo-build` instead of direct `bash` commands like `cargo build`
- **DO**: Use `Rust-cargo-check` for quick code validation
- **DO**: Use `Rust-cargo-clippy` for linting instead of manual clippy commands
- **WHY**: MCP tools provide better defaults, structured output, and superior error handling

### 2. Development Workflow

Follow this systematic approach when working on code changes:

1. **Check current state**: Use `Rust-cargo-check` with `all_targets: true, all_features: true`
2. **Make changes**: Edit code using appropriate development tools
3. **Validate**: Use `Rust-cargo-clippy` with `workspace: true, all_targets: true`
4. **Format**: Use `Rust-cargo-fmt` with `all: true`
5. **Test**: Use `Rust-cargo-test` with `all_features: true`
6. **Build**: Use `Rust-cargo-build` with `all_targets: true, all_features: true` for final verification
7. **Check unused dependencies**: Use `Rust-cargo-machete` to identify unused dependencies
8. **Verify security compliance**: Use `Rust-cargo-deny-check` to ensure security and licensing compliance

### 3. Dependency Management

- When adding dependencies, prefer workspace-level dependencies in the root `Cargo.toml`
- Use `Rust-cargo-add` and `Rust-cargo-remove` for dependency management
- Regularly run `Rust-cargo-update` to keep dependencies current

### 4. Code Quality Standards

This project maintains strict code quality standards:

- **Clippy**: All clippy warnings must be resolved
- **Formatting**: Code must be formatted with rustfmt using the nightly toolchain
- **Tests**: All changes must maintain or improve test coverage
- **Documentation**: Public APIs must be thoroughly documented
- **Security**: All dependencies must pass security and licensing checks
```

## Additional Resources

- [Rust MCP Server Repository](https://github.com/Vaiz/rust-mcp-server)
- [GitHub Copilot Coding Agent Documentation](https://docs.github.com/en/enterprise-cloud@latest/copilot/concepts/about-assigning-tasks-to-copilot)
- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
