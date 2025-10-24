# Contributing to IFM-Ruta

Thank you for your interest in contributing to IFM-Ruta! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70+ (stable, beta, or nightly)
- Git
- A code editor (VS Code, Cursor, or similar)

### Getting Started

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/your-username/ifm-ruta.git
   cd ifm-ruta
   ```

2. **Set up the development environment**:
   ```bash
   # Install dependencies
   cargo build
   
   # Run tests
   cargo test --workspace
   
   # Check formatting
   cargo fmt --all -- --check
   
   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Guidelines

### Code Style

- Follow Rust naming conventions (snake_case for variables, PascalCase for types)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Write self-documenting code with minimal comments
- Keep functions small and focused on single responsibility

### Testing

- Write tests for all new functionality
- Follow TDD (Test-Driven Development) principles
- Ensure all tests pass before submitting PR
- Add integration tests for MCP protocol changes

### Documentation

- Update README.md for user-facing changes
- Document public APIs with rustdoc comments
- Update examples and usage instructions
- Keep documentation concise and clear

### Architecture

- Follow the established trait-based architecture
- Keep core system minimal and focused
- Use dependency injection where appropriate
- Maintain clear separation of concerns

## Project Structure

```
ifm-ruta/
├── core/                   # Core system (traits, models, services)
├── mcp/                    # MCP server implementation
├── egui/                   # egui GUI application
├── docs/                   # Documentation
├── .github/workflows/      # GitHub Actions
└── tests/                  # Integration tests
```

## Pull Request Process

1. **Create a feature branch** from `develop`
2. **Make your changes** following the guidelines above
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run the full test suite**:
   ```bash
   cargo test --workspace
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```
6. **Commit your changes** with clear commit messages
7. **Push to your fork** and create a pull request
8. **Request review** from maintainers

### Commit Message Format

Use clear, descriptive commit messages:

```
feat: add conversation history storage
fix: resolve JSON-RPC parsing issue
docs: update README with new features
test: add integration tests for MCP protocol
refactor: improve error handling in storage service
```

### Pull Request Template

When creating a pull request, please include:

- **Description**: What changes were made and why
- **Type**: Bug fix, feature, documentation, refactoring
- **Testing**: How the changes were tested
- **Breaking Changes**: Any breaking changes and migration steps
- **Screenshots**: For UI changes
- **Checklist**: Ensure all items are completed

## Issue Reporting

When reporting issues, please include:

- **Description**: Clear description of the problem
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Environment**: OS, Rust version, etc.
- **Logs**: Relevant error messages or logs

## Feature Requests

For feature requests, please:

- Check existing issues first
- Provide a clear description of the feature
- Explain the use case and benefits
- Consider implementation complexity
- Be open to discussion and feedback

## Code Review Process

- All code must be reviewed before merging
- Reviewers will check for:
  - Code quality and style
  - Test coverage
  - Documentation updates
  - Performance implications
  - Security considerations
- Address review feedback promptly
- Be respectful and constructive in discussions

## Release Process

- Releases are created from the `master` branch
- Version numbers follow semantic versioning
- Release notes are automatically generated
- Binaries are built for Windows, macOS, and Linux

## Community Guidelines

- Be respectful and inclusive
- Help others learn and grow
- Provide constructive feedback
- Follow the code of conduct
- Welcome newcomers and help them get started

## Getting Help

- Check the documentation first
- Search existing issues
- Ask questions in discussions
- Join the community chat (if available)

## License

By contributing to IFM-Ruta, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to IFM-Ruta! 🚀
