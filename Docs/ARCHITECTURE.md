# Architecture: Modular Micro-Services Framework in Rust

## Executive Summary

room619 is a modern micro-services framework built in Rust for building scalable, distributed systems. It provides abstractions for service communication, resilience, and observability while leveraging Rust's type safety and performance characteristics.

## Design Principles

### 1. **Modularity & Loose Coupling**
- Each service is independently deployable and testable
- Services communicate through well-defined APIs (gRPC, REST)
- Clear separation of concerns with trait-based interfaces
- Minimal dependencies between services

### 2. **High Performance & Low Latency**
- Async-first design using Tokio runtime
- Optimized serialization with Protocol Buffers
- Connection pooling and request batching
- Zero-copy message passing where possible

### 3. **Observability & Debuggability**
- Structured logging with context propagation
- Distributed tracing across service boundaries
- Metrics collection for performance monitoring
- Health checks and service status reporting

### 4. **Resilience & Fault Tolerance**
- Circuit breaker pattern for failure isolation
- Retry logic with exponential backoff
- Timeout handling for bounded latency
- Graceful degradation with fallback strategies

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

### 1. **Service Layer** (`src/service/`)

**Purpose**: Service lifecycle management and registration

**Key Traits**:
```rust
pub trait Service: Send + Sync {
    async fn start(&mut self) -> Result<(), ServiceError>;
    async fn stop(&mut self) -> Result<(), ServiceError>;
    fn health_check(&self) -> HealthStatus;
}

pub struct ServiceContext {
    pub name: String,
    pub version: String,
    pub logger: Logger,
    pub metrics: MetricsCollector,
}
```

**Files**:
- `mod.rs` — Service trait and lifecycle
- `builder.rs` — Service construction with configuration
- `context.rs` — Execution context and shared resources
- `registry.rs` — Service discovery and registration

### 2. **RPC Layer** (`src/rpc/`)

**Purpose**: Inter-service communication abstraction

**Key Traits**:
```rust
pub trait RpcClient: Send + Sync {
    async fn call(&self, request: &Request) -> Result<Response, RpcError>;
}

pub struct RpcConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub max_retries: u32,
}
```

**Supported Protocols**:
- **gRPC** — Type-safe, efficient streaming RPC
- **HTTP/REST** — Standard web APIs
- **Message Queues** — Async, decoupled communication

**Files**:
- `mod.rs` — RPC client/server traits
- `grpc.rs` — gRPC implementation with Tonic
- `http.rs` — REST/HTTP implementation with Axum
- `codec.rs` — Message serialization (Protobuf, JSON)

### 3. **Observability Layer** (`src/observability/`)

**Purpose**: Logging, metrics, and tracing

**Key Components**:

#### Structured Logging
```rust
pub trait Logger: Send + Sync {
    fn log(&self, level: Level, message: &str, context: &Context);
}
```

#### Metrics Collection
```rust
pub trait MetricsCollector: Send + Sync {
    fn counter(&self, name: &str) -> u64;
    fn histogram(&self, name: &str) -> Histogram;
    fn gauge(&self, name: &str) -> f64;
}
```

#### Distributed Tracing
```rust
pub trait Tracer: Send + Sync {
    fn span(&self, name: &str) -> Span;
}
```

**Files**:
- `mod.rs` — Observability traits
- `logging.rs` — Structured logging with tracing-rs
- `metrics.rs` — Prometheus-compatible metrics
- `tracing.rs` — OpenTelemetry integration

### 4. **Resilience Layer** (`src/resilience/`)

**Purpose**: Error handling and failure recovery

**Patterns**:

#### Circuit Breaker
```rust
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failure_threshold: u32,
    success_threshold: u32,
}
```

#### Retry with Backoff
```rust
pub struct RetryPolicy {
    max_retries: u32,
    backoff_strategy: BackoffStrategy,
    jitter: bool,
}
```

#### Timeout Management
```rust
pub async fn with_timeout<F, T>(
    future: F,
    timeout: Duration,
) -> Result<T, TimeoutError> { }
```

