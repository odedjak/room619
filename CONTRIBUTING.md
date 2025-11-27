# Contributing to room619

Thank you for contributing to the Modular Micro-Services Framework! This document provides guidelines for participating in the project.

## Getting Started

### Prerequisites
- Rust 1.70+ (stable)
- Git
- Cargo
- Docker (for testing services locally)

### Setup Development Environment
```bash
git clone https://github.com/odedjak/room619.git
cd room619
git config user.name "Your Name"
git config user.email "your.email@example.com"
cargo build
cargo test
```

## Development Workflow

### 1. Create a Feature Branch
```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes Following Code Standards
- All code must use Rust best practices
- No `.unwrap()` or `.expect()` — use `?` operator
- Async-first design for I/O operations
- Comprehensive error handling with `Result<T, E>`
- Document all public APIs with examples

### 3. Run Local Validation
```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features --verbose

# Check with all features
cargo check-all
```

### 4. Commit with Conventional Messages
```
feat: add user service with gRPC API
fix: handle service connection timeout
docs: document resilience patterns
test: add integration tests for auth middleware
refactor: simplify RPC error handling
perf: optimize service discovery cache
ci: improve GitHub Actions workflows
```

Example detailed message:
```
feat: add user service with gRPC API

- Implement CRUD operations for users
- Add authentication middleware
- Include integration tests
- Add OpenTelemetry tracing support
- Document service contract in proto/user.proto
```

### 5. Push and Create Pull Request
```bash
git push origin feature/your-feature-name
```

Then create a PR with:
- Clear title describing the change
- Description of what/why/how
- Link to related issues
- Mention any breaking changes

## Code Standards

### Rust Best Practices

#### ❌ Forbidden Patterns
```rust
// Never use unwrap/expect
let val = option.unwrap();  // ❌ FORBIDDEN
let val = option.expect("msg");  // ❌ FORBIDDEN

// Never panic in production code
panic!("Something went wrong");  // ❌ FORBIDDEN

// Never suppress warnings
#[allow(clippy::should_implement_trait)]  // ❌ FORBIDDEN
```

#### ✅ Required Patterns
```rust
// Use ? operator for error propagation
pub async fn operation() -> Result<(), Error> {
    let val = operation()?;  // ✅ Proper
    Ok(())
}

// Use Result<T, E> for all fallible operations
pub async fn fetch_user(id: &str) -> Result<User, UserError> {
    // Implementation
}

// Document errors in docstring
/// Fetches a user by ID.
/// 
/// # Errors
/// Returns `UserError::NotFound` if user doesn't exist.
pub async fn get_user(id: &str) -> Result<User, UserError> { }
```

### Documentation Requirements

Every public function and service must document:
- What it does
- Possible errors and how to handle them
- Examples of usage

```rust
/// Sends a request to the user service.
/// 
/// This function handles authentication, retries, and circuit breaking
/// automatically.
/// 
/// # Arguments
/// * `user_id` - The ID of the user to fetch
/// 
/// # Returns
/// Returns the user data or an error if not found or service unavailable.
/// 
/// # Errors
/// Returns `UserServiceError::NotFound` if user doesn't exist.
/// Returns `UserServiceError::ServiceUnavailable` if service is down.
/// 
/// # Example
/// ```
/// let user = client.get_user("user123").await?;
/// println!("User: {}", user.name);
/// ```
pub async fn get_user(&self, user_id: &str) -> Result<User, UserServiceError> {
    // Implementation
}
```

### Async/Await Guidelines

- Use `async fn` for all I/O-bound operations
- Never block async code with synchronous operations
- Use `tokio::spawn` for background tasks
- Always propagate errors with `?`

```rust
// ✅ Good
pub async fn fetch_data(&self, url: &str) -> Result<Data, Error> {
    let response = reqwest::get(url).await?;
    let data = response.json().await?;
    Ok(data)
}

