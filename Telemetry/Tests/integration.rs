//! Integration tests for the Telemetry crate.
//!
//! These tests verify the core behavior of the telemetry abstraction:
//! - Message serialization and deserialization
//! - Sending via different sink implementations
//! - Error propagation

use std::sync::Arc;
use telemetry::{InMemorySink, TelemetryClient, TelemetryMessage, TelemetrySink};

#[test]
fn integration_full_message_flow() {
    // Setup: Create a client with an in-memory sink
    let sink = InMemorySink::new();
    let records_arc = sink.records_arc();
    let client = TelemetryClient::new(Arc::new(sink));

    // Act: Create and send a structured message
    let payload = serde_json::json!({
        "sensor_id": "temp_01",
        "value": 24.5,
        "unit": "celsius",
        "timestamp": 1234567890
    });
    let msg = TelemetryMessage::new("sensors/temperature", payload.clone());

    // Assert: Send succeeds
    let result = client.send_message(&msg);
    assert!(result.is_ok(), "send_message should succeed");

    // Assert: Message was recorded
    let records = records_arc.lock().expect("lock should succeed");
    assert_eq!(records.len(), 1, "exactly one message should be recorded");

    let (topic, bytes) = &records[0];
    assert_eq!(topic, "sensors/temperature");

    // Assert: Recorded bytes deserialize back to the original message
    let parsed: TelemetryMessage = serde_json::from_slice(bytes).expect("valid JSON");
    assert_eq!(parsed.topic, "sensors/temperature");
    assert_eq!(parsed.payload, payload);
}

#[test]
fn integration_multiple_sinks_independent() {
    // Verify that multiple clients with different sinks don't interfere
    let sink1 = InMemorySink::new();
    let records1 = sink1.records_arc();
    let client1 = TelemetryClient::new(Arc::new(sink1));

    let sink2 = InMemorySink::new();
    let records2 = sink2.records_arc();
    let client2 = TelemetryClient::new(Arc::new(sink2));

    // Each client sends to its own sink
    let msg1 = TelemetryMessage::new("topic1", serde_json::json!({ "id": 1 }));
    let msg2 = TelemetryMessage::new("topic2", serde_json::json!({ "id": 2 }));

    client1.send_message(&msg1).expect("send to client1");
    client2.send_message(&msg2).expect("send to client2");

    // Verify isolation
    let recs1 = records1.lock().expect("lock");
    let recs2 = records2.lock().expect("lock");

    assert_eq!(recs1.len(), 1);
    assert_eq!(recs2.len(), 1);
    assert_eq!(recs1[0].0, "topic1");
    assert_eq!(recs2[0].0, "topic2");
}

#[test]
fn integration_binary_and_json_coexist() {
    // Verify that binary and JSON sends can be mixed in the same sink
    let sink = InMemorySink::new();
    let records_arc = sink.records_arc();
    let client = TelemetryClient::new(Arc::new(sink));

    // Send JSON message
    let json_msg = TelemetryMessage::new("data/json", serde_json::json!({ "type": "json" }));
    client.send_message(&json_msg).expect("send JSON");

    // Send binary data
    let binary_data = b"\x00\x01\x02\x03";
    client
        .send_binary("data/binary", binary_data)
        .expect("send binary");

    // Verify both are recorded
    let records = records_arc.lock().expect("lock");
    assert_eq!(records.len(), 2);

    // First should be JSON
    assert_eq!(records[0].0, "data/json");
    assert!(String::from_utf8_lossy(&records[0].1).contains("json"));

    // Second should be binary
    assert_eq!(records[1].0, "data/binary");
    assert_eq!(records[1].1, binary_data);
}

#[test]
fn integration_concurrent_sends() {
    // Verify that InMemorySink is thread-safe (Arc + Mutex)
    let sink = InMemorySink::new();
    let records_arc = sink.records_arc();
    let client = Arc::new(TelemetryClient::new(Arc::new(sink)));

    let mut handles = vec![];

    for i in 0..10 {
        let client = Arc::clone(&client);
        let handle = std::thread::spawn(move || {
            let msg = TelemetryMessage::new(
                &format!("thread/{}", i),
                serde_json::json!({ "thread_id": i }),
            );
            client.send_message(&msg).expect("send in thread");
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().expect("thread should join");
    }

    // Verify all messages were recorded
    let records = records_arc.lock().expect("lock");
    assert_eq!(records.len(), 10, "all 10 messages should be recorded");

    // Verify no duplicates and correct topics
    for i in 0..10 {
        assert_eq!(records[i].0, format!("thread/{}", i));
    }
}
