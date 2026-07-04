# Adding New Tools

Research CLI tool: run `--help`, understand arguments/flags. Tools in `src/tools/`, cargo subcommands in `src/tools/cargo/`.

## File Location
- Cargo subcommand: `src/tools/cargo/your_tool.rs`
- Cargo extension: `src/tools/cargo_[name].rs`
- Other: `src/tools/[tool].rs`

## Implementation

### 1. Create Request Struct

```rust
use crate::{Tool, execute_command, serde_utils::*};
use std::process::Command;

#[derive(Debug, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct YourToolRequest {
    /// Doc comments for each field
    #[serde(default, deserialize_with = "deserialize_string")]
    toolchain: Option<String>,
    
    pub required_param: String,
    
    #[serde(default, deserialize_with = "deserialize_string")]
    optional_string: Option<String>,
    
    #[serde(default, deserialize_with = "deserialize_string_vec")]
    optional_vec: Option<Vec<String>>,
    
    #[serde(default)]
    some_flag: Option<bool>,
    
    #[serde(default, deserialize_with = "deserialize_string")]
    output_verbosity: Option<String>,
    
    #[serde(default, deserialize_with = "deserialize_string")]
    locking_mode: Option<String>,
}

impl YourToolRequest {
    pub fn build_cmd(&self) -> Result<Command, rmcp::ErrorData> {
        let mut cmd = Command::new("your-command");
        
        if let Some(t) = &self.toolchain {
            cmd.arg(format!("+{t}"));
        }
        cmd.arg("subcommand");
        
        cmd.arg(&self.required_param);
        
        if let Some(v) = &self.optional_string {
            cmd.arg("--flag").arg(v);
        }
        
        if let Some(values) = &self.optional_vec {
            for v in values {
                cmd.arg("--multi").arg(v);
            }
        }
        
        if self.some_flag.unwrap_or(false) {
            cmd.arg("--some-flag");
        }
        
        cmd.args(output_verbosity_to_cli_flags(self.output_verbosity.as_deref())?);
        cmd.args(locking_mode_to_cli_flags(self.locking_mode.as_deref(), "locked")?);
        
        Ok(cmd)
    }
}
```

### 2. Implement Tool

```rust
pub struct YourToolRmcpTool;

impl Tool for YourToolRmcpTool {
    const NAME: &'static str = "tool-name";
    const TITLE: &'static str = "Short Title";
    const DESCRIPTION: &'static str = "Clear description";
    type RequestArgs = YourToolRequest;

    fn call_rmcp_tool(&self, req: Self::RequestArgs) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        execute_command(req.build_cmd()?, Self::NAME).map(Into::into)
    }
}
```

### 3. Export Module

**Cargo tools** in `src/tools/cargo/mod.rs`:
```rust
mod your_tool;
pub use your_tool::YourToolRmcpTool;
```

**Standalone** in `src/tools/mod.rs`:
```rust
pub mod your_tool;
```

### 4. Register in Server

Add to `src/rmcp_server.rs`:
```rust
use crate::tools::cargo::YourToolRmcpTool; // or appropriate path

// In Server::new():
tools.insert(YourToolRmcpTool::NAME, Box::new(YourToolRmcpTool));
```

## Patterns

**Serde deserializers** (from `crate::serde_utils`):
- `deserialize_string` - Optional<String>
- `deserialize_string_vec` - Optional<Vec<String>>
- `locking_mode_to_cli_flags(mode, default)` - Converts "locked"/"unlocked"/"offline"/"frozen" to flags
- `output_verbosity_to_cli_flags(level)` - Converts "quiet"/"normal"/"verbose" to flags

**Common fields**:
```rust
#[serde(default, deserialize_with = "deserialize_string")]
toolchain: Option<String>,

#[serde(default, deserialize_with = "deserialize_string")]
locking_mode: Option<String>, // locked/unlocked/offline/frozen

#[serde(default, deserialize_with = "deserialize_string")]
output_verbosity: Option<String>, // quiet/normal/verbose
```

## Verification

Run: `#cargo-check`, `#cargo-clippy`, `#cargo-fmt`
