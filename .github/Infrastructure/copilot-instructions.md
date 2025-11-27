# Rust Hard Real-Time Programming: Copilot Instructions

## Non-Negotiable Rules

**❌ Never:**
```rust
.unwrap() / .expect()       // Use ? instead
vec![] in hot paths         // Stack alloc or SmallVec
HashMap / thread::spawn()   // Use BTreeMap / crossbeam channels
SystemTime                  // Use std::time::Instant
Mutex / RwLock              // Use atomics or lock-free structures
allocate / deallocate       // Pre-allocate or use stack
```

**✅ Always:**
```rust
operation()?                // Error propagation
let buf = [0u8; 256];      // Stack allocation
let (tx, rx) = crossbeam::channel::bounded(N);  // Bounded channels
std::sync::atomic::*        // Lock-free synchronization
```

## Core Principles

- **Safety**: Respect ownership. Use `&T`, `&mut T` appropriately.
- **Errors**: All `Result`/`Option` use `?` operator. No `.unwrap()`.
- **Hard Real-Time**: Zero allocations, GC, or unbounded operations in timing-critical sections.
- **Deterministic**: Lock-free primitives only. No mutexes/RwLocks. Message-passing with bounded channels.
- **Predictable Latency**: O(1) complexity, bounded worst-case execution time (WCET).

## Standards

- Code must pass `cargo check` on first generation without warnings
- Exhaustive pattern matching; let compiler catch errors
- Prefer iterator chains; zero-cost abstractions
- Document unsafe with `// SAFETY:` comments and memory barriers
- Public APIs return `Result<T, E>` with bounded execution time
- Document timing constraints: `/// WCET: ≤ 100µs` for each function

## Dependencies

**Allowed:** `crossbeam`, `parking_lot` (parking_lot is faster but still blocking), atomics, `tokio` with timeout bounds, `heapless`, minimal pure-Rust crates  
**Forbidden:** C/C++ FFI, unvetted unsafe, unbounded async, `Mutex`/`RwLock` in hard real-time paths, allocating crates

**Hard Real-Time Safe:** Use only `crossbeam::atomic`, `std::sync::atomic`, bounded channels. No thread::spawn without CPU pinning.

## Doc Template

```rust
/// Operation description.
/// # Real-Time Guarantees
/// WCET: O(1), ≤ 100µs. Zero allocations.
/// # Errors
/// Returns `Err` if timeout or resource exhausted.
pub fn operation() -> Result<T, E> { }
```

## Validate

```bash
cargo check              # Must pass, no warnings
cargo fmt --check; cargo clippy
RUST_BACKTRACE=1 cargo test
# For hard real-time, measure and verify WCET with profiler
```

## Watch For

- Lifetime mismatches, trait objects needing `'static`, recursive borrows
- Hidden allocations: `.clone()`, `format!()`, method chaining, string operations
- Unbounded recursion: use iteration in hot paths to avoid stack overflow
- Context switches, page faults, memory fragmentation
- Use profilers: `perf`, `flamegraph`, cycle counters for latency verification

## Cross-Platform / Platform Independence

- Use `std` abstractions (threads, channels, timers) for portability
- Isolate platform-specific code with `#[cfg(target_os = "windows")]`, `#[cfg(target_os = "linux")]`, etc.
- Prefer `std::time::Instant` over OS-specific timing APIs
- Use `crossbeam` for cross-platform concurrency—works on all Tier-1 platforms
- For CPU pinning/scheduling: use platform-specific `#[cfg]` blocks with fallback
- Test on Linux, macOS, Windows; verify WCET determinism on each platform