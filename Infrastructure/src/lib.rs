//! Platform Abstraction Layer
//!
//! Provides trait-based abstractions for platform-specific implementations.

pub mod platform;
pub mod scheduler;
pub mod timer;

pub use platform::PlatformAbstraction;
pub use platform::PlatformError;
