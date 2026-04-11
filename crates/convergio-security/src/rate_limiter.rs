//! Token-bucket rate limiter — per-IP, configurable.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::Instant;

/// Rate limiter state shared across requests.
pub struct RateLimiter {
    buckets: Mutex<HashMap<IpAddr, Bucket>>,
    max_tokens: u32,
    refill_per_sec: f64,
}

struct Bucket {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter. `max_per_minute` = max requests/min per IP.
    pub fn new(max_per_minute: u32) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            max_tokens: max_per_minute,
            refill_per_sec: max_per_minute as f64 / 60.0,
        }
    }

    /// Check if request from IP is allowed. Returns false if rate-limited.
    pub fn check(&self, ip: IpAddr) -> bool {
        let mut map = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let max = self.max_tokens as f64;
        let refill = self.refill_per_sec;
        let bucket = map.entry(ip).or_insert(Bucket {
            tokens: max,
            last_refill: now,
        });
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * refill).min(max);
        bucket.last_refill = now;
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Cleanup stale entries (IPs not seen in 10 minutes).
    pub fn cleanup(&self) {
        let mut map = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        let cutoff = Instant::now() - std::time::Duration::from_secs(600);
        map.retain(|_, b| b.last_refill > cutoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_within_limit() {
        let rl = RateLimiter::new(10);
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        for _ in 0..10 {
            assert!(rl.check(ip));
        }
    }

    #[test]
    fn blocks_after_exhausted() {
        let rl = RateLimiter::new(2);
        let ip: IpAddr = "5.6.7.8".parse().unwrap();
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn separate_ip_buckets() {
        let rl = RateLimiter::new(1);
        let ip1: IpAddr = "1.1.1.1".parse().unwrap();
        let ip2: IpAddr = "2.2.2.2".parse().unwrap();
        assert!(rl.check(ip1));
        assert!(rl.check(ip2));
        assert!(!rl.check(ip1));
    }
}
