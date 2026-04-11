//! Cross-platform daemon restart primitives.
//!
//! macOS: launchctl unload/load (launchd)
//! Linux: systemctl restart (systemd)
//! Windows: net stop/start (Windows service)
//! Fallback: SIGHUP or process restart

use std::process::Command;

/// Outcome of a restart attempt.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RestartResult {
    pub success: bool,
    pub method: &'static str,
    pub message: String,
}

/// Restart the daemon using the platform-native service manager.
pub fn restart_daemon() -> RestartResult {
    #[cfg(target_os = "macos")]
    {
        restart_launchd()
    }
    #[cfg(target_os = "linux")]
    {
        restart_systemd()
    }
    #[cfg(target_os = "windows")]
    {
        restart_windows_service()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        RestartResult {
            success: false,
            method: "unsupported",
            message: "No restart method for this platform".into(),
        }
    }
}

/// Stop the daemon using the platform-native service manager.
pub fn stop_daemon() -> RestartResult {
    #[cfg(target_os = "macos")]
    {
        launchctl("unload")
    }
    #[cfg(target_os = "linux")]
    {
        systemctl("stop")
    }
    #[cfg(target_os = "windows")]
    {
        net_service("stop")
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        RestartResult {
            success: false,
            method: "unsupported",
            message: "No stop method for this platform".into(),
        }
    }
}

/// Path to the platform service config file.
pub fn service_config_path() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "macos")]
    {
        const PLIST_LABEL: &str = "com.convergio.daemon";
        dirs::home_dir().map(|h| {
            h.join("Library/LaunchAgents")
                .join(format!("{PLIST_LABEL}.plist"))
        })
    }
    #[cfg(target_os = "linux")]
    {
        Some(std::path::PathBuf::from(
            "/etc/systemd/system/convergio.service",
        ))
    }
    #[cfg(target_os = "windows")]
    {
        None // Windows services are registry-based
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

// -- macOS (launchd) --

#[cfg(target_os = "macos")]
fn restart_launchd() -> RestartResult {
    let stop = launchctl("unload");
    if !stop.success {
        // Not fatal — maybe service wasn't loaded
        eprintln!("launchd unload: {}", stop.message);
    }
    launchctl("load")
}

#[cfg(target_os = "macos")]
fn launchctl(action: &str) -> RestartResult {
    let plist = match service_config_path() {
        Some(p) => p,
        None => {
            return RestartResult {
                success: false,
                method: "launchd",
                message: "Cannot determine plist path".into(),
            }
        }
    };
    let result = Command::new("launchctl")
        .args([action, &plist.to_string_lossy()])
        .output();
    match result {
        Ok(out) if out.status.success() => RestartResult {
            success: true,
            method: "launchd",
            message: format!("launchctl {action} OK"),
        },
        Ok(out) => RestartResult {
            success: false,
            method: "launchd",
            message: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Err(e) => RestartResult {
            success: false,
            method: "launchd",
            message: e.to_string(),
        },
    }
}

// -- Linux (systemd) --

#[cfg(target_os = "linux")]
fn restart_systemd() -> RestartResult {
    systemctl("restart")
}

#[cfg(target_os = "linux")]
fn systemctl(action: &str) -> RestartResult {
    let result = Command::new("systemctl")
        .args(["--user", action, "convergio"])
        .output();
    match result {
        Ok(out) if out.status.success() => RestartResult {
            success: true,
            method: "systemd",
            message: format!("systemctl {action} OK"),
        },
        Ok(out) => RestartResult {
            success: false,
            method: "systemd",
            message: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Err(e) => RestartResult {
            success: false,
            method: "systemd",
            message: e.to_string(),
        },
    }
}

// -- Windows --

#[cfg(target_os = "windows")]
fn restart_windows_service() -> RestartResult {
    let stop = net_service("stop");
    if !stop.success {
        eprintln!("net stop: {}", stop.message);
    }
    net_service("start")
}

#[cfg(target_os = "windows")]
fn net_service(action: &str) -> RestartResult {
    let result = Command::new("net").args([action, "Convergio"]).output();
    match result {
        Ok(out) if out.status.success() => RestartResult {
            success: true,
            method: "windows-service",
            message: format!("net {action} OK"),
        },
        Ok(out) => RestartResult {
            success: false,
            method: "windows-service",
            message: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Err(e) => RestartResult {
            success: false,
            method: "windows-service",
            message: e.to_string(),
        },
    }
}
