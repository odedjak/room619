//! Platform Abstraction Traits
//! 
//! Defines the core traits that all platform implementations must provide.

use std::time::Duration;

/// Platform abstraction trait
pub trait PlatformAbstraction: Send + Sync {
    fn platform_name(&self) -> &'static str;
    fn start(&mut self) -> Result<(), PlatformError>;
    fn stop(&mut self) -> Result<(), PlatformError>;
}

/// Platform error type
#[derive(Debug, Clone)]
pub enum PlatformError {
    InitializationFailed(String),
    OperationFailed(String),
    NotSupported(String),
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::InitializationFailed(msg) => write!(f, "Initialization failed: {}", msg),
            PlatformError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            PlatformError::NotSupported(msg) => write!(f, "Not supported: {}", msg),
        }
    }
}

impl std::error::Error for PlatformError {}

/// Timer backend trait
pub trait TimerBackend: Send + Sync {
    fn start(&mut self, duration: Duration) -> Result<(), PlatformError>;
    fn elapsed(&self) -> Duration;
    fn stop(&mut self) -> Result<(), PlatformError>;
}

/// Scheduler backend trait
pub trait SchedulerBackend: Send + Sync {
    fn schedule_task(&mut self, task_id: u32) -> Result<(), PlatformError>;
    fn yield_cpu(&self);
    fn current_task_id(&self) -> u32;
}

/// Default desktop platform implementation
pub struct DesktopPlatform;

impl PlatformAbstraction for DesktopPlatform {
    fn platform_name(&self) -> &'static str {
        "Desktop (Tokio-based)"
    }

    fn start(&mut self) -> Result<(), PlatformError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), PlatformError> {
        Ok(())
    }
}
