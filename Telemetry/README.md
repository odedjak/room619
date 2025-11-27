# Telemetry crate

This crate contains the telemetry building blocks for the room619 project.

Purpose
- Provide message structures and helpers for sending telemetry from embedded
  modules to the remote system.

Quick start
- Build: `cargo build --manifest-path Telemetry/Cargo.toml`
- Test:  `cargo test --manifest-path Telemetry/Cargo.toml`

Where to work
- Put telemetry implementation code under `Telemetry/src/` and unit tests in
  the same crate. Integration tests may be placed under `Telemetry/tests/`.
Telemetry Module — Template
===========================

This directory contains the telemetry template and a minimal Rust crate
(`telemetry`) intended as the starting point for the Telemetry Engineer.

Goals (Telemetry Engineer)
- Provide a telemetry abstraction that allows sending structured telemetry to
  remote systems (MQTT, gRPC, or custom binary).
- Ensure the interface is testable on-host (desktop) and suitable for
  embedded targets.

Recommended structure
- `Telemetry/Cargo.toml` — crate manifest
- `Telemetry/src/lib.rs` — trait definitions and lightweight mocks
- `Telemetry/Tests` — helper scripts and CI-run wrappers (see below)

Design template (telemetry block)
- Interface: Define a small trait (e.g. `TelemetrySink`) with methods like
  `send(topic: &str, payload: &[u8]) -> Result<(), Error>`.
- Message schema: Prefer protobuf or CBOR for compact, typed messages.
- Transport: Implementation-specific (MQTT/gRPC/custom) behind the trait.
- Error handling: Provide clear, non-panicking error types for retry logic.

Testing
- Unit tests: Provide fast, in-crate unit tests (see `src/lib.rs`).
- Integration/CI: `Telemetry/Tests/run_tests.ps1` will run the crate tests in CI.

Next steps
- Implement a production sink (MQTT or gRPC) behind `TelemetrySink`.
- Add serialization helpers for the chosen message format.
