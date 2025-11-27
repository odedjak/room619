//! Telemetry crate - thin template for the Telemetry Engineer
//!
//! This crate provides a small, testable telemetry abstraction suitable for
//! the `room619` project. It abstracts the transport layer (MQTT, gRPC, custom
//! binary protocol) behind the `TelemetrySink` trait, allowing other modules
//! to send structured telemetry data without coupling to a specific protocol.
//!
//! **Why traits?** A trait-based design lets each protocol (MQTT, gRPC, etc.)
//! provide its own `TelemetrySink` implementation, and allows tests to inject
//! mock or in-memory sinks without external dependencies.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ============================================================================
// Error type
// ============================================================================

/// Errors that can occur when sending telemetry data.
#[derive(Debug, Clone)]
pub struct TelemetryError {
    /// Human-readable error message.
    pub message: String,
}

impl TelemetryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TelemetryError: {}", self.message)
    }
}

impl std::error::Error for TelemetryError {}

/// Convenience type alias for telemetry operations.
pub type TelemetryResult<T> = Result<T, TelemetryError>;

// ============================================================================
// Message type
// ============================================================================

/// Basic telemetry message structure used for examples and tests.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryMessage {
    pub topic: String,
    pub payload: serde_json::Value,
}

impl TelemetryMessage {
    /// Create a new telemetry message.
    ///
    /// The topic should follow a hierarchical path convention (e.g., `sensors/temp`).
    /// The payload is a JSON value allowing flexible data structures.
    pub fn new(topic: impl Into<String>, payload: serde_json::Value) -> Self {
        TelemetryMessage {
            topic: topic.into(),
            payload,
        }
    }

    /// Serialize message to a JSON string.
    ///
    /// This is a convenience method for protocol implementations that want JSON
    /// transmission; other implementations may use custom encoding.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("serialization should succeed")
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn telemetry_message_serializes_and_deserializes() {
        let payload = serde_json::json!({ "temp": 23.5, "unit": "C" });
        let msg = TelemetryMessage::new("sensors/temperature", payload.clone());

        let s = msg.to_json();
        let parsed: TelemetryMessage = serde_json::from_str(&s).expect("parse should work");

        assert_eq!(parsed.topic, "sensors/temperature");
        assert_eq!(parsed.payload, payload);
    }

    #[test]
    fn telemetry_message_new_has_topic() {
        let payload = serde_json::json!(null);
        let msg = TelemetryMessage::new("a/topic", payload);
        assert_eq!(msg.topic, "a/topic");
    }
}
pub trait TelemetrySink: Send + Sync {
    /// Send a telemetry payload to a named topic/channel.
    ///
    /// **Why bytes?** Byte slices are protocol-agnostic; senders can pre-encode
    /// using JSON, msgpack, protobuf, or custom binary without the trait
    /// knowing about them.
    ///
    /// Returns `Ok(())` on success or `TelemetryError` on transport failure.
    fn send(&self, topic: &str, payload: &[u8]) -> TelemetryResult<()>;
}

/// A small mock sink used for local testing and CI.
pub struct MockSink;

impl TelemetrySink for MockSink {
    fn send(&self, topic: &str, _payload: &[u8]) -> TelemetryResult<()> {
        // For testing: print to stdout so developers can see what was sent.
        println!("MockSink sending to '{}': {:?}", topic, _payload);
        Ok(())
    }
}

/// A client that sends structured `TelemetryMessage` instances through a
/// `TelemetrySink`. This separates message construction from the transport.
pub struct TelemetryClient {
    sink: Arc<dyn TelemetrySink>,
}

impl TelemetryClient {
    /// Create a new client that uses the provided sink.
    ///
    /// **Why Arc?** Multiple threads/tasks may need to send telemetry concurrently.
    /// An Arc allows safe, cheap cloning of the client or direct sharing.
    pub fn new(sink: Arc<dyn TelemetrySink>) -> Self {
        Self { sink }
    }

    /// Send a structured telemetry message. The default serialization is JSON.
    ///
    /// This is the primary API for most use cases: create a `TelemetryMessage`,
    /// then call this to serialize and transmit it.
    pub fn send_message(&self, msg: &TelemetryMessage) -> TelemetryResult<()> {
        let payload = msg.to_json();
        self.sink.send(&msg.topic, payload.as_bytes())
    }

    /// Send arbitrary binary payload to a topic.
    ///
    /// Use this when you have pre-encoded data (msgpack, protobuf, custom binary)
    /// that should not be re-encoded by `TelemetryMessage`.
    pub fn send_binary(&self, topic: &str, data: &[u8]) -> TelemetryResult<()> {
        self.sink.send(topic, data)
    }
}

/// An in-memory sink useful for testing and local inspection.
pub struct InMemorySink {
    pub records: Arc<Mutex<Vec<(String, Vec<u8>)>>>,
}

impl InMemorySink {
    /// Create a new in-memory sink with no records.
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get a cloneable `Arc` to the internal storage.
    ///
    /// Useful in tests to inspect recorded messages without ownership issues.
    pub fn records_arc(&self) -> Arc<Mutex<Vec<(String, Vec<u8>)>>> {
        Arc::clone(&self.records)
    }
}

