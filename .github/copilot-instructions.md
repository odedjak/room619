# AI Coding Agent Instructions for room619

## Project Identity

**room619** is a Modular Real-Time Embedded Framework for Autonomous Systems built in Rust.

- **Vision**: Scalable, component-based framework with real-time capability and robust telemetry streaming
- **Core Principle**: Memory safety first (no C/C++ dependencies), real-time capable, modular design, cross-platform
- **Architecture**: Component-based with 6 core modules owned by 3 developers
- **Platform**: Desktop testing (Linux) + embedded hardware (STM32/ARM)

## Module Ownership

| Module | Owner | Purpose |
|--------|-------|---------|
| HAL & Scheduling | Dev 1 | Hardware abstraction, real-time task scheduling |
| Navigation & Control | Dev 2 | Autonomous logic, actuator control |
| Telemetry | Dev 3 | Data collection, remote transmission |
| Common/Utils | All | Shared error types, logging, config |

## Code Generation Guidelines

### Traits & Abstractions First
- Define traits before implementations
- Use trait objects (`dyn Trait`) in public APIs
- Keep platform-specific code in isolated modules with `#[cfg(...)]` guards
- Example trait pattern: `pub trait HardwareDriver: Send + Sync { ... }`

### Real-Time Annotations
- Label timing-critical functions with doc comments: `/// Real-Time: 10ms deadline`
- Use trait methods for context switches (avoid blocking operations in async code)
- Document scheduling constraints at module level

### Cross-Platform Development
- Separate platform implementations: `src/hal/linux/`, `src/hal/stm32/`, `src/hal/generic/`
- Use Cargo features for platform selection: `desktop`, `embedded`, `stm32h7`
- Conditional compilation for platform-specific code
- Test on desktop (Linux simulation) before targeting embedded hardware

### Error Handling
- Centralize error types in `src/common/error.rs`
- Implement `std::error::Error` for custom errors
- Use `Result<T, CustomError>` pattern throughout
- No unwraps in production code (document why if necessary with `// SAFETY: ...`)

### Module Communication
- Dev 1 → Dev 2: HAL traits and scheduling interfaces
- Dev 2 → Dev 3: State/sensor data via telemetry channels
- Dev 3 ← All modules: Structured telemetry events
- Use async channels (`tokio::sync::mpsc`) for data flow

### Testing & Quality
- Unit tests in `#[cfg(test)]` modules adjacent to implementation
- Integration tests in `tests/` directory
- Must pass: `cargo fmt --check`, `cargo clippy`, `cargo test`
- Target 80%+ coverage for core modules

## Build & Development

**Setup**: `cargo build`  
**Test**: `cargo test`  
**Format**: `cargo fmt`  
**Lint**: `cargo clippy`  

**Branch**: Commit to feature branches, PR to `oded`, then to `master`  
**Dependencies**: Rust-native only, no C/C++ wrappers

## Integration Points

- **Remote Monitoring System**: Receives structured telemetry via MQTT/gRPC/custom binary
- **Embedded Hardware**: HAL abstracts platform differences; currently targeting STM32/ARM
- **Scheduling**: `tokio` for soft real-time (desktop), `rtic` for hard real-time (embedded)

## Documentation References

- **Architecture details**: See `docs/ARCHITECTURE.md`
- **Team roles & conventions**: See `docs/DEVELOPER_GUIDE.md`
- **All doc comments must explain "why" not just "what"**

---

**Last updated**: 27 November 2025  
**For AI agents using this project**: Follow these patterns consistently. When uncertain about architecture, prioritize trait-based abstractions and cross-platform compatibility.
