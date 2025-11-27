# Developer Guide

## Team Organization (3 Developers)

The team is divided into 3 focus areas, with each developer owning one domain and collaborating on interfaces:

### **Developer 1: Real-Time Core & Scheduling**

**Responsible for**: Timing-critical operations and system orchestration

**Modules:**
- **Hardware Abstraction Layer (HAL)**: Safe Rust traits for hardware access (timers, interrupts, GPIO)
- **Scheduling Layer**: Real-time task scheduling (`tokio` soft real-time or `rtic` hard real-time)
- **Common/Utilities**: Shared error types, timing utilities, configuration loading

**Key Deliverable**: Reliable foundation for all other modules to build on

**Interface Contracts**:
- Exposes `HardwareDriver` traits (generic over platform)
- Provides async task execution guarantees with deadline tracking
- Documents all timing constraints in `// Real-time: Xms deadline` comments

---

### **Developer 2: Navigation & Control Logic**

**Responsible for**: Autonomous decision-making and actuation

**Modules:**
- **Navigation Module**: Position estimation, path planning, obstacle avoidance algorithms
- **Control Module**: Real-time control loops (PID, state machines) for actuators and system dynamics

**Key Deliverable**: Working autonomous behavior on test platform

**Interface Contracts**:
- Consumes `HardwareDriver` traits from scheduling layer
- Publishes state/sensor data via telemetry interface
- Accepts commands from remote system (via telemetry channel)

---

### **Developer 3: Telemetry & Remote Communication**

**Responsible for**: Data transmission and remote system integration

**Modules:**
- **Telemetry Module**: Structured data collection from all modules, serialization, buffering
- **Protocol Implementation**: MQTT, gRPC, or custom binary formats for remote transmission
- **Error Handling**: Graceful degradation when remote system unavailable

**Key Deliverable**: Real-time telemetry stream flowing to remote monitoring team

**Interface Contracts**:
- Defines standard `TelemetryEvent` and `Command` schemas (using `serde`)
- Provides async channels for modules to publish telemetry
- Handles backpressure and retries transparently

---

## Development Conventions

### When Adding Features

1. **Identify ownership**: Determine which module/role owns this feature
2. Create source files under appropriate `src/` subdirectory following Rust module conventions
3. **Define HAL traits first** if accessing hardware (abstract before implementing)
4. **Add telemetry hooks** if data should be monitored remotely (discuss format with Telemetry Engineer)
5. Run `cargo check` frequently during development to catch errors early
6. Use `cargo fmt` before committing to maintain consistent style
7. Run `cargo clippy` to identify idiomatic Rust improvements
8. Add tests alongside implementation using `#[cfg(test)]` modules
9. **Document real-time constraints** if the feature has timing requirements

### Module Interface Patterns

- Use trait objects for component abstraction (prefer `dyn Trait` over concrete types in public APIs)
- Define async boundaries clearly (tokio tasks vs. blocking operations)
- Include timing annotations in docs: `/// Real-time: 10ms deadline`
- Version public APIs; breaking changes require team discussion

### Branch Strategy

- **Current branch**: `oded` (active development)
- **Primary branch**: `master` (synced with origin/master)
- **Feature branches**: Create from `oded` with naming convention: `feature/module-name` or `fix/issue-description`
- **Commit to feature branches** and create PRs to merge to `oded`, then to `master`

---

## Code Quality Standards

- **Formatting**: Must pass `cargo fmt`
- **Linting**: Must pass `cargo clippy` with no warnings
- **Testing**: 
  - Unit tests for module logic (at least 80% coverage for core modules)
  - Integration tests for cross-module data flow
  - Real-time tests verifying timing constraints (use `criterion` for benchmarking)
- **Documentation**: 
  - Doc comments (`///`) for all public APIs with examples
  - Module-level comments explaining architecture and constraints
  - Timing requirements clearly labeled
- **Safety**: Minimize `unsafe` blocks; document with `// SAFETY: ...` comments explaining invariants
- **No External C/C++**: All dependencies must be Rust-native

---

## Common Development Tasks

| Task | Command |
|------|---------|
| Check syntax/types | `cargo check` |
| Run tests | `cargo test` |
| Build release | `cargo build --release` |
| Generate docs | `cargo doc --open` |
| Update dependencies | `cargo update` |
| Format code | `cargo fmt` |
| Lint check | `cargo clippy` |

---

## Questions to Clarify During Development

- [ ] Which embedded platform(s) will be targeted? (STM32, ARM, custom)
- [ ] What Rust edition and MSRV (Minimum Supported Rust Version) should be targeted?
- [ ] How will the HAL abstract different hardware platforms?
- [ ] What are the hard real-time deadlines for critical modules?
- [ ] Which async runtime (`tokio` vs `rtic`) per module?
- [ ] Telemetry protocol finalization (MQTT/gRPC/binary) and schema?
- [ ] Deployment strategy: firmware flashing, over-the-air updates?
- [ ] Performance targets: CPU/memory constraints on embedded hardware?
