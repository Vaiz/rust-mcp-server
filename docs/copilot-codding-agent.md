# Using Rust MCP server with Copilot AI agent

Recently, Github announced a new feature that allows GitHub Copilot to work independently in the background to complete tasks, just like a human developer - [link](https://docs.github.com/en/enterprise-cloud@latest/copilot/concepts/about-assigning-tasks-to-copilot). This expirience can be enchanced by adding Rust MCP server into workflow.


## Setting up repo

### 1. Setup the enviroment

add `.github/workflows/copilot-setup-steps.yml` that pre-installs all required dependencies
more information: [link](https://docs.github.com/en/enterprise-cloud@latest/copilot/how-tos/agents/copilot-coding-agent/extending-copilot-coding-agent-with-mcp#example-azure)

```
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
    - name: Install Rust MCP Server
      run: cargo install --git https://github.com/Vaiz/rust-mcp-server.git --tag stable
    - name: Install machete
      run: cargo install cargo-machete
    - name: Install cargo-deny
      run: cargo install cargo-deny  
```

### 2. Configure MCP server

Go to *Settings* -> *Copilot* -> *Codding agent* and add the next config. You will need to find out the right path where the repo is checked out that can differs from repo to repo

```
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

### 3. Instruct copilot to use Rust MCP server

in `.github/copilot-instructions.md` add something like this

```
## AI Agent Guidelines

### 1. Always Use Rust MCP Tools

- **DO**: Use `Rust-cargo-build` instead of `bash` commands like `cargo build`
- **DO**: Use `Rust-cargo-check` for quick validation
- **DO**: Use `Rust-cargo-clippy` for linting instead of manual clippy commands
- **WHY**: MCP tools provide better defaults, better structured output and better error handling

### 2. Development Workflow

When working on code changes:

1. **Check current state**: Use `Rust-cargo-check` with `all_targets: true, all_features: true`
2. **Make changes**: Edit code using appropriate tools
3. **Validate**: Use `Rust-cargo-clippy` with `workspace: true, all_targets: true`
4. **Format**: Use `Rust-cargo-fmt` with `all: true`
5. **Test**: Use `Rust-cargo-test` with `all_features: true`
6. **Build**: Use `Rust-cargo-build` with `all_targets: true, all_features: true` for final verification
7. **Check unused dependencies**: Use `Rust-cargo-machete` to find unused dependencies
8. **Check security compliance**: Use `Rust-cargo-deny-check` to verify security and compliance

### 3. Dependency Management

- When adding dependencies, prefer workspace-level dependencies in the root `Cargo.toml`

### 4. Code Quality Standards

This project follows strict code quality standards:

- **Clippy**: All clippy warnings must be addressed
- **Formatting**: Code must be formatted with rustfmt using nightly toolchain
- **Tests**: All changes must maintain test coverage
- **Documentation**: Public APIs must be documented
```