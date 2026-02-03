// Integration tests for the Clock-NTP library

use clock::{Clock, SyncStats, DEFAULT};

#[test]
fn test_sync_stats_functionality() {
    let mut stats = SyncStats::default();
    assert_eq!(stats.success_rate(), 0.0);
    
    stats.total_attempts = 10;
    stats.successful_syncs = 8;
    stats.failed_syncs = 2;
    
    assert_eq!(stats.success_rate(), 80.0);
}

#[test]
fn test_clock_with_default_servers() {
    let clock = Clock::new(None);
    // Clock should be initialized even if NTP servers are unavailable
    assert!(clock.latest_instant.elapsed().as_secs() < 1);
}

#[test]
fn test_clock_with_custom_single_server() {
    let servers = vec!["time.google.com:123".to_string()];
    let clock = Clock::new(Some(servers));
    // Verify clock was created (even if NTP fails, it should have a fallback)
    let current_time = clock.get_current_time();
    assert!(current_time >= DEFAULT);
}

#[test]
fn test_clock_with_multiple_custom_servers() {
    let servers = vec![
        "time.google.com:123".to_string(),
        "time.cloudflare.com:123".to_string(),
    ];
    let clock = Clock::new(Some(servers.clone()));
    // The ntp_servers field should match what we provided
    assert_eq!(clock.ntp_servers, servers);
}

#[test]
fn test_clock_current_time_advances() {
    let clock = Clock::new(None);
    let time1 = clock.get_current_time();
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let time2 = clock.get_current_time();
    // Time should advance
    assert!(time2 > time1);
}

#[test]
fn test_stats_accumulation() {
    let stats = SyncStats {
        total_attempts: 100,
        successful_syncs: 95,
        failed_syncs: 5,
    };
    
    assert_eq!(stats.total_attempts, 100);
    assert_eq!(stats.successful_syncs + stats.failed_syncs, 100);
    assert!(stats.success_rate() > 90.0);
}
