//! Scheduler implementations
//!
//! Provides scheduling primitives for different platforms.

use crate::platform::PlatformError;

/// Task definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub priority: u8,
    pub period_ms: u32,
}

/// Scheduler trait
pub trait Scheduler {
    fn add_task(&mut self, task: Task) -> Result<(), PlatformError>;
    fn remove_task(&mut self, task_id: u32) -> Result<(), PlatformError>;
    fn run(&mut self) -> Result<(), PlatformError>;
}

/// Default scheduler implementation
pub struct DefaultScheduler {
    tasks: Vec<Task>,
}

impl DefaultScheduler {
    pub fn new() -> Self {
        DefaultScheduler { tasks: Vec::new() }
    }
}

impl Scheduler for DefaultScheduler {
    fn add_task(&mut self, task: Task) -> Result<(), PlatformError> {
        self.tasks.push(task);
        Ok(())
    }

    fn remove_task(&mut self, task_id: u32) -> Result<(), PlatformError> {
        self.tasks.retain(|t| t.id != task_id);
        Ok(())
    }

    fn run(&mut self) -> Result<(), PlatformError> {
        // TODO: Implement scheduling logic
        Ok(())
    }
}
