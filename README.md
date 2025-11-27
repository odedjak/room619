# room619 - Modular Real-Time Embedded Framework for Autonomous Systems

A modern, safe, and performant embedded framework built in **Rust** for autonomous systems. This framework replaces traditional C/C++ embedded development with Rust's memory safety, zero-cost abstractions, and strong concurrency model.

## 🎯 Core Vision

- **Modular Architecture**: Component-based design for sensors, actuators, decision-making, and communication
- **Real-Time Performance**: Hard real-time guarantees suitable for mission-critical autonomous operations
- **Telemetry Streaming**: Robust streaming to remote systems (MQTT, gRPC, or custom binary formats)
- **No Legacy Dependencies**: Pure Rust—no C/C++ FFI in critical paths
- **Cross-Platform**: Runs on embedded hardware and desktop environments for testing

## 🏗️ Architecture

```
src/
├── lib.rs                    # Core framework entry point
├── telemetry/               # Remote telemetry abstraction layer
│   ├── mod.rs              # Telemetry trait and interfaces
│   ├── protocol.rs         # Protocol handlers (MQTT, gRPC)
│   └── schema.rs           # Data structures for streaming
├── hal/                     # Hardware abstraction layer
│   ├── mod.rs              # HAL trait definitions
│   ├── sensor.rs           # Sensor interfaces
│   ├── actuator.rs         # Actuator interfaces
│   └── platform.rs         # Platform-specific implementations
├── scheduler/              # Real-time scheduling
│   ├── mod.rs              # Scheduling engine
│   ├── task.rs             # Task definitions
│   └── timing.rs           # Timing utilities (WCET tracking)
└── components/             # Modular autonomous system components
    ├── mod.rs              # Component registry
    ├── navigation.rs       # Navigation module
    ├── control.rs          # Control module
    └── diagnostics.rs      # Diagnostics module
```

## 🚀 Key Features

### 1. **Component-Based Design**
Each module is independently deployable and testable:
- Sensors, actuators, and decision-making modules
- Clean interfaces for integration
- Hot-swappable components

### 2. **Hard Real-Time Guarantees**
- **Zero allocations** in timing-critical paths
- **O(1) operations** with bounded worst-case execution time (WCET)
- Lock-free synchronization using atomics and bounded channels
- No mutexes, RwLocks, or unbounded async operations

### 3. **Telemetry Abstraction**
- Structured data streaming to remote systems
- Support for multiple protocols (MQTT, gRPC, custom)
- Backpressure handling and circuit breaking
- Minimal latency overhead

### 4. **Hardware Abstraction Layer**
- Pure Rust HAL without unsafe code in critical paths
- Support for multiple platforms (x86, ARM, RISC-V)
- Platform-specific fallbacks with `#[cfg]` blocks

## 📋 Requirements

### Build
- **Rust**: 1.70+ (stable)
- **Cargo**: Latest

### Runtime
- **Desktop Testing**: Linux, Windows, macOS (x86_64)
- **Embedded Targets**: ARM, RISC-V (configurable)

## 🛠️ Getting Started

### Build
```bash
cargo build --release
```

### Test (Desktop Simulation)
```bash
cargo test --all-features --verbose
cargo test --all-features --release
```

### Cross-Platform Build
```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS
cargo build --release --target x86_64-apple-darwin
```

### Run with Telemetry
```bash
RUST_LOG=debug cargo run --release --features "telemetry-mqtt"
```

## 🔐 Code Standards

All code must adhere to **hard real-time Rust** standards:

### ❌ Forbidden Patterns
```rust
.unwrap() / .expect()       // Use ? operator instead
vec![] in hot paths         // Stack alloc or heapless collections
HashMap / thread::spawn()   // Use BTreeMap / crossbeam channels
Mutex / RwLock              // Use atomics or lock-free structures
allocate / deallocate       // Pre-allocate or use stack
```

### ✅ Required Patterns
```rust
operation()?                // Error propagation
let buf = [0u8; 256];      // Stack allocation
let (tx, rx) = crossbeam::channel::bounded(N);  // Bounded channels
std::sync::atomic::*        // Lock-free synchronization
```

