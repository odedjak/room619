//! room619 Core - Modular Micro-Services Framework
//!
//! This is the main entry point for the framework.

use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("🚀 Starting room619 Framework");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Print system information
    print_system_info();

    // TODO: Initialize platform abstraction layer
    // TODO: Start service registry
    // TODO: Launch core services

    info!("✅ Framework initialized successfully");

    // Keep the application running
    tokio::signal::ctrl_c().await?;
    info!("⏹️  Shutting down gracefully");

    Ok(())
}

/// Display system information
fn print_system_info() {
    info!("System Information:");
    info!("  Platform: {}", std::env::consts::OS);
    info!("  Architecture: {}", std::env::consts::ARCH);
    info!("  CPU Count: {}", num_cpus::get());
}
