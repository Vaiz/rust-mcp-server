# Adding New Tools to RustMCP

This guide provides concise instructions for adding new tools to the RustMCP server.

## Prerequisites 

Research the CLI tool thoroughly: run `--help`, understand all arguments, flags, and usage patterns. Tools are organized in `src/tools/` with cargo subcommands in `src/tools/cargo/`.

## Project Structure Overview

The RustMCP project follows a modular structure for organizing tools:

```
src/
├── tools/
│   ├── mod.rs                 # Main tools module and registry
│   ├── cargo/
│   │   ├── mod.rs            # Cargo tools module
│   │   ├── build.rs          # cargo build tool
│   │   ├── check.rs          # cargo check tool
│   │   ├── update.rs         # cargo update tool (example)
│   │   └── ...               # Other cargo tools
│   ├── cargo_deny.rs         # cargo-deny tools
│   ├── cargo_machete.rs      # cargo-machete tools  
│   └── rustup.rs             # rustup tools
```

## Implementation Steps

### 1. Choose Location & Research
- **Cargo subcommand**: Add to `src/tools/cargo/` 
- **Cargo extension**: Add to `src/tools/cargo_[name].rs`
- **Other tool**: Add to `src/tools/[tool_name].rs`

### 2. Create Tool Implementation

```rust
use std::process::Command;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};
use crate::{
    serde_utils::{default_true, deserialize_string, deserialize_string_vec},
    tools::execute_command,
};

#[mcp_tool(
    name = "tool-name",
    description = "Clear, concise description",
    openWorldHint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct YourToolName {
    /// The toolchain to use (for Rust tools)
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,
    
    /// Required parameters (no #[serde(default)])
    pub required_param: String,
    
    /// Optional parameters with deserializers
    #[serde(default, deserialize_with = "deserialize_string")]
    pub optional_string: Option<String>,
    
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    pub optional_vec: Option<Vec<String>>,
    
    /// Boolean flags (default false)
    #[serde(default)]
    pub some_flag: bool,
    
    /// MCP automation-friendly defaults (quiet=true)
    #[serde(default = "default_true")]
    pub quiet: bool,
}

impl YourToolName {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("your-command");
        
        // Toolchain for Rust tools
        if let Some(toolchain) = &self.toolchain {
            cmd.arg(format!("+{}", toolchain));
        }
        cmd.arg("subcommand");

        // Add parameters systematically:
        // 1. Required parameters
        cmd.arg(&self.required_param);
        
        // 2. Optional parameters
        if let Some(value) = &self.optional_string {
            cmd.arg("--flag").arg(value);
        }
        
        // 3. Vector parameters
        if let Some(values) = &self.optional_vec {
            for value in values {
                cmd.arg("--multi-flag").arg(value);
            }
        }
        
        // 4. Boolean flags
        if self.some_flag {
            cmd.arg("--some-flag");
        }
        if self.quiet {
            cmd.arg("--quiet");
        }

        execute_command(cmd)
    }
}
```

### 3. Export the Tool

**For cargo tools** (`src/tools/cargo/`):
1. Add module in `src/tools/cargo/mod.rs`: `mod your_tool;`
2. Export tool: `pub use your_tool::YourToolName;`

**For standalone tools** (`src/tools/your_tool.rs`):
1. Add module in `src/tools/mod.rs`: `pub mod your_tool;`
2. Import tool: `use your_tool::YourToolName;`

### 4. Register the Tool

Update `src/tools/mod.rs`:

1. **Add to imports**:
```rust
use cargo::{
    // ...existing tools...
    YourToolName,
};
```

2. **Add to tool registry**:
```rust
rust_mcp_sdk::tool_box!(
    AllTools,
    [
        // ...existing tools...
        YourToolName,
        // ...rest of tools...
    ]
);
```

3. **Add match arm**:
```rust
match tool_params {
    // ...existing match arms...
    AllTools::YourToolName(tool) => tool.call_tool(),
    // ...remaining match arms...
}
```

### 5. Build and Test

1. **Build the project using RustMCP tools**:
   - Use the `#cargo-build` tool via MCP protocol to compile the project

2. **Check for compilation errors**:
   - Use the `#cargo-check` tool via MCP protocol to analyze the package and report errors

3. **Run clippy for best practices**:
   - Use the `#cargo-clippy` tool via MCP protocol to check for common mistakes and improve code quality

### 6. Update README.md

Update `## Features` section in `README.md` to include the new tool.


## Best Practices

### MCP-Friendly Defaults
Set automation-friendly defaults (differ from CLI defaults):
```rust
/// Quiet by default for better automation
#[serde(default = "default_true")]
pub quiet: bool,

/// Locked by default to prevent blocking
#[serde(default = "default_true")] 
pub locked: bool,
```

### Documentation & Error Handling
- Clear tool descriptions and parameter documentation
- Use `execute_command` helper for consistent error handling
- Include usage examples in parameter documentation

### Consistency
- Follow existing patterns in the codebase
- Use consistent naming: `ToolName` → `tool-name`
- Group parameters logically (package selection, compilation options, output options)

## Common Patterns

**Simple installation tool**:
```rust
#[mcp_tool(name = "tool-install", description = "Installs the tool")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ToolInstallTool {}

impl ToolInstallTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("install").arg("tool-name");
        execute_command(cmd)
    }
}
```

## Troubleshooting

- **Tool not found**: Verify registration in `AllTools` enum
- **Parameter errors**: Check JSON schema and parameter types
- **Missing imports**: Ensure all required imports are present
- **Command errors**: Verify command and arguments are correct
