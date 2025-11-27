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
   use telemetry::{TelemetryMessage, TelemetrySink};

   pub struct MySink;

   impl TelemetrySink for MySink {
       fn send(&self, topic: &str, payload: &[u8]) -> Result<(), String> {
           // encode / transmit using MQTT / gRPC / socket, etc.
           Ok(())
       }
   }

   // Example: create message and send
   let payload = serde_json::json!({"temp": 24.5});
   let msg = TelemetryMessage::new("sensors/temp", payload);
   let sink = MySink;
   sink.send(&msg.topic, msg.to_json().as_bytes())?;
   ```

API reference (high level)
--------------------------
- `TelemetryMessage`:
  - fields: `topic: String`, `payload: serde_json::Value`
  - helpers: `TelemetryMessage::new(topic, payload)`, `to_json()`
- `TelemetrySink` trait:
  - `fn send(&self, topic: &str, payload: &[u8]) -> Result<(), String>`
- `MockSink`:
  - Useful for tests: it prints the outbound payload to stdout and returns Ok.

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
