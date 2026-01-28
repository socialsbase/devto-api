# Contributing to devto-api

Thank you for your interest in contributing to devto-api! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/devto-api.git
   cd devto-api
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## Development

### Project Structure

```
devto-api/
├── src/
│   ├── lib.rs      # Main library entry point
│   └── config.rs   # Client configuration helpers
├── openapi/        # OpenAPI specification
├── examples/       # Usage examples
├── tests/          # Integration tests
└── build.rs        # Code generation from OpenAPI spec
```

### Code Generation

The API client is auto-generated from the OpenAPI specification using [Progenitor](https://github.com/oxidecomputer/progenitor). The generation happens at build time via `build.rs`.

To update the API bindings:
1. Update the OpenAPI spec in `openapi/api_v1.json`
2. Run `cargo build` to regenerate the client

### Running Examples

```bash
# Public access (no API key needed)
cargo run --example basic_usage

# With authentication
DEVTO_API_KEY=your-api-key cargo run --example basic_usage
```

## Making Changes

### Code Style

- Follow standard Rust conventions and idioms
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Add documentation for public APIs

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 72 characters

### Pull Requests

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes and commit them
3. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
4. Open a pull request against the `main` branch

### PR Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Ensure all tests pass
- Update documentation if needed

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

## Reporting Issues

When reporting issues, please include:

- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- Rust version (`rustc --version`)
- Operating system

## Questions

If you have questions, feel free to open an issue for discussion.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