impl Default for InMemorySink {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetrySink for InMemorySink {
    fn send(&self, topic: &str, payload: &[u8]) -> TelemetryResult<()> {
        let mut lock = self
            .records
            .lock()
            .map_err(|e| TelemetryError::new(format!("lock poisoned: {}", e)))?;
        lock.push((topic.to_string(), payload.to_vec()));
        Ok(())
    }
}

#[cfg(test)]
mod sink_tests {
    use super::*;

    #[test]
    fn mock_sink_send_ok() {
        let sink = MockSink;
        let res = sink.send("telemetry/health", b"alive");
        assert!(res.is_ok());
    }

    #[test]
    fn in_memory_sink_records_messages() {
        let sink = InMemorySink::new();
        let records_arc = sink.records_arc();
        let client = TelemetryClient::new(Arc::new(sink));

        let payload = serde_json::json!({ "status": "ok" });
        let msg = TelemetryMessage::new("svc/status", payload.clone());

        // send the message
        client.send_message(&msg).expect("send should succeed");

        // inspect recorded messages
        let records = records_arc.lock().expect("lock");
        assert_eq!(records.len(), 1);
        let (topic, bytes) = &records[0];
        assert_eq!(topic, "svc/status");
        let parsed: TelemetryMessage = serde_json::from_slice(bytes).expect("valid json");
        assert_eq!(parsed.topic, "svc/status");
        assert_eq!(parsed.payload, payload);
    }

    #[test]
    fn send_binary_via_client() {
        let sink = InMemorySink::new();
        let records_arc = sink.records_arc();
        let client = TelemetryClient::new(Arc::new(sink));

        let data = [1u8, 2, 3, 4];
        client
            .send_binary("binary/topic", &data)
            .expect("send binary");

        let records = records_arc.lock().expect("lock");
        assert_eq!(records.len(), 1);
        let (topic, bytes) = &records[0];
        assert_eq!(topic, "binary/topic");
        assert_eq!(bytes.as_slice(), &data);
    }

    #[test]
    fn telemetry_error_display() {
        let err = TelemetryError::new("transport failed");
        let msg = err.to_string();
        assert!(msg.contains("transport failed"));
    }

    #[test]
    fn client_propagates_sink_errors() {
        // MockSink always returns Ok, but this documents the error path.
        let sink = Arc::new(MockSink);
        let client = TelemetryClient::new(sink);
        let payload = serde_json::json!({ "test": true });
        let msg = TelemetryMessage::new("test/topic", payload);

        let result = client.send_message(&msg);
        assert!(result.is_ok());
    }

    #[test]
    fn in_memory_sink_default() {
        let sink = InMemorySink::default();
        let records_arc = sink.records_arc();
        let client = TelemetryClient::new(Arc::new(sink));

        let data = b"test";
        client.send_binary("test", data).expect("send");

        let records = records_arc.lock().expect("lock");
        assert_eq!(records.len(), 1);
    }
}

// ============================================================================
// Protocol implementations (feature-gated)
// ============================================================================

#[cfg(feature = "mqtt")]
pub mod mqtt {
    //! MQTT transport for telemetry data.
    //!
    //! **Why feature-gated?** Not all deployments need MQTT; gating reduces
    //! binary size and avoids pulling in heavy dependencies.
    //! Enable with `features = ["mqtt"]` in Cargo.toml.

    use super::{TelemetryResult, TelemetrySink};

    /// MQTT sink stub. A real implementation would:
    /// - Connect to an MQTT broker (mosquitto, AWS IoT, etc.)
    /// - Publish messages to broker topics
    /// - Handle reconnection and QoS
    pub struct MqttSink {
        /// Placeholder for MQTT client (would be paho_mqtt::AsyncClient, etc.)
        pub broker_url: String,
    }

    impl MqttSink {
        /// Create a new MQTT sink pointing to a broker.
        pub fn new(broker_url: impl Into<String>) -> Self {
            Self {
                broker_url: broker_url.into(),
            }
        }
    }

    impl TelemetrySink for MqttSink {
        fn send(&self, topic: &str, _payload: &[u8]) -> TelemetryResult<()> {
            // TODO: Implement MQTT publish
            // For now, this is a stub that logs intent.
            log::debug!("MQTT: would publish to {} @ {}", topic, self.broker_url);
            Ok(())
        }
    }
}

#[cfg(feature = "grpc")]
pub mod grpc {
    //! gRPC transport for telemetry data.
    //!
    //! **Why feature-gated?** gRPC adds protobuf/networking complexity;
    //! only enable if your deployment uses gRPC for telemetry.

    use super::{TelemetryResult, TelemetrySink};

    /// gRPC sink stub. A real implementation would:
    /// - Connect to a gRPC service
    /// - Send telemetry as gRPC messages
    /// - Handle service availability and retries
    pub struct GrpcSink {
        /// Placeholder for gRPC client endpoint
        pub endpoint: String,
    }

    impl GrpcSink {
        /// Create a new gRPC sink pointing to a service endpoint.
        pub fn new(endpoint: impl Into<String>) -> Self {
            Self {
                endpoint: endpoint.into(),
            }
        }
    }

    impl TelemetrySink for GrpcSink {
        fn send(&self, topic: &str, _payload: &[u8]) -> TelemetryResult<()> {
            // TODO: Implement gRPC send
            // For now, this is a stub that logs intent.
            log::debug!("gRPC: would send to {} @ {}", topic, self.endpoint);
            Ok(())
        }
    }
}
