use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const NATIVE: NaiveDateTime = NaiveDate::from_ymd_opt(2000, 1, 1)
    .unwrap()
    .and_hms_opt(0, 0, 0)
    .unwrap();
const DEFAULT: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(NATIVE, Utc);

pub struct Clock {
    latest_time_ntp: Option<DateTime<Utc>>,
    latest_time: DateTime<Utc>,
    pub latest_instant: Instant,
}

impl Clock {
    pub fn new() -> Self {
        // Try to get NTP time, fallback to None time if it fails

        let latest_time_ntp = match Self::get_ntp_time() {
            Ok(time) => Some(time),
            Err(e) => {
                print!("NTP fetch failed, falling back to default time\n  {e}");

                None
            }
        };

        // we need to run current time updater here to set the initial time
        Clock {
            latest_time_ntp,
            latest_time: latest_time_ntp.unwrap_or(DEFAULT),
            latest_instant: Instant::now(),
        }
    }

    fn elapsed(&self) -> Duration {
        chrono::Duration::from_std(self.latest_instant.elapsed()).unwrap_or_default()
    }

    fn get_ntp_time() -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        let servers = [
            "time.google.com:123",
            "time.cloudflare.com:123",
            "pool.ntp.org:123",
        ];

        for server in &servers {
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
                                            return Ok(dt);
                                        }
                                    }
                                }
                            }
                            Err(_) => continue,
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        Err("All NTP servers failed".into())
    }

    pub fn get_current_time(&self) -> DateTime<Utc> {
        self.latest_time + self.elapsed()
    }
    fn update_latest_time(&mut self) {
        let latest_time_ntp = match Self::get_ntp_time() {
            Ok(time) => Some(time),
            Err(e) => {
                print!("NTP fetch failed, falling back to default time\n  {e}");

                None
            }
        };
        if latest_time_ntp.is_none() {
            return;
        }
        self.latest_time_ntp = latest_time_ntp;

        if self.latest_time == DEFAULT && !self.latest_time_ntp.is_none() {
            self.latest_time = self.latest_time_ntp.unwrap() - self.elapsed();
            self.latest_instant = Instant::now();
        } else if !self.latest_time_ntp.is_none() && !latest_time_ntp.is_none() {
        }
    }
    pub fn start(clock: Arc<Mutex<Self>>) {
        std::thread::spawn(move || {
            loop {
                {
                    let mut clock = clock.lock().unwrap();
                    clock.update_latest_time();
                    println!("=================================");
                    println!("Updated the time {}", clock.latest_time);
                    println!("=================================");
                }
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clock = Arc::new(Mutex::new(Clock::new()));
    Clock::start(Arc::clone(&clock));
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let clock = clock.lock().unwrap();
        println!("Updated time after 1 seconds: {}", clock.get_current_time());
    }
}
