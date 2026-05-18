# Tauri Command Implementation Skill

## Description
Specialized skill for implementing Tauri commands in LingoMate backend. Helps create secure, type-safe Rust functions that expose functionality to the React frontend.

## When to Use
- Adding new backend functionality accessible from frontend
- Creating database operations
- Implementing system integrations (Ollama, TTS, STT)
- Handling file operations
- Managing application state

## Command Structure Template

```rust
// src-tauri/src/commands/[module_name].rs

use tauri::{AppHandle, Manager};
use crate::{
    errors::{AppError, CommandResult},
    models::*,
    services::*,
};

/// [Brief description of what this command does]
/// 
/// # Arguments
/// * `app_handle` - Tauri application handle
/// * `[param_name]` - [Description of parameter]
/// 
/// # Returns
/// * `CommandResult<[ReturnType]>` - Success or error
/// 
/// # Errors
/// * `[ErrorType]` - [When this error occurs]
/// 
/// # Example
/// ```typescript
/// // Frontend usage
/// const result = await invoke('[command_name]', { param: value });
/// ```
#[tauri::command]
pub async fn command_name(
    app_handle: AppHandle,
    param_name: String,
) -> CommandResult<ReturnType> {
    // 1. Validate inputs
    if param_name.is_empty() {
        return Err(AppError::InvalidParameter {
            field: "param_name".to_string(),
            reason: "Parameter cannot be empty".to_string(),
        });
    }
    
    // 2. Get database connection
    let db = app_handle.state::<Database>();
    
    // 3. Execute business logic
    let result = perform_operation(&db, param_name).await?;
    
    // 4. Emit events if needed
    app_handle.emit_all("event_name", EventData { /* ... */ })?;
    
    // 5. Return result
    Ok(result)
}
```

## Step-by-Step Implementation

### Step 1: Define Error Types
Check if appropriate error variants exist in `src-tauri/src/errors.rs`:

```rust
#[derive(Error, Debug, Serialize)]
pub enum AppError {
    // Add new error variant if needed
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}
```

### Step 2: Create Data Models
Define request/response types in `src-tauri/src/models/`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub field1: String,
    pub field2: i32,
}

#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub data: Option<String>,
}
```

### Step 3: Implement Service Logic
Create or update service in `src-tauri/src/services/`:

```rust
use rusqlite::Connection;
use crate::errors::AppError;

pub async fn perform_operation(
    db: &Connection,
    param: String,
) -> Result<String, AppError> {
    // Database operation with parameterized query
    let result = db.query_row(
        "SELECT name FROM table WHERE id = ?",
        rusqlite::params![param],
        |row| row.get::<_, String>(0)
    ).map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(result)
}
```

### Step 4: Register Command
Add to `src-tauri/src/main.rs`:

```rust
use commands::module_name::command_name;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Existing commands...
            command_name,  // Add new command here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 5: Create TypeScript Interface
In `src/types/commands.ts`:

```typescript
export interface CommandRequest {
  field1: string;
  field2: number;
}

export interface CommandResponse {
  success: boolean;
  data?: string;
}

// Add to Commands type union
export type Commands = {
  // ...existing commands
  command_name: (params: CommandRequest) => Promise<CommandResponse>;
};
```

### Step 6: Write Tests
Create test file `src-tauri/tests/command_name_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_command_success() {
        // Setup mock database
        let db = setup_test_db();
        
        // Execute command
        let result = perform_operation(&db, "test".to_string()).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_command_error() {
        let db = setup_test_db();
        let result = perform_operation(&db, "".to_string()).await;
        assert!(result.is_err());
    }
}
```

## Best Practices

### ✅ DO
- Always validate input parameters
- Use parameterized SQL queries
- Handle all error cases explicitly
- Add comprehensive doc comments
- Emit events for state changes
- Log important operations
- Write unit tests

### ❌ DON'T
- Never use `unwrap()` in production code
- Don't concatenate SQL strings
- Don't expose sensitive data in errors
- Avoid blocking operations in async functions
- Don't skip input validation
- Never hardcode paths or credentials

## Common Patterns

### Database Query Pattern
```rust
let items = db.prepare("SELECT * FROM table WHERE field = ?")?
    .query_map(rusqlite::params![value], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
```

### Event Emission Pattern
```rust
app_handle.emit_all("data_updated", Payload {
    id: item_id,
    action: "created".to_string(),
})?;
```

### Async Operation Pattern
```rust
let result = tokio::spawn(async move {
    // Long-running operation
    heavy_computation(data).await
}).await??;
```

## Error Handling Examples

### Validation Error
```rust
if input.len() > MAX_LENGTH {
    return Err(AppError::InvalidParameter {
        field: "input".to_string(),
        reason: format!("Input exceeds maximum length of {}", MAX_LENGTH),
    });
}
```

### Database Error
```rust
db.execute("INSERT INTO table ...", params!)?
    .map_err(|e| AppError::DatabaseError(format!("Insert failed: {}", e)))?;
```

### External Service Error
```rust
let response = reqwest::get(url).await
    .map_err(|e| AppError::NetworkError(e.to_string()))?;
```

## Testing Checklist
- [ ] Test successful execution
- [ ] Test invalid inputs
- [ ] Test database errors
- [ ] Test network timeouts
- [ ] Test concurrent access
- [ ] Test memory usage
- [ ] Verify error messages are user-friendly

## Security Checklist
- [ ] Input validation implemented
- [ ] SQL injection prevented
- [ ] No sensitive data logged
- [ ] Permissions checked
- [ ] Path traversal prevented
- [ ] Rate limiting considered

## Performance Tips
- Use connection pooling for database
- Cache frequently accessed data
- Use async I/O for external calls
- Minimize data serialization
- Batch operations when possible
- Monitor command execution time

## Debugging Tips
- Use `log::debug!()` for detailed logging
- Check Tauri dev console for frontend errors
- Use `cargo test -- --nocapture` for test output
- Profile with `cargo flamegraph` for performance
- Monitor with `tokio-console` for async issues

## References
- [Tauri Commands Documentation](https://tauri.app/v1/guides/features/command/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Project API Specification](../../doc/LingoMate-API接口规范.md)