**Files**:
- `mod.rs` — Resilience trait definitions
- `circuit_breaker.rs` — Circuit breaker pattern
- `retry.rs` — Retry logic with exponential backoff
- `timeout.rs` — Timeout and deadline handling
- `fallback.rs` — Fallback strategies

### 5. **Middleware Layer** (`src/middleware/`)

**Purpose**: Cross-cutting concerns

**Examples**:

#### Authentication
```rust
pub trait AuthMiddleware {
    async fn authenticate(&self, request: &Request) -> Result<Identity, AuthError>;
}
```

#### Rate Limiting
```rust
pub struct RateLimiter {
    requests_per_second: u32,
}
```

#### Request Validation
```rust
pub trait Validator {
    fn validate(&self, request: &Request) -> Result<(), ValidationError>;
}
```

**Files**:
- `mod.rs` — Middleware chain
- `auth.rs` — Authentication and authorization
- `rate_limit.rs` — Rate limiting
- `validation.rs` — Request validation

## Data Flow

### Typical Request Flow

```
1. Client sends request to API Gateway
   ↓
2. Gateway routes to appropriate service
   ↓
3. Service receives request with distributed tracing
   ↓
4. Authentication middleware validates identity
   ↓
5. Rate limiter checks quotas
   ↓
6. Validation middleware checks request format
   ↓
7. Service business logic executes
   ↓
8. Optional: Call downstream services (with circuit breaker)
   ↓
9. Metrics and traces recorded
   ↓
10. Response sent back to client
```

### Service-to-Service Communication

```
Service A                   Service B
   │                           │
   ├─ Create RPC Client ──────┐│
   │                          ││
   ├─ Add Auth Headers ───────┤├─ Authenticate
   │                          ││
   ├─ Add Trace Context ──────┤├─ Extract Trace
   │                          ││
   ├─ Send with Timeout ──────┤├─ Process Request
   │                          ││
   ├─ Circuit Breaker ◄───────┤├─ Send Response
   │                          ││
   └─ Record Metrics ────────┬┘│
                            └─┘
```

## Error Handling

### Result-Based Error Handling

All fallible operations return `Result<T, E>`:

```rust
pub enum ServiceError {
    NotFound(String),
    InvalidRequest(String),
    Timeout,
    Internal(String),
}

pub async fn get_user(id: &str) -> Result<User, ServiceError> {
    // Implementation using ? operator
}
```

### Error Propagation

- Use `?` operator for automatic error conversion and propagation
- Never use `.unwrap()` or `.expect()` in production code
- Convert errors to appropriate HTTP status codes at API boundaries

## Configuration & Features

### Compile-Time Configuration
```toml
[features]
default = ["grpc", "logging"]
grpc = ["tonic", "prost"]
http = ["axum", "serde_json"]
logging = ["tracing", "tracing-subscriber"]
metrics = ["prometheus"]
```

### Runtime Configuration
Environment variables:
```bash
RUST_LOG=info              # Logging level
SERVICE_NAME=user-service  # Service identifier
SERVICE_PORT=50051         # gRPC port
HTTP_PORT=8080             # HTTP port
DATABASE_URL=...           # Database connection
```

## Concurrency Model

### Async-First with Tokio

All I/O operations are async:
```rust
#[tokio::main]
async fn main() {
    let service = UserService::new().await?;
    service.start().await?;
}
```

### No Blocking Operations in Async Context

- No synchronous I/O (use `tokio::fs`, `tokio::net`)
- No blocking locks (use `tokio::sync::Mutex`)
- No CPU-bound operations that block for long

## Performance Characteristics

| Operation | Latency Target | Notes |
|-----------|---------|-------|
| Service startup | < 5s | Cold start |
| RPC request | < 100ms | p99 latency |
| Metric write | < 1ms | Non-blocking |
| Log write | < 10ms | Buffered |
| Trace collection | < 2ms | Sampling supported |

## Platform Support

### Development
- Linux (x86_64, ARM64)
- Windows (x86_64)
- macOS (x86_64, ARM64)

### Deployment
- Kubernetes (preferred)
- Docker containers
- Standalone binaries on Linux servers
