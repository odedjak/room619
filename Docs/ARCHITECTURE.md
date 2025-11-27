# room619 Architecture

## Project Overview

**room619** is a **Modular Real-Time Embedded Framework for Autonomous Systems** built in Rust. The project challenges traditional C/C++ embedded development by leveraging Rust's safety, concurrency, and performance features to create a modern alternative for mission-critical autonomous operations.

### Vision
Build a scalable, component-based framework that enables real-time autonomous systems with robust telemetry streaming to remote monitoring systems, all while maintaining memory safety and zero runtime overhead.

### Core Principles
- **Memory Safety First**: No C/C++ dependencies; Rust-native solutions only
- **Real-Time Capable**: Predictable scheduling and deterministic performance
- **Modular Design**: Each component independently deployable and testable
- **Cross-Platform**: Runnable on desktop (testing) and embedded hardware

## Architecture & Core Modules

### Component-Based Design

The framework consists of modular, independently testable components:

| Module | Developer Owner | Purpose |
|--------|-----------------|---------|
| **Hardware Abstraction Layer (HAL)** | Dev 1 (Core & Scheduling) | Safe Rust interfaces to embedded hardware |
| **Scheduling Layer** | Dev 1 (Core & Scheduling) | Real-time task scheduling (`tokio`/`rtic`) |
| **Navigation Module** | Dev 2 (Logic & Control) | Position estimation, path planning, obstacle avoidance |
| **Control Module** | Dev 2 (Logic & Control) | Real-time control loops for actuators |
| **Telemetry Module** | Dev 3 (Communication) | Structured data collection and remote transmission |
| **Common/Utilities** | All (Shared) | Shared types, error enums, logging, config |

### Communication Architecture

```
Embedded System (room619)
    ├── HAL (hardware drivers)
    ├── Core Modules (navigation, control, telemetry)
    └── Telemetry Client
         └── Remote Monitoring System (separate team)
```

### Real-Time Constraints

- Identify hard/soft real-time deadlines per module
- Use `rtic` for predictable hardware timers or `tokio` for soft real-time
- Document timing requirements in module-level comments
- Measure and validate latency during integration testing

## Key Files & Structure

- `Cargo.toml` - Workspace manifest with feature flags per module
- `src/main.rs` - Entry point for embedded/desktop simulation
- `src/lib.rs` - Library root exposing public module interfaces
- `src/hal/` - Hardware abstraction layer (platform-agnostic traits)
- `src/modules/` - Core modules (navigation, control, telemetry)
- `src/scheduling/` - Real-time scheduling layer
- `src/common/` - Shared types, error enums, logging
- `tests/` - Integration tests validating cross-module behavior

## External Dependencies & Integration Points

### Telemetry Integration
- **Primary Integration**: Remote monitoring system (separate team)
- **Supported Protocols**: MQTT, gRPC, or custom binary formats
- **Data Format**: Structured telemetry with timestamps, module source, and metadata
- **Error Handling**: Graceful degradation if telemetry is unavailable

### Rust Crates Strategy
- **Async Runtime**: `tokio` for soft real-time tasks or `rtic` for hard real-time
- **HAL**: Platform abstractions using `embedded-hal` patterns
- **Serialization**: `serde` + `bincode`/`postcard` for embedded efficiency
- **Logging**: `tracing` or `log` crate with structured output

### No External C/C++ Rule
- All dependencies must be Rust-native
- If a capability requires C/C++, implement Rust-native alternative or document as architectural limitation
