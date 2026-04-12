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
/// Returns Err if the URL uses a non-HTTP scheme, contains credentials,
/// or the resolved IP is private/reserved.
pub fn validate_outbound_url(url: &str) -> Result<(), String> {
    let parsed = url::Url::parse(url).map_err(|e| format!("invalid URL: {e}"))?;

    // Only allow http/https schemes
    match parsed.scheme() {
        "http" | "https" => {}
        s => return Err(format!("blocked scheme: {s}")),
    }

    // Block credentials in URL (user:pass@host)
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("URL must not contain credentials".into());
    }

    let host = parsed.host_str().unwrap_or("");
    if host.is_empty() {
        return Err("empty host".into());
    }

    // Block well-known localhost hostnames
    let lower = host.to_ascii_lowercase();
    if lower == "localhost"
        || lower == "localhost."
        || lower.ends_with(".localhost")
        || lower.ends_with(".internal")
        || lower == "metadata.google.internal"
    {
        return Err(format!("SSRF blocked: hostname {host} resolves to local"));
    }

    // Strip brackets from IPv6 literals for parsing
    let bare = host.trim_start_matches('[').trim_end_matches(']');
    if let Ok(ip) = bare.parse::<IpAddr>() {
        if is_private_ip(ip) {
            return Err(format!("SSRF blocked: {ip} is a private address"));
        }
    }

    // For non-IP hostnames, the caller MUST re-validate after DNS resolution
    // using `is_private_ip` on the resolved address.
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

    #[test]
    fn validate_url_blocks_non_http_schemes() {
        assert!(validate_outbound_url("ftp://evil.com/file").is_err());
        assert!(validate_outbound_url("file:///etc/passwd").is_err());
        assert!(validate_outbound_url("gopher://evil.com").is_err());
    }

    #[test]
    fn validate_url_blocks_credentials() {
        assert!(validate_outbound_url("http://user:pass@evil.com").is_err());
        assert!(validate_outbound_url("http://admin@10.0.0.1").is_err());
    }

    #[test]
    fn validate_url_blocks_localhost_hostname() {
        assert!(validate_outbound_url("http://localhost/admin").is_err());
        assert!(validate_outbound_url("http://localhost.").is_err());
        assert!(validate_outbound_url("http://foo.localhost/x").is_err());
    }

    #[test]
    fn validate_url_blocks_ipv6_loopback() {
        assert!(validate_outbound_url("http://[::1]/admin").is_err());
    }
}
