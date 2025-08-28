# API Client

A fast, async Rust CLI client for interacting with REST APIs. This client is designed with modern async programming patterns and provides a clean, type-safe interface for API operations.

## Features

- **Async HTTP Client**: Built with `reqwest` and `tokio` for high-performance async operations
- **CLI Interface**: Command-line interface using `clap` with subcommands for different operations
- **JSON Handling**: Automatic JSON serialization/deserialization with `serde`
- **Colored Output**: Pretty-printed JSON responses with syntax highlighting
- **Error Handling**: Robust error handling with proper error propagation
- **Type Safety**: Leverages Rust's type system for compile-time safety

## Installation

### Prerequisites

- Rust 1.70+ (for edition 2024 support)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd api-client

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

The API client provides a command-line interface with subcommands for different operations. All commands require a base URL as the first argument.

### Command Structure

```bash
api-client <BASE_URL> <COMMAND> [OPTIONS]
```

### Available Commands

#### List Todos
Retrieve all todos from the API:

```bash
api-client http://localhost:8080 list
```

#### Create Todo
Create a new todo item:

```bash
api-client http://localhost:8080 create "Buy groceries"
```

#### Update Todo
Update an existing todo item:

```bash
# Update todo body
api-client http://localhost:8080 update "todo-id" "Updated todo text"

# Mark todo as completed
api-client http://localhost:8080 update "todo-id" "Todo text" --completed

# Update both body and completion status
api-client http://localhost:8080 update "todo-id" "New todo text" --completed
```

#### Delete Todo
Delete a todo item:

```bash
api-client http://localhost:8080 delete "todo-id"
```

### Examples

```bash
# List all todos from a local development server
api-client http://localhost:3000 list

# Create a new todo
api-client https://api.example.com create "Complete project documentation"

# Update a todo and mark it as completed
api-client https://api.example.com update "123" "Updated task description" --completed

# Delete a todo
api-client https://api.example.com delete "123"
```

## API Endpoints

The client expects the following REST API endpoints:

- `GET /v1/todos` - List all todos
- `POST /v1/todos` - Create a new todo
- `PUT /v1/todos/{id}` - Update a todo
- `DELETE /v1/todos/{id}` - Delete a todo

### Request/Response Format

#### Create Todo Request
```json
{
  "body": "Todo description"
}
```

#### Update Todo Request
```json
{
  "body": "Updated todo description",
  "completed": true
}
```

## Development

### Project Structure

```
src/
├── main.rs      # CLI interface and command handling
└── request.rs   # HTTP request logic and response handling
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: Async HTTP client
- **serde/serde_json**: JSON serialization/deserialization
- **tokio**: Async runtime
- **colored_json**: Pretty JSON output with syntax highlighting

### Building and Testing

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run clippy for linting
cargo clippy
```

### Adding New Commands

To add new API operations:

1. Add a new variant to the `Commands` enum in `main.rs`
2. Add the corresponding match arm in the main function
3. Update the request logic in `request.rs` if needed

Example:
```rust
#[derive(Subcommand, Debug)]
enum Commands {
    // ... existing commands ...
    /// Get a specific todo
    Get {
        /// The todo ID
        id: String,
    },
}
```

## Error Handling

The client handles various error scenarios:

- **Network Errors**: Connection failures, timeouts
- **HTTP Errors**: 4xx and 5xx status codes
- **JSON Errors**: Malformed JSON responses
- **Validation Errors**: Invalid input parameters

All errors are properly propagated and displayed to the user.

## Performance

- **Async Operations**: All HTTP requests are non-blocking
- **Connection Reuse**: HTTP client reuses connections for better performance
- **Minimal Dependencies**: Lightweight dependency tree for faster builds
- **Release Optimizations**: Use `--release` flag for production builds

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Use meaningful variable names that convey intent
- Add appropriate error handling with `?` operator
- Write tests for new functionality
- Document public APIs with Rustdoc comments

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Tokio](https://tokio.rs/) for async runtime
- HTTP client powered by [Reqwest](https://github.com/seanmonstar/reqwest)
- CLI framework by [Clap](https://clap.rs/)
- JSON handling with [Serde](https://serde.rs/)
