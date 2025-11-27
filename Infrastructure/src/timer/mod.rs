//! Timer implementations
//! 
//! Provides timing primitives for different platforms.

use std::time::{Duration, Instant};
use crate::platform::PlatformError;

/// Timer trait
pub trait Timer {
    fn start(&mut self) -> Result<(), PlatformError>;
    fn elapsed(&self) -> Duration;
    fn stop(&mut self) -> Result<(), PlatformError>;
    fn is_running(&self) -> bool;
}

/// Desktop timer implementation
pub struct DesktopTimer {
    start_time: Option<Instant>,
}

impl DesktopTimer {
    pub fn new() -> Self {
        DesktopTimer {
            start_time: None,
        }
    }
}

impl Timer for DesktopTimer {
    fn start(&mut self) -> Result<(), PlatformError> {
        self.start_time = Some(Instant::now());
        Ok(())
    }

    fn elapsed(&self) -> Duration {
        self.start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    fn stop(&mut self) -> Result<(), PlatformError> {
        self.start_time = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.start_time.is_some()
    }
}
