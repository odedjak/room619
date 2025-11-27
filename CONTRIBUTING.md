# Contributing to room619

Thank you for contributing to the Modular Real-Time Embedded Framework! This document provides guidelines for participating in the project.

## Getting Started

### Prerequisites
- Rust 1.70+ (stable)
- Git
- Cargo

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
- All code must adhere to **hard real-time Rust** guidelines
- No `.unwrap()` or `.expect()` — use `?` operator
- No unbounded allocations in critical paths
- Document timing constraints with `WCET:` comments

### 3. Run Local Validation
```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features --verbose

# Check real-time constraints
cargo check-all
```

### 4. Commit with Conventional Messages
```
feat: add MQTT telemetry protocol
fix: resolve sensor timeout issue
docs: update architecture guide
test: add integration tests for scheduler
refactor: simplify HAL abstraction
perf: optimize task context switching
ci: improve GitHub Actions workflows
```

Example detailed message:
```
feat: add MQTT telemetry protocol support

- Implement MQTT client with bounded message queue
- Add automatic reconnection with exponential backoff
- Verify hard real-time constraints (≤50µs send latency)
- Include integration tests and documentation

WCET: O(1), ≤ 50µs
Allocation: None in hot path
```

### 5. Push and Create Pull Request
```bash
git push origin feature/your-feature-name
```

Then create a PR with:
- Clear title describing the change
- Description of what/why/how
- Link to related issues
- WCET guarantees for new functions

## Code Standards

### Hard Real-Time Requirements

#### ❌ Forbidden in Timing-Critical Code
```rust
// Never use unwrap/expect
let val = option.unwrap();  // ❌ FORBIDDEN
let val = option.expect("msg");  // ❌ FORBIDDEN

// Never allocate
let vec = vec![1, 2, 3];  // ❌ FORBIDDEN in hot paths
let map = HashMap::new();  // ❌ FORBIDDEN

// Never use synchronization primitives
let lock = Mutex::new(data);  // ❌ FORBIDDEN
let rw = RwLock::new(data);  // ❌ FORBIDDEN
```

#### ✅ Required Patterns
```rust
// Use ? operator for error propagation
fn operation() -> Result<(), Error> {
    let val = option?;  // ✅ Proper
    Ok(())
}

// Use stack allocation
let buf = [0u8; 256];  // ✅ Fixed-size

// Use bounded channels
let (tx, rx) = crossbeam::channel::bounded(32);  // ✅ Proper

// Use atomics
use std::sync::atomic::{AtomicU32, Ordering};
let counter = AtomicU32::new(0);
counter.fetch_add(1, Ordering::SeqCst);  // ✅ Lock-free
```

### Documentation Requirements

Every public function must document:
- What it does
- WCET (Worst-Case Execution Time)
- Memory requirements
- Error conditions

```rust
/// Reads sensor data and returns processed measurement.
///
/// This function is part of the main control loop and must complete
/// within the documented deadline to maintain hard real-time guarantees.
///
/// # Real-Time Guarantees
/// - WCET: O(1), ≤ 10µs
/// - Allocation: None
/// - Lock-free: Yes
///
/// # Errors
/// Returns `SensorError` if:
/// - Sensor is not initialized
/// - Read times out
/// - Data is invalid or out of range
///
/// # Safety
/// This function does not call any unsafe code.
pub fn read_sensor(&mut self) -> Result<f32, SensorError> {
    // Implementation
}
```

## Testing

### Unit Tests
Add tests in the same file as the code:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_sensor() {
        // Test implementation
    }
}
```

### Integration Tests
Add tests in `tests/` directory:
```bash
tests/
├── sensor_integration.rs
├── telemetry_integration.rs
└── scheduler_integration.rs
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
cargo flamegraph --bin room619 -- --profile
```

### Checking Binary Size
```bash
cargo build --release
wc -c target/release/room619
```

## Security Considerations

### Dependency Review
- All external dependencies must be reviewed
- No C/C++ FFI in critical paths
- Use `cargo deny` to check licenses and vulnerabilities

```bash
cargo deny check
```

### Unsafe Code
- Minimize unsafe blocks
- Always document with `// SAFETY:` comments
- Include memory barriers where needed

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
- ✅ Security audit
- ✅ Real-time validation

## Commit Hooks (Optional)

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

## Reviewing PRs

When reviewing, ensure:
1. ✅ Code follows hard real-time standards
2. ✅ No new allocations in hot paths
3. ✅ All public functions documented with WCET
4. ✅ Tests are comprehensive
5. ✅ CI passes
6. ✅ Commit messages follow convention

## Reporting Issues

### Security Issues
For security vulnerabilities, **do not** open a public issue. Instead, email the maintainers privately.

### Bugs
Include:
- Rust version (`rustc --version`)
- Platform (Linux/Windows/macOS)
- Minimal reproducible example
- Expected vs actual behavior

### Feature Requests
Include:
- Use case description
- Why it's needed
- How it impacts WCET/memory constraints

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