// ❌ Bad (blocking async context)
pub async fn fetch_data(&self, url: &str) -> Result<Data, Error> {
    let data = std::thread::sleep(Duration::from_secs(1));  // NEVER!
    Ok(data)
}
```

## Testing

### Unit Tests
Add tests in the same file as the code:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user() {
        // Test implementation
    }
}
```

### Integration Tests
Add tests in `tests/` directory:
```bash
tests/
├── user_service_integration.rs
├── auth_middleware_integration.rs
└── circuit_breaker_integration.rs
```

### Use Testcontainers for Services
```rust
use testcontainers::*;

#[tokio::test]
async fn test_with_database() {
    let docker = clients::Cli::default();
    let postgres = docker.run(images::postgres::Postgres::default());
    // Test implementation
}
```

### Run All Tests
```bash
cargo test --all-features --verbose
cargo test --all-features --release
```

## Performance & Profiling

### Benchmarking
```bash
cargo bench --all-features
```

### Profiling with Flamegraph
```bash
cargo install flamegraph
cargo flamegraph --bin room619
```

## Security Considerations

### Dependency Review
- All external dependencies must be reviewed
- Use `cargo deny` to check licenses and vulnerabilities

```bash
cargo deny check
```

### Unsafe Code
- Minimize unsafe blocks
- Only use when absolutely necessary
- Always document with `// SAFETY:` comments

```rust
// SAFETY: This is safe because:
// 1. We have exclusive mutable access
// 2. The lifetime is bounded by the function scope
unsafe {
    // Unsafe code here
}
```

## Continuous Integration

All PRs must pass:
- ✅ Cargo check
- ✅ Rustfmt (formatting)
- ✅ Clippy (linting)
- ✅ Unit tests
- ✅ Integration tests
- ✅ Cross-platform builds
- ✅ Docker image build
- ✅ Security audit
- ✅ Code coverage

## Pre-Commit Hooks (Optional)

Set up pre-commit hooks to catch issues early:

```bash
# Create .git/hooks/pre-commit
#!/bin/bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --quiet
```

```bash
chmod +x .git/hooks/pre-commit
```

## Service Development Guidelines

### When Adding a New Service

1. **Define the API Contract**
   - Use Protocol Buffers for gRPC or OpenAPI for REST
   - Document request/response schemas
   - Define error codes

2. **Implement the Service**
   - Implement `Service` trait
   - Add middleware for cross-cutting concerns
   - Include comprehensive error handling

3. **Add Observability**
   - Structured logging at key points
   - Metrics for important operations
   - Distributed tracing integration

4. **Write Tests**
   - Unit tests for business logic
   - Integration tests with dependencies
   - Contract tests for API

5. **Document**
   - Service README
   - API documentation
   - Configuration options
   - Deployment instructions

## Reviewing PRs

When reviewing, ensure:
1. ✅ Code follows Rust best practices
2. ✅ Error handling is comprehensive
3. ✅ All public APIs are documented
4. ✅ Tests are comprehensive
5. ✅ CI passes
6. ✅ Commit messages follow convention
7. ✅ No unnecessary dependencies added

## Reporting Issues

### Security Issues
For security vulnerabilities, **do not** open a public issue. Instead, email the maintainers privately.

### Bugs
Include:
- Rust version (`rustc --version`)
- Platform (Linux/Windows/macOS)
- Minimal reproducible example
- Expected vs actual behavior
- Service logs if applicable

### Feature Requests
Include:
- Use case description
- Why it's needed
- Proposed implementation approach
- Potential impact on performance/latency

## Discussion & Questions

- **Architecture questions**: Start a discussion in GitHub Discussions
- **Design proposals**: Open an issue with the `design` label
- **Quick questions**: Ask in the team chat

## Recognition

Contributors will be recognized in:
- `CONTRIBUTORS.md` file
- GitHub Contributor badge
- Release notes

Thank you for making room619 better! 🚀
