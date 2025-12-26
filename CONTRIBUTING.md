# Contributing to route-ratelimit

Thank you for your interest in contributing to route-ratelimit!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/route-ratelimit.git`
3. Create a branch: `git checkout -b my-feature`

## Development

### Prerequisites

- Rust 1.85.0 or later
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo fmt --check
cargo clippy --all-features -- -D warnings
```

## Pull Request Process

1. Ensure all tests pass and there are no clippy warnings
2. Update documentation if you're changing public APIs
3. Add a changelog entry if the change is user-facing
4. Submit your pull request

## Code Style

- Follow the existing code style
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
