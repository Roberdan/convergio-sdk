//! Integration tests for convergio-security.
//! Tests crypto, auth, and trust as an external consumer.

use convergio_security::aead;
use convergio_security::jwt;
use convergio_security::ssrf;
use convergio_security::trust::TrustLevel;
use std::net::IpAddr;

#[test]
fn aead_encrypt_decrypt_roundtrip() {
    aead::init_master_key(None);
    let org = "test-org";
    let plaintext = "sensitive data";

    let encrypted = aead::encrypt(org, plaintext).expect("encrypt should work");
    assert_ne!(encrypted, plaintext);

    let decrypted = aead::decrypt(org, &encrypted).expect("decrypt should work");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn jwt_issue_and_validate_roundtrip() {
    jwt::init_jwt_secret(None);

    let token = jwt::issue_token("agent-007", jwt::AgentRole::Executor, vec![], 3600)
        .expect("issue should work");
    assert!(!token.is_empty());

    let claims = jwt::validate_token(&token).expect("validate should work");
    assert_eq!(claims.sub, "agent-007");
}

#[test]
fn ssrf_blocks_private_ips() {
    assert!(ssrf::is_private_ip("127.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(ssrf::is_private_ip("10.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(ssrf::is_private_ip(
        "192.168.1.1".parse::<IpAddr>().unwrap()
    ));
    assert!(!ssrf::is_private_ip("8.8.8.8".parse::<IpAddr>().unwrap()));
}

#[test]
fn ssrf_validates_urls() {
    assert!(ssrf::validate_outbound_url("https://example.com/api").is_ok());
    assert!(ssrf::validate_outbound_url("").is_err());
}

#[test]
fn trust_level_ordering() {
    assert_eq!(TrustLevel::from_i64(0), TrustLevel::Untrusted);
    assert_eq!(TrustLevel::from_i64(1), TrustLevel::Basic);
    assert_eq!(TrustLevel::from_i64(2), TrustLevel::Standard);
}
