//! SSRF protection — blocks outbound HTTP to private/reserved IP ranges.

use std::net::IpAddr;

/// Returns true if the IP address is in a private/reserved range.
/// Block these in outbound HTTP to prevent SSRF attacks.
pub fn is_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let octets = v4.octets();
            // 10.0.0.0/8
            octets[0] == 10
            // 172.16.0.0/12
            || (octets[0] == 172 && (16..=31).contains(&octets[1]))
            // 192.168.0.0/16
            || (octets[0] == 192 && octets[1] == 168)
            // 127.0.0.0/8 (loopback)
            || octets[0] == 127
            // 169.254.0.0/16 (link-local)
            || (octets[0] == 169 && octets[1] == 254)
            // 0.0.0.0
            || v4.is_unspecified()
        }
        IpAddr::V6(v6) => {
            // WHY: ::ffff:127.0.0.1 is IPv4-mapped IPv6 — common SSRF bypass
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_private_ip(IpAddr::V4(v4));
            }
            v6.is_loopback() || v6.is_unspecified()
            // fe80::/10 (link-local)
            || (v6.segments()[0] & 0xffc0) == 0xfe80
            // fc00::/7 (unique local)
            || (v6.segments()[0] & 0xfe00) == 0xfc00
        }
    }
}

/// Validate a URL target before making outbound HTTP request.
/// Returns Err if the resolved IP is private/reserved.
pub fn validate_outbound_url(url: &str) -> Result<(), String> {
    // Extract hostname from URL
    let host = url
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("");

    if host.is_empty() {
        return Err("empty host".into());
    }

    // Try parsing as IP directly
    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_private_ip(ip) {
            return Err(format!("SSRF blocked: {ip} is a private address"));
        }
    }

    // For hostnames, we can't resolve DNS here (async).
    // The caller should resolve and re-check after DNS resolution.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_ranges_detected() {
        assert!(is_private_ip("10.0.0.1".parse().unwrap()));
        assert!(is_private_ip("172.16.0.1".parse().unwrap()));
        assert!(is_private_ip("172.31.255.255".parse().unwrap()));
        assert!(is_private_ip("192.168.1.1".parse().unwrap()));
        assert!(is_private_ip("127.0.0.1".parse().unwrap()));
        assert!(is_private_ip("169.254.1.1".parse().unwrap()));
        assert!(is_private_ip("0.0.0.0".parse().unwrap()));
        assert!(is_private_ip("::1".parse().unwrap()));
    }

    #[test]
    fn public_ranges_allowed() {
        assert!(!is_private_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_private_ip("1.1.1.1".parse().unwrap()));
        assert!(!is_private_ip("172.32.0.1".parse().unwrap()));
        assert!(!is_private_ip("172.15.0.1".parse().unwrap()));
    }

    #[test]
    fn validate_url_blocks_private() {
        assert!(validate_outbound_url("http://10.0.0.1/admin").is_err());
        assert!(validate_outbound_url("http://127.0.0.1:8080").is_err());
        assert!(validate_outbound_url("http://192.168.1.1").is_err());
    }

    #[test]
    fn validate_url_allows_public() {
        assert!(validate_outbound_url("https://api.github.com").is_ok());
        assert!(validate_outbound_url("http://8.8.8.8/dns").is_ok());
    }

    #[test]
    fn validate_url_rejects_empty() {
        assert!(validate_outbound_url("").is_err());
    }
}
