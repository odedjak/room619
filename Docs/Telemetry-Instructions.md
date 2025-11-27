Telemetry — Integration & Developer Instructions
===============================================

Purpose
-------
This document explains what the `Telemetry` crate provides, how to integrate
it into other parts of `room619`, and how to run its tests locally or in CI.

What is the "main" of Telemetry?
-------------------------------
- `Telemetry` is a library crate (not a binary) providing a small, testable
  telemetry abstraction. There is no `fn main()` inside the crate — it
  exposes types and traits you consume from other crates/services.
- Primary items:
  - `TelemetryMessage` — a simple message struct (topic + JSON payload).
  - `TelemetrySink` — a trait describing the sink interface (send(topic, payload)).
  - `MockSink` — a lightweight, in-repo mock implementation useful for
    tests and local development.

Quick usage
-----------
1. Add the `Telemetry` dependency to your crate. If you're working in the same
   repository and prefer a path dependency, add to your `Cargo.toml`:

   telemetry = { path = "Telemetry" }

   Or, if you published or use a git remote, use the git specification.

2. Use the public API:

   ```rust
   use telemetry::{TelemetryMessage, TelemetrySink, TelemetryClient, TelemetryResult};
   use std::sync::Arc;

   pub struct MySink;

   impl TelemetrySink for MySink {
       fn send(&self, topic: &str, payload: &[u8]) -> TelemetryResult<()> {
           // encode / transmit using MQTT / gRPC / socket, etc.
           println!("Sending to {}: {} bytes", topic, payload.len());
           Ok(())
       }
   }

   // Example: create message and send
   let payload = serde_json::json!({"temp": 24.5});
   let msg = TelemetryMessage::new("sensors/temp", payload);
   let client = TelemetryClient::new(Arc::new(MySink));
   client.send_message(&msg)?;
   ```

API reference (high level)
--------------------------
- `TelemetryError`:
  - A centralized error type implementing `std::error::Error`.
  - Used for transport failures and lock poisoning (thread-safe operations).
  - Create with: `TelemetryError::new("message")`
  - Example: `TelemetryError::new("MQTT publish failed")`

- `TelemetryResult<T>`:
  - Type alias: `Result<T, TelemetryError>`
  - Used throughout the crate for consistent error handling.

- `TelemetryMessage`:
  - fields: `topic: String`, `payload: serde_json::Value`
  - helpers: `TelemetryMessage::new(topic, payload)`, `to_json()`
  - Serializable: implements `Serialize` and `Deserialize` for easy transmission.

- `TelemetrySink` trait:
  - `fn send(&self, topic: &str, payload: &[u8]) -> TelemetryResult<()>`
  - Trait bounds: `Send + Sync` (safe for concurrent use across threads)
  - Implement this to add support for MQTT, gRPC, custom binary protocols, etc.

- `TelemetryClient`:
  - Constructor: `TelemetryClient::new(Arc<dyn TelemetrySink>)`
  - Methods:
    - `send_message(&self, msg: &TelemetryMessage) -> TelemetryResult<()>` — serialize and send a message
    - `send_binary(&self, topic: &str, data: &[u8]) -> TelemetryResult<()>` — send raw bytes

- `MockSink`:
  - Useful for tests: it prints the outbound payload to stdout and returns `Ok(())`.

- `InMemorySink`:
  - Test-friendly: stores all sent messages in a thread-safe `Arc<Mutex<Vec<...>>>`.
  - Retrieve records with: `sink.records_arc()` to inspect what was sent.
  - Implements `Default` for convenience: `InMemorySink::default()`.

- Feature-gated protocol stubs (optional):
  - `mqtt::MqttSink` (requires `features = ["mqtt"]`) — MQTT broker abstraction
  - `grpc::GrpcSink` (requires `features = ["grpc"]`) — gRPC endpoint abstraction
  - `all-protocols` — convenience flag enabling all protocol features
  - Currently these are stubs; implement the real transport when ready.

Tests & CI
---------
- Run tests for the Telemetry crate locally:

  ```powershell
  # from repository root
  cargo test --manifest-path Telemetry/Cargo.toml --verbose
  ```

- The repository CI discovers all `Cargo.toml` files and runs `cargo build` and
  `cargo test` for every crate it finds. Pushing changes to `amit` triggers the
  workflow, which therefore builds and tests the `Telemetry` crate automatically.

Integration notes and best practices
-----------------------------------
- If your sink requires external setup (MQTT broker, emulator, secrets), keep
  the integration tests that depend on external services behind a feature flag
  (for example `integration-tests`) and do not enable it by default in CI.
- Prefer to use `MockSink` in unit tests to avoid network dependencies.
- If multiple crates in this repo need Telemetry, consider adding a workspace
  `Cargo.toml` at the repo root to manage versions and run workspace-level tests.

Troubleshooting
---------------
- **Duplicate imports error**: If you see "the name `X` is defined multiple times", 
  check that you're not importing the same type twice in a module (e.g., two 
  `use std::sync::{Arc, Mutex};` statements). The compiler will indicate the 
  duplicate line number.

- **Unused variable warnings**: To suppress compiler warnings for intentional 
  unused parameters, prefix the variable name with underscore: 
  `fn send(&self, topic: &str, _payload: &[u8])`.

- **Unused import warnings**: Remove imports you're not using in that module. 
  If a protocol stub (MQTT/gRPC) doesn't reference a type, exclude it from the 
  use statement.

- If `cargo build` fails in CI with dependency errors, ensure the `Telemetry`
  `Cargo.toml` is valid and that required crates are published/available.

- If tests that use live external services fail in CI, either mark them as
  ignored or add setup steps and secrets in the workflow to provide required
  credentials and services.

Contact
-------
If something is unclear or you want a different integration pattern (workspace
member, binary wrapper, or explicit CI job for Telemetry), tell me and I will
update this document and the repository accordingly.