### Documentation Requirements
```rust
/// Operation description.
/// # Real-Time Guarantees
/// WCET: O(1), ≤ 100µs. Zero allocations.
/// # Errors
/// Returns `Err` if timeout or resource exhausted.
pub fn operation() -> Result<T, E> { }
```

## 🔄 CI/CD Pipelines

### Continuous Integration (CI)
Runs on every push and pull request:
- ✅ Cargo check
- ✅ Rustfmt (code formatting)
- ✅ Clippy (linting)
- ✅ Unit tests
- ✅ Desktop simulation tests
- ✅ Integration tests (telemetry)
- ✅ Cross-platform builds (Linux, Windows, macOS)

### Continuous Deployment (CD)
Triggered on version tags (`v*`):
- 📦 Automated release builds
- 📤 Platform-specific artifacts
- 🏷️ GitHub release creation

### Security & Real-Time Checks
Runs on every push and pull request:
- 🔍 Dependency vulnerability scanning (RustSec)
- 📜 License compliance (Cargo Deny)
- 🐛 Undefined behavior detection (Miri)
- ⏱️ Hard real-time validation (forbidden patterns)
- 📡 Telemetry interface validation

## 📦 Dependencies

### Allowed
- `crossbeam` — Cross-platform concurrency (bounded channels, work-stealing)
- `parking_lot` — High-performance synchronization primitives
- `tokio` — Async runtime (with timeout bounds)
- `heapless` — Static collections for embedded systems
- Minimal pure-Rust crates

### Forbidden
- C/C++ FFI (without explicit approval)
- Unbounded async operations
- Mutex/RwLock in hard real-time paths
- Allocating collections in hot paths

## 📊 Telemetry Protocol

### Supported Formats
- **MQTT**: Lightweight, pub-sub messaging
- **gRPC**: High-performance RPC with streaming
- **Custom Binary**: Optimized for specific use cases

### Message Structure
```rust
pub struct TelemetryFrame {
    timestamp: Instant,
    component_id: u8,
    sensor_data: Vec<SensorReading>,
    actuator_states: Vec<ActuatorState>,
    diagnostics: DiagnosticsInfo,
}
```

## 🧪 Testing Strategy

### Unit Tests
Located in source files with `#[cfg(test)]` modules:
```bash
cargo test --lib
```

### Integration Tests
Located in `tests/` directory:
```bash
cargo test --test '*'
```

### Benchmarks
Performance profiling with `criterion`:
```bash
cargo bench
```

## 🎓 Team Roles & Responsibilities

- **System Architect** — Modular structure, interface design
- **Embedded Developer** — HAL implementation, low-level hardware interactions
- **Telemetry Engineer** — Communication layer, protocol handlers, remote system compatibility
- **Integrator & Tester** — System validation, performance verification, cross-platform testing

## 📚 Documentation

- **Architecture**: See `docs/architecture.md`
- **API Reference**: Generated via `cargo doc --open`
- **Hard Real-Time Guidelines**: See `.github/Infrastructure/copilot-instructions.md`

## 🚢 Deployment

### Desktop Testing
```bash
cargo run --release --features "simulation"
```

### Embedded Deployment
```bash
cargo build --release --target arm-unknown-linux-gnueabihf
```

## 📝 Commit Convention

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Test improvements
- `refactor:` Code restructuring
- `perf:` Performance optimization
- `ci:` CI/CD updates

Example:
```
feat: add MQTT telemetry protocol support

- Implement MQTT client abstraction
- Add bounded message queue
- Verify hard real-time constraints
```

## 🤝 Contributing

1. **Fork** and create a feature branch (`git checkout -b feature/your-feature`)
2. **Commit** with descriptive messages following conventions
3. **Push** to your fork and open a **Pull Request**
4. Ensure all CI checks pass and code follows hard real-time standards

## 📄 License

[Add your license here]

## 🎉 Hackathon Deliverables

By the end of the hackathon, we aim to deliver:
- ✅ Working prototype running on desktop and embedded hardware
- ✅ Real-time telemetry streaming to remote system
- ✅ Comprehensive architecture documentation
- ✅ Integration points clearly defined
