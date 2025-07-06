# Merging Multiple CLI Flags into a Single Parameter for MCP Tools

## Context

When designing MCP (Model Context Protocol) tools, it's often better to consolidate multiple related boolean flags into a single string parameter. MCP doesn't work well with enums, so using string parameters with validation provides better compatibility and user experience.

## Problem

Consider a scenario where you have multiple boolean flags that control related behavior:

```rust
/// Assert that `Cargo.lock` will remain unchanged
#[serde(default)]
locked: bool,

/// Run without accessing the network  
#[serde(default)]
offline: bool,

/// Equivalent to specifying both --locked and --offline
#[serde(default)]
frozen: bool,
```

This approach has several issues:
1. **Mutual exclusivity**: Some combinations don't make sense
2. **Complexity**: Users need to understand multiple flags
3. **MCP limitations**: Enums don't work well with MCP
4. **Redundancy**: Some flags are combinations of others

## Solution Pattern

### 1. Define a Single String Parameter

Replace multiple boolean flags with a single optional string parameter:

```rust
/// Locking mode for dependency resolution.
///
/// Valid options:
/// - "locked" (default): Assert that `Cargo.lock` will remain unchanged
/// - "unlocked": Allow `Cargo.lock` to be updated
/// - "offline": Run without accessing the network
/// - "frozen": Equivalent to specifying both --locked and --offline
#[serde(default, deserialize_with = "deserialize_string")]
locking_mode: Option<String>,
```

### 2. Create a Helper Function

Implement a conversion function that maps string values to CLI flags:

```rust
/// Convert locking mode string to appropriate CLI flags
pub fn locking_mode_to_cli_flags(mode: Option<&str>) -> Result<Vec<&'static str>, CallToolError> {
    Ok(match mode.unwrap_or("locked") {
        "locked" => vec!["--locked"],
        "unlocked" => vec![], // No flags needed
        "offline" => vec!["--offline"], 
        "frozen" => vec!["--frozen"],
        unknown => {
            return Err(CallToolError(
                anyhow::anyhow!(
                    "Unknown locking mode: {unknown}. Valid options are: locked, unlocked, offline, frozen"
                ).into()
            ));
        }
    })
}
```

### 3. Use the Helper in Tool Implementation

Apply the helper function in your tool's `call_tool` method:

```rust
impl CargoInfoTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("cargo");
        cmd.arg("info");
        
        // ... other arguments ...
        
        // Apply locking mode flags
        let locking_flags = locking_mode_to_cli_flags(self.locking_mode.as_deref())?;
        cmd.args(locking_flags);
        
        // ... continue with execution ...
    }
}
```

## Key Design Principles

### 1. Use Optional Strings, Not Enums
```rust
// ✅ Good - Works well with MCP
locking_mode: Option<String>,

// ❌ Avoid - MCP doesn't handle enums well
locking_mode: Option<LockingMode>,
```

### 2. Provide Clear Documentation
- List all valid options in the doc comment
- Explain what each option does
- Specify the default behavior
- Use bullet points for readability

### 3. Implement Proper Error Handling
- Return descriptive error messages for invalid values
- List all valid options in error messages
- Use `Result` types for validation functions

### 4. Choose Sensible Defaults
```rust
// Use unwrap_or() to provide a sensible default
mode.unwrap_or("locked")
```

### 5. Handle Empty Cases Gracefully
```rust
"unlocked" => vec![], // No flags needed - explicit empty vector
```

## Benefits

1. **Simplified API**: Single parameter instead of multiple flags
2. **Better UX**: Clear, self-documenting options
3. **MCP Compatibility**: String parameters work reliably with MCP
4. **Validation**: Catch invalid combinations at runtime
5. **Extensibility**: Easy to add new modes without breaking changes
6. **Documentation**: Options are self-documenting in the schema

## Migration Strategy

When migrating existing tools:

1. **Replace old flags** with the new parameter completely
2. **Update the implementation** to use the helper function
3. **Update tests** to use the new parameter format
4. **Update documentation** and examples

## Example Usage

```json
{
  "locking_mode": "frozen"
}
```

Instead of:
```json
{
  "locked": true,
  "offline": true,
  "frozen": false
}
```

## Testing

Ensure your helper function is well-tested:

```rust
#[test]
fn test_locking_mode_cli_flags() {
    assert_eq!(locking_mode_to_cli_flags(None).unwrap(), vec!["--locked"]);
    assert_eq!(locking_mode_to_cli_flags(Some("locked")).unwrap(), vec!["--locked"]);
    assert_eq!(locking_mode_to_cli_flags(Some("unlocked")).unwrap(), vec![]);
    assert_eq!(locking_mode_to_cli_flags(Some("offline")).unwrap(), vec!["--offline"]);
    assert_eq!(locking_mode_to_cli_flags(Some("frozen")).unwrap(), vec!["--frozen"]);
    
    assert!(locking_mode_to_cli_flags(Some("invalid")).is_err());
}
```

This pattern provides a clean, maintainable, and MCP-friendly way to handle complex flag combinations in your tools.
