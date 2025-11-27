# Architecture: Modular Real-Time Embedded Framework

## Executive Summary

room619 is a modular, scalable, real-time embedded framework built in Rust for autonomous systems. It replaces traditional C/C++ embedded development with Rust's memory safety and zero-cost abstractions while maintaining hard real-time guarantees.

## Design Principles

### 1. **Modularity**
- Each component (HAL, Telemetry, Scheduler, Autonomous Modules) is independently deployable
- Clear separation of concerns with trait-based interfaces
- Minimal coupling between modules

### 2. **Real-Time Determinism**
- **Hard Real-Time**: Zero allocations, lock-free primitives, bounded operations
- **Bounded Worst-Case Execution Time (WCET)**: All functions document timing guarantees
- **Predictable Latency**: O(1) operations in critical paths

### 3. **Safety & Reliability**
- Memory safety without runtime overhead
- Exhaustive pattern matching with compiler enforcement
- Result-based error handling with no panics in production code

### 4. **Cross-Platform Support**
- Desktop testing (Linux, Windows, macOS)
- Embedded targets (ARM, RISC-V) via conditional compilation
- Platform abstraction layer for hardware interactions

## System Architecture

```
┌──────────────────────────────────────────────────────────┐
│         Autonomous System Components                      │
│  (Navigation, Control, Diagnostics, Decision-Making)     │
└────────────────┬─────────────────────────────────────────┘
                 │
        ┌────────▼────────┐
        │   Scheduler     │
        │  Real-Time Task │
        │   Management    │
        └────────┬────────┘
                 │
    ┌────────────┼────────────┐
    │            │            │
┌───▼──────┐ ┌──▼──────┐ ┌───▼──────┐
│   HAL    │ │Telemetry│ │ Network  │
│(Sensors, │ │(Remote  │ │(MQTT,    │
│ Actuator)│ │Streaming)│ │gRPC)     │
└─────────┬┘ └───┬─────┘ └─────┬────┘
          │      │             │
          └──────┴─────────────┘
                 │
        ┌────────▼────────┐
        │  Embedded Hw or │
        │   Simulation    │
        └─────────────────┘
```

## Core Modules

### 1. **Hardware Abstraction Layer (HAL)** (`src/hal/`)

**Purpose**: Abstraction for sensor and actuator interactions

**Key Traits**:
```rust
pub trait Sensor<T> {
    fn read(&mut self) -> Result<T, SensorError>;
    fn sample_rate(&self) -> Duration;
}

pub trait Actuator<T> {
    fn write(&mut self, value: T) -> Result<(), ActuatorError>;
    fn response_time(&self) -> Duration;
}
```

**Files**:
- `mod.rs` — Trait definitions and platform selection
- `sensor.rs` — Sensor trait and implementations
- `actuator.rs` — Actuator trait and implementations
- `platform.rs` — Platform-specific hardware bindings

**Hard Real-Time Constraints**:
- Read/write operations must complete within documented deadlines
- No allocations in hot read/write loops
- Stack-allocated buffers only

### 2. **Telemetry Layer** (`src/telemetry/`)

**Purpose**: Structured data streaming to remote systems

**Key Traits**:
```rust
pub trait TelemetryClient {
    fn send_frame(&mut self, frame: &TelemetryFrame) -> Result<(), TelemetryError>;
    fn is_connected(&self) -> bool;
}

pub struct TelemetryFrame {
    timestamp: Instant,
    component_id: u8,
    sensor_data: &'static [SensorReading],
    diagnostics: DiagnosticsInfo,
}
```

**Supported Protocols**:
- **MQTT** — Lightweight pub-sub (low bandwidth)
- **gRPC** — High-performance streaming (low latency)
- **Custom Binary** — Optimized for specific systems

**Files**:
- `mod.rs` — Client trait and frame definitions
- `protocol.rs` — Protocol implementations
- `schema.rs` — Serialization/deserialization

**Real-Time Constraints**:
- Send operations use bounded channels
- Backpressure handling prevents allocation spikes
- Zero allocations in send path

### 3. **Scheduler** (`src/scheduler/`)

**Purpose**: Real-time task scheduling and coordination

**Key Concepts**:
```rust
pub struct Task {
    id: TaskId,
    deadline: Duration,
    period: Duration,
    priority: Priority,
    wcet: Duration,  // Worst-case execution time
}

pub trait Scheduler {
    fn schedule(&mut self, task: Task) -> Result<(), ScheduleError>;
    fn run(&mut self) -> Result<(), ScheduleError>;
}
```

**Scheduling Strategies**:
- **Fixed-Priority** — Deterministic, suitable for embedded
- **Rate-Monotonic** — Priority inversely proportional to period
- **Deadline-Monotonic** — Priority inversely proportional to deadline

**Files**:
- `mod.rs` — Scheduler trait and runner
- `task.rs` — Task definitions
- `timing.rs` — WCET tracking and validation

**Hard Real-Time Guarantees**:
- No task starvation
- Deadline misses detected and reported
- Context switching overhead minimized

### 4. **Autonomous Components** (`src/components/`)

**Purpose**: Mission-specific autonomous system modules

**Example Components**:

