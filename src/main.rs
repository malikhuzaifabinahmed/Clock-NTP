//! # NTP-Synchronized Clock Application
//!
//! Command-line application for displaying NTP-synchronized time.

use chrono::Duration;
use clap::Parser;
use clock::Clock;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Command-line arguments for the NTP clock application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// NTP update interval in seconds
    #[arg(short, long, default_value_t = 10)]
    interval: u64,

    /// Display interval in seconds
    #[arg(short, long, default_value_t = 1)]
    display_interval: u64,

    /// Custom NTP server (can be specified multiple times)
    #[arg(short, long)]
    server: Vec<String>,

    /// Timezone offset in hours (e.g., -5 for EST, 0 for UTC)
    #[arg(short, long, default_value_t = 0)]
    timezone_offset: i32,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Show statistics
    #[arg(long)]
    show_stats: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    info!("Starting NTP-synchronized clock");
    info!(
        "Configuration: interval={}s, display_interval={}s, timezone_offset={}h",
        args.interval, args.display_interval, args.timezone_offset
    );

    let ntp_servers = if args.server.is_empty() {
        None
    } else {
        Some(args.server.clone())
    };

    let clock = Arc::new(Mutex::new(Clock::new(ntp_servers)));
    let shutdown = Arc::new(AtomicBool::new(false));

    // Set up Ctrl+C handler
    let shutdown_clone = Arc::clone(&shutdown);
    ctrlc::set_handler(move || {
        info!("Received shutdown signal");
        shutdown_clone.store(true, Ordering::Relaxed);
    })?;

    Clock::start(Arc::clone(&clock), args.interval, Arc::clone(&shutdown));

    let timezone_offset = Duration::hours(args.timezone_offset as i64);

    while !shutdown.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_secs(args.display_interval));
        let clock_guard = clock.lock().unwrap();
        let current_time = clock_guard.get_current_time();
        let adjusted_time = current_time + timezone_offset;

        if args.show_stats {
            let stats = clock_guard.get_stats();
            println!(
                "Time (UTC{:+}): {} | Syncs: {}/{} ({:.1}% success)",
                args.timezone_offset,
                adjusted_time.format("%Y-%m-%d %H:%M:%S"),
                stats.successful_syncs,
                stats.total_attempts,
                stats.success_rate()
            );
        } else {
            println!(
                "Time (UTC{:+}): {}",
                args.timezone_offset,
                adjusted_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    info!("Shutting down gracefully");
    Ok(())
}
