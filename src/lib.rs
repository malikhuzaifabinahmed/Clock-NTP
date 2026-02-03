//! # NTP-Synchronized Clock Library
//!
//! This library provides functionality to synchronize with NTP (Network Time Protocol) servers
//! to maintain accurate time. It periodically fetches time from configured NTP servers and
//! provides real-time clock updates.

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};
use log::{error, info, warn};
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NATIVE: NaiveDateTime = NaiveDate::from_ymd_opt(2000, 1, 1)
    .unwrap()
    .and_hms_opt(0, 0, 0)
    .unwrap();

/// Default fallback time (January 1, 2000)
pub const DEFAULT: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(NATIVE, Utc);

/// Statistics for NTP synchronization
#[derive(Debug, Default, Clone)]
pub struct SyncStats {
    pub total_attempts: u64,
    pub successful_syncs: u64,
    pub failed_syncs: u64,
}

impl SyncStats {
    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.successful_syncs as f64 / self.total_attempts as f64) * 100.0
        }
    }
}

/// Main Clock structure that maintains synchronized time
pub struct Clock {
    latest_time_ntp: Option<DateTime<Utc>>,
    latest_time: DateTime<Utc>,
    pub latest_instant: Instant,
    pub ntp_servers: Vec<String>,
    stats: SyncStats,
}

impl Clock {
    /// Creates a new Clock instance with specified NTP servers
    pub fn new(ntp_servers: Option<Vec<String>>) -> Self {
        let servers = ntp_servers.unwrap_or_else(|| {
            vec![
                "time.google.com:123".to_string(),
                "time.cloudflare.com:123".to_string(),
                "pool.ntp.org:123".to_string(),
            ]
        });

        info!("Initializing clock with NTP servers: {:?}", servers);

        let latest_time_ntp = match Self::get_ntp_time(&servers) {
            Ok(time) => {
                info!("Successfully fetched initial NTP time: {}", time);
                Some(time)
            }
            Err(e) => {
                error!("NTP fetch failed, falling back to default time: {}", e);
                None
            }
        };

        Clock {
            latest_time_ntp,
            latest_time: latest_time_ntp.unwrap_or(DEFAULT),
            latest_instant: Instant::now(),
            ntp_servers: servers,
            stats: SyncStats::default(),
        }
    }

    /// Returns the duration elapsed since the last sync
    fn elapsed(&self) -> Duration {
        chrono::Duration::from_std(self.latest_instant.elapsed()).unwrap_or_else(|e| {
            warn!(
                "Failed to convert elapsed time: {}. Using zero duration.",
                e
            );
            Duration::zero()
        })
    }

    /// Fetches current time from NTP servers
    fn get_ntp_time(servers: &[String]) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        for server in servers {
            info!("Attempting to connect to NTP server: {}", server);
            match server.to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        match UdpSocket::bind("0.0.0.0:0") {
                            Ok(socket) => {
                                // Set timeouts
                                let _ = socket
                                    .set_read_timeout(Some(std::time::Duration::from_secs(3)));
                                let _ = socket
                                    .set_write_timeout(Some(std::time::Duration::from_secs(3)));

                                if socket.connect(addr).is_ok() {
                                    let mut buf = [0u8; 48];
                                    buf[0] = 0x1b; // NTP version 3, client mode

                                    if socket.send(&buf).is_ok() && socket.recv(&mut buf).is_ok() {
                                        let seconds = u32::from_be_bytes([
                                            buf[40], buf[41], buf[42], buf[43],
                                        ])
                                            as i64
                                            - 2_208_988_800;
                                        if let Some(dt) = Utc.timestamp_opt(seconds, 0).single() {
                                            info!(
                                                "Successfully retrieved time from {}: {}",
                                                server, dt
                                            );
                                            return Ok(dt);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Failed to bind socket: {}", e);
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to resolve {}: {}", server, e);
                    continue;
                }
            }
        }

        Err("All NTP servers failed".into())
    }

    /// Returns the current time with elapsed offset
    pub fn get_current_time(&self) -> DateTime<Utc> {
        self.latest_time + self.elapsed()
    }

    /// Updates the latest time from NTP servers
    fn update_latest_time(&mut self) {
        self.stats.total_attempts += 1;

        let latest_time_ntp = match Self::get_ntp_time(&self.ntp_servers) {
            Ok(time) => {
                self.stats.successful_syncs += 1;
                info!("NTP sync successful. Updated time: {}", time);
                Some(time)
            }
            Err(e) => {
                self.stats.failed_syncs += 1;
                error!("NTP fetch failed: {}", e);
                None
            }
        };

        if latest_time_ntp.is_none() {
            return;
        }

        let new_time = latest_time_ntp.unwrap();
        self.latest_time_ntp = Some(new_time);

        // If we're using default time and got a valid NTP time, update
        if self.latest_time == DEFAULT {
            self.latest_time = new_time - self.elapsed();
            self.latest_instant = Instant::now();
            info!("Initialized time from default to NTP time");
        } else {
            // Calculate drift and update time
            let expected_time = self.get_current_time();
            let drift = new_time.signed_duration_since(expected_time);

            if drift.num_milliseconds().abs() > 100 {
                info!("Correcting time drift: {} ms", drift.num_milliseconds());
                self.latest_time = new_time;
                self.latest_instant = Instant::now();
            }
        }
    }

    /// Starts the background thread for periodic NTP updates
    pub fn start(clock: Arc<Mutex<Self>>, interval_secs: u64, shutdown: Arc<AtomicBool>) {
        std::thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                {
                    let mut clock = clock.lock().unwrap();
                    clock.update_latest_time();
                    info!("=================================");
                    info!("Updated the time: {}", clock.latest_time);
                    info!("=================================");
                }
                std::thread::sleep(std::time::Duration::from_secs(interval_secs));
            }
            info!("Background sync thread shutting down");
        });
    }

    /// Returns current synchronization statistics
    pub fn get_stats(&self) -> &SyncStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_stats_default() {
        let stats = SyncStats::default();
        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.successful_syncs, 0);
        assert_eq!(stats.failed_syncs, 0);
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_sync_stats_success_rate() {
        let stats = SyncStats {
            total_attempts: 10,
            successful_syncs: 8,
            failed_syncs: 2,
        };
        assert_eq!(stats.success_rate(), 80.0);
    }

    #[test]
    fn test_clock_initialization() {
        let clock = Clock::new(None);
        // Clock should be initialized (even if NTP fails, it uses default time)
        assert!(clock.latest_time >= DEFAULT);
    }

    #[test]
    fn test_clock_with_custom_servers() {
        let servers = vec!["time.google.com:123".to_string()];
        let clock = Clock::new(Some(servers.clone()));
        assert_eq!(clock.ntp_servers, servers);
    }

    #[test]
    fn test_clock_get_current_time() {
        let clock = Clock::new(None);
        let current_time = clock.get_current_time();
        // Current time should be greater than or equal to the initial time
        assert!(current_time >= clock.latest_time);
    }
}
