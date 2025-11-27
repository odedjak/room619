# room619 Core Infrastructure

Modular real-time framework core with platform abstraction layer.

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Testing

```bash
cargo test
```

## Features

- **Platform Abstraction Layer** — Trait-based implementations for different platforms
- **Scheduler** — Task scheduling and management
- **Timer** — Timing primitives
- **Tracing** — Structured logging with tracing-rs

## Supported Platforms

- ✅ Desktop (Linux, Windows, macOS)
- 🔄 ARM Cortex-M (RTIC support - coming soon)
- 🔄 VxWorks (integration - coming soon)
- 🔄 FreeRTOS (integration - coming soon)

## Directory Structure

```
Infrastructure/
├── Cargo.toml           # Project manifest
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library root
│   ├── platform/       # Platform abstraction
│   ├── scheduler/      # Scheduling logic
│   └── timer/          # Timing primitives
├── tests/              # Integration tests
└── README.md           # This file
```
