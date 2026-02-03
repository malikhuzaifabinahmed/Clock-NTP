# Contributing to Clock-NTP

Thank you for your interest in contributing to Clock-NTP! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/Clock-NTP.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites
- Rust 1.70 or later
- Cargo

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running the Application
```bash
cargo run
```

## Code Quality Standards

### Before Submitting a PR

1. **Format your code**
   ```bash
   cargo fmt
   ```

2. **Run clippy**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Run all tests**
   ```bash
   cargo test
   ```

4. **Check documentation**
   ```bash
   cargo doc --no-deps --open
   ```

### Code Style
- Follow Rust naming conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Write doc comments for public APIs
- Keep functions small and focused

### Testing
- Add unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass before submitting
- Aim for high test coverage

### Documentation
- Update README.md if adding new features
- Add doc comments for public APIs
- Include examples in doc comments where appropriate
- Update CHANGELOG.md (if exists)

## Types of Contributions

### Bug Reports
- Use the GitHub issue tracker
- Include steps to reproduce
- Include expected vs actual behavior
- Include system information (OS, Rust version)

### Feature Requests
- Use the GitHub issue tracker
- Clearly describe the feature
- Explain the use case
- Discuss potential implementation approaches

### Code Contributions
- Fix bugs
- Implement new features
- Improve documentation
- Add tests
- Refactor code

## Pull Request Process

1. **Create a descriptive PR title**
   - Format: `[Type] Brief description`
   - Types: Feature, Fix, Docs, Refactor, Test, Chore

2. **Provide detailed description**
   - What changes were made?
   - Why were these changes needed?
   - How do these changes work?
   - Any breaking changes?

3. **Link related issues**
   - Use "Fixes #123" or "Closes #123"

4. **Request review**
   - Tag relevant maintainers

5. **Address feedback**
   - Respond to review comments
   - Make requested changes
   - Update PR as needed

## Feature Ideas

Some ideas for contributions:
- Configuration file support
- Web UI for monitoring
- Additional time format outputs
- NTPv4 support
- Enhanced error handling
- Performance optimizations
- Cross-platform testing

## Questions?

If you have questions, feel free to:
- Open an issue
- Start a discussion
- Contact the maintainers

Thank you for contributing! 🎉
