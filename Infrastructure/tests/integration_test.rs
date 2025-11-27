#[cfg(test)]
mod tests {
    use room619_core::platform::PlatformAbstraction;
    use room619_core::scheduler::{Scheduler, Task};
    use room619_core::timer::Timer;

    #[test]
    fn test_desktop_platform() {
        let mut platform = room619_core::platform::DesktopPlatform;
        assert!(platform.start().is_ok());
        assert_eq!(platform.platform_name(), "Desktop (Tokio-based)");
        assert!(platform.stop().is_ok());
    }

    #[test]
    fn test_desktop_timer() {
        let mut timer = room619_core::timer::DesktopTimer::new();

        assert!(!timer.is_running());
        assert!(timer.start().is_ok());
        assert!(timer.is_running());

        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);

        assert!(timer.stop().is_ok());
        assert!(!timer.is_running());
    }

    #[test]
    fn test_scheduler() {
        let mut scheduler = room619_core::scheduler::DefaultScheduler::new();

        let task = Task {
            id: 1,
            priority: 10,
            period_ms: 100,
        };

        assert!(scheduler.add_task(task).is_ok());
        assert!(scheduler.run().is_ok());
        assert!(scheduler.remove_task(1).is_ok());
    }
}
