# room619 - Modular Micro-Services Framework in Rust

A modern, safe, and performant micro-services framework built in **Rust**. This framework provides a robust foundation for building scalable, distributed systems with Rust's memory safety, zero-cost abstractions, and strong concurrency model.

## 🎯 Core Vision

- **Modular Architecture**: Service-based design with clear boundaries and independent deployability
- **High Performance**: Optimized for low-latency inter-service communication
- **Production Ready**: Built-in observability, error handling, and resilience patterns
- **Type-Safe APIs**: Leverage Rust's type system for safe service contracts
- **Cross-Platform**: Runs on Linux, Windows, macOS, and containerized environments

## 🏗️ Architecture

```
src/
├── lib.rs                    # Core framework entry point
├── service/                  # Service definition and lifecycle
│   ├── mod.rs              # Service trait and registry
│   ├── builder.rs          # Service builder pattern
│   └── context.rs          # Service execution context
├── rpc/                      # RPC and inter-service communication
│   ├── mod.rs              # RPC trait definitions
│   ├── grpc.rs             # gRPC implementation
│   ├── http.rs             # REST/HTTP implementation
│   └── codec.rs            # Message serialization
├── observability/           # Logging, metrics, tracing
│   ├── mod.rs              # Observability traits
│   ├── logging.rs          # Structured logging
│   ├── metrics.rs          # Metrics collection
│   └── tracing.rs          # Distributed tracing
├── resilience/             # Error handling and resilience
│   ├── mod.rs              # Resilience patterns
│   ├── circuit_breaker.rs  # Circuit breaker pattern
│   ├── retry.rs            # Retry with backoff
│   └── timeout.rs          # Timeout handling
└── middleware/             # Cross-cutting concerns
    ├── mod.rs              # Middleware chain
    ├── auth.rs             # Authentication/authorization
    ├── validation.rs       # Request validation
    └── rate_limit.rs       # Rate limiting
```

## 🚀 Key Features

### 1. **Service-Based Architecture**
Build independent, deployable services with:
- Clear service boundaries and contracts
- Service registry and discovery
- Hot-reload and dynamic service registration
- Versioned APIs for backward compatibility

### 2. **High-Performance Communication**
- **gRPC** — Type-safe, efficient RPC with streaming
- **REST/HTTP** — Standard web APIs with middleware support
- **Message Queues** — Asynchronous communication patterns
- Built-in serialization with protobuf and JSON

### 3. **Production-Ready Observability**
- **Structured Logging** — Context-aware, queryable logs
- **Metrics** — Built-in Prometheus-compatible metrics
- **Distributed Tracing** — Trace requests across services
- Health checks and service status monitoring

### 4. **Resilience & Error Handling**
- **Circuit Breaker** — Prevent cascading failures
- **Retry with Backoff** — Exponential backoff and jitter
- **Timeout Handling** — Bounded execution times
- **Graceful Degradation** — Fallback strategies

## 📋 Requirements

### Build
- **Rust**: 1.70+ (stable)
- **Cargo**: Latest
- **Protocol Buffers** (optional): For gRPC

### Runtime
- **Development**: Linux, Windows, macOS (x86_64, ARM64)
- **Production**: Kubernetes, Docker, or standalone binaries

## 🛠️ Getting Started

### Build
```bash
cargo build --release
```

### Run Tests
```bash
cargo test --all-features --verbose
cargo test --all-features --release
```

### Run a Service
```bash
# With debug logging
RUST_LOG=debug cargo run --release

# With specific feature
cargo run --release --features "grpc,metrics"
```

### Build Docker Image
```bash
docker build -t room619:latest .
docker run -p 50051:50051 room619:latest
```

## 🔐 Code Standards

All code must follow these Rust best practices:

### ❌ Forbidden Patterns
```rust
.unwrap() / .expect()       // Use ? operator instead
panic!() in production code // Use Result<T, E>
#[allow(clippy::*)]         // Fix warnings, don't suppress
unsafe {} without docs      // Document with // SAFETY:
```

### ✅ Required Patterns
```rust
operation()?                // Error propagation with ?
Err(error)?                 // Convert errors properly
pub fn op() -> Result<T, E> { }  // Return results
/// # Errors
/// Describes possible errors
pub fn op() -> Result<T, E> { }
```

