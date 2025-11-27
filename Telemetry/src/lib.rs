//! Telemetry crate - thin template for the Telemetry Engineer
//!
//! This crate provides a small, testable telemetry abstraction suitable for
//! the `room619` project. Implementations can use MQTT, gRPC, or a custom
//! binary protocol behind the `TelemetrySink` trait.

use serde::{Deserialize, Serialize};

/// Basic telemetry message structure used for examples and tests.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryMessage {
    pub topic: String,
    pub payload: serde_json::Value,
}

impl TelemetryMessage {
    /// Create a new telemetry message
    pub fn new(topic: impl Into<String>, payload: serde_json::Value) -> Self {
        TelemetryMessage {
            topic: topic.into(),
            payload,
        }
    }

    /// Serialize message to a JSON string.
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
pub trait TelemetrySink {
    /// Send a telemetry payload to a named topic/channel.
    ///
    /// Implementations should return `Ok(())` on success or an `Err` with a
    /// short description on failure.
    fn send(&self, topic: &str, payload: &[u8]) -> Result<(), String>;
}

/// A small mock sink used for local testing and CI.
pub struct MockSink;

impl TelemetrySink for MockSink {
    fn send(&self, topic: &str, payload: &[u8]) -> Result<(), String> {
        // In a real sink, this would encode and transmit the payload.
        // For the template we keep it observable via stdout for tests.
        println!("MockSink sending to '{}': {:?}", topic, payload);
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
}