#### Navigation Module
```rust
pub struct NavigationState {
    position: [f32; 3],
    velocity: [f32; 3],
}

pub trait NavigationEngine {
    fn update(&mut self, sensor_data: &SensorData) -> Result<NavigationState, NavError>;
}
```

#### Control Module
```rust
pub trait ControlSystem {
    fn compute_control(&mut self, state: &SystemState) -> Result<ControlCommands, ControlError>;
}
```

#### Diagnostics Module
```rust
pub trait DiagnosticsMonitor {
    fn check_health(&self) -> HealthStatus;
    fn report_anomalies(&self) -> Vec<Anomaly>;
}
```

**Files**:
- `mod.rs` — Component registry
- `navigation.rs` — Navigation logic
- `control.rs` — Control logic
- `diagnostics.rs` — Health monitoring

## Data Flow

### Typical Execution Cycle

```
1. Scheduler wakes periodic task
   ↓
2. HAL reads sensors (bounded time)
   ↓
3. Component processes sensor data
   ↓
4. Control system computes actuator commands
   ↓
5. HAL writes to actuators (bounded time)
   ↓
6. Telemetry sends frame to remote system
   ↓
7. Diagnostics checks system health
   ↓
8. Sleep until next deadline
```

### Memory Management

**Stack Allocation** (Preferred):
```rust
let buffer = [0u8; 256];  // Fixed-size, zero-allocation
```

**Bounded Heap** (If necessary):
```rust
use heapless::Vec;
let mut vec = Vec::<u8, 256>::new();  // Bounded capacity
```

**Lock-Free Synchronization**:
```rust
use crossbeam::channel::bounded;
let (tx, rx) = bounded(32);  // Bounded queue, no locks
```

## Error Handling

All fallible operations use `Result<T, E>`:

```rust
pub fn read_sensor(&mut self) -> Result<SensorData, SensorError> {
    // Implementation
}

// Usage with ? operator (no panic)
pub fn update_state(&mut self) -> Result<(), StateError> {
    let data = self.read_sensor()?;
    self.process_data(data)?;
    Ok(())
}
```

## Configuration & Features

### Compile-Time Configuration
```toml
[features]
default = ["desktop"]
desktop = ["simulation"]
embedded-arm = ["hal-arm"]
embedded-riscv = ["hal-riscv"]
telemetry-mqtt = ["mqtt"]
telemetry-grpc = ["grpc"]
```

### Runtime Configuration
Environment variables:
```bash
RUST_LOG=debug              # Logging level
TELEMETRY_ENDPOINT=...     # Remote endpoint
SCHEDULING_MODE=fixed      # Scheduling algorithm
```

## Concurrency Model

### No Mutex/RwLock in Real-Time Paths

**Instead Use**:
- **Atomics** — Lock-free state updates
```rust
use std::sync::atomic::{AtomicU32, Ordering};
let counter = AtomicU32::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

- **Bounded Channels** — Message passing
```rust
let (tx, rx) = crossbeam::channel::bounded(32);
tx.send(message)?;
let msg = rx.recv()?;
```

- **Arc + Atomic** — Shared ownership + atomic updates
```rust
use std::sync::Arc;
let shared = Arc::new(AtomicU32::new(0));
```

## Testing Strategy

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test '*'
```

### Real-Time Validation
```bash
cargo test --all-features --release
```

## Performance Guarantees

| Operation | WCET | Allocation |
|-----------|------|-----------|
| Sensor Read | ≤ 10µs | None |
| Control Compute | ≤ 100µs | None |
| Telemetry Send | ≤ 50µs | None |
| Task Context Switch | ≤ 5µs | None |

## Platform Support

### Desktop (Testing)
- Linux (x86_64)
- Windows (x86_64)
- macOS (x86_64, ARM64)

### Embedded (Production)
- ARM Cortex-M4/M7 (STM32, NXP, etc.)
- ARM Cortex-A9+ (BeagleBone, RPI4, etc.)
- RISC-V (Future)

## Deployment Pipeline

```
Source Code
    ↓
CI/CD (GitHub Actions)
  ├─ Cargo check
  ├─ Rustfmt
  ├─ Clippy
  ├─ Tests
  └─ Cross-platform build
    ↓
Security Checks
  ├─ RustSec audit
  ├─ Cargo Deny
  ├─ Miri (UB detection)
  └─ Real-time validation
    ↓
Release Artifacts
  ├─ Linux binary
  ├─ Windows binary
  └─ macOS binary
```

## Extension Points

### Adding a New Sensor
1. Implement `Sensor<T>` trait in `src/hal/sensor.rs`
2. Register in `src/hal/mod.rs`
3. Add integration test in `tests/sensor_integration.rs`

### Adding a New Protocol
1. Implement `TelemetryClient` in `src/telemetry/protocol.rs`
2. Define serialization in `src/telemetry/schema.rs`
3. Add feature flag in `Cargo.toml`

### Adding a New Component
1. Create module in `src/components/`
2. Implement component interface
3. Register in `src/components/mod.rs`
4. Add to scheduler

## Future Enhancements

- [ ] Multi-core scheduling with core affinity
- [ ] Time-triggered vs event-triggered hybrid scheduler
- [ ] Formal verification of timing properties
- [ ] Probabilistic real-time analysis
- [ ] Hardware-in-the-loop simulation