### Documentation Requirements
```rust
/// Sends a request to the downstream service.
/// 
/// # Errors
/// Returns error if service is unavailable or request times out.
/// 
/// # Example
/// ```
/// let response = client.send(request).await?;
/// ```
pub async fn send(&self, request: Request) -> Result<Response, Error> { }
```

## 🔄 CI/CD Pipelines

### Continuous Integration (CI)
Runs on every push and pull request:
- ✅ Cargo check
- ✅ Rustfmt (code formatting)
- ✅ Clippy (linting)
- ✅ Unit & integration tests
- ✅ Cross-platform builds (Linux, Windows, macOS)
- ✅ Docker image builds
- ✅ Performance benchmarks

### Continuous Deployment (CD)
Triggered on version tags (`v*`):
- 📦 Build release binaries
- 🐳 Push Docker images to registry
- 📤 Upload artifacts to GitHub releases
- 🏷️ Create release notes

### Security & Quality Checks
Runs on every push and pull request:
- 🔍 Dependency vulnerability scanning (RustSec)
- 📜 License compliance (Cargo Deny)
- 🐛 Undefined behavior detection (Miri)
- 📊 Code coverage reporting
- ⚡ Performance regression detection

## 📦 Dependencies

### Core
- `tokio` — Async runtime
- `tonic` — gRPC framework
- `prost` — Protocol Buffers serialization
- `serde` — Serialization framework

### Observability
- `tracing` — Structured logging and tracing
- `prometheus` — Metrics collection
- `opentelemetry` — Distributed tracing

### Resilience
- `futures` — Async utilities
- `async-retry` — Retry logic
- `tower` — Service middleware

### Testing
- `mockall` — Mock generation
- `testcontainers` — Docker test containers

## 📊 Inter-Service Communication

### Supported Protocols
- **gRPC** — Type-safe, high-performance RPC with streaming
- **REST/HTTP** — Standard web APIs with JSON/protobuf
- **Message Queues** — Async, decoupled communication

### Service Contract Example
```rust
#[derive(Serialize, Deserialize)]
pub struct ServiceRequest {
    pub user_id: String,
    pub action: String,
}

pub struct ServiceResponse {
    pub status: String,
    pub data: Option<Vec<u8>>,
}
```

## 🧪 Testing Strategy

### Unit Tests
Located in source files with `#[cfg(test)]` modules:
```bash
cargo test --lib
```

### Integration Tests
Located in `tests/` directory with testcontainers:
```bash
cargo test --test '*'
```

### Performance Benchmarks
Criterion-based benchmarks:
```bash
cargo bench
```

## 🎓 Team Roles & Responsibilities

- **Platform Architect** — Framework design, service contracts
- **Backend Developer** — Service implementation, business logic
- **DevOps Engineer** — CI/CD, containerization, deployment
- **QA/Tester** — Integration testing, performance validation

## 📚 Documentation

- **Architecture**: See `docs/ARCHITECTURE.md`
- **API Reference**: Generated via `cargo doc --open`
- **Development Guide**: See `CONTRIBUTING.md`
- **Coding Standards**: See `.github/Infrastructure/copilot-instructions.md`

## 🚢 Deployment

### Local Development
```bash
cargo run --release --all-features
```

### Docker Deployment
```bash
docker build -t room619:latest .
docker run -p 50051:50051 -e RUST_LOG=info room619:latest
```

### Kubernetes Deployment
```bash
kubectl apply -f k8s/deployment.yaml
```

## 📝 Commit Convention

- `feat:` New feature or service
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Test improvements
- `refactor:` Code restructuring
- `perf:` Performance optimization
- `ci:` CI/CD updates

Example:
```
feat: add user service with gRPC API

- Implement user CRUD operations
- Add authentication middleware
- Include integration tests
```

## 🤝 Contributing

1. **Fork** and create a feature branch (`git checkout -b feature/your-feature`)
2. **Commit** with descriptive messages following conventions
3. **Push** to your fork and open a **Pull Request**
4. Ensure all CI checks pass and code follows hard real-time standards

## 📄 License

[Add your license here]

## 🎉 Project Milestones

Target deliverables:
- ✅ Core framework with service registry and RPC layer
- ✅ Observability (logging, metrics, tracing)
- ✅ Resilience patterns (circuit breaker, retry, timeout)
- ✅ Example services (User, Auth, Product)
- ✅ Comprehensive documentation and API reference
- ✅ Production-ready Docker/Kubernetes support
