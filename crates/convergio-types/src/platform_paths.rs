//! Platform-aware directory resolution.
//!
//! macOS: ~/Library/Application Support/Convergio
//! Linux: ~/.local/share/convergio
//! Windows: %APPDATA%/Convergio
//! Fallback: ~/.convergio/

use std::path::PathBuf;

/// Primary Convergio data directory.
pub fn convergio_data_dir() -> PathBuf {
    if let Some(data) = dirs::data_dir() {
        return data.join("Convergio");
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".convergio")
}

/// Output directory for a named project.
pub fn project_output_dir(project_name: &str) -> PathBuf {
    convergio_data_dir()
        .join("projects")
        .join(project_name)
        .join("output")
}

/// Validate a path is within an allowed base directory. Prevents path traversal.
/// Returns the canonicalized path on success, or an error if traversal detected.
pub fn sanitize_path(
    path: &std::path::Path,
    allowed_base: &std::path::Path,
) -> Result<PathBuf, String> {
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("invalid path {}: {e}", path.display()))?;
    let base = allowed_base
        .canonicalize()
        .map_err(|e| format!("invalid base {}: {e}", allowed_base.display()))?;
    if canonical.starts_with(&base) {
        Ok(canonical)
    } else {
        Err(format!(
            "path {} is outside allowed directory {}",
            canonical.display(),
            base.display()
        ))
    }
}

/// Validate a path only contains safe characters (no traversal components).
/// Does NOT require the path to exist yet (for create operations).
pub fn validate_path_components(path: &std::path::Path) -> Result<(), String> {
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err(format!("path traversal '..' in {}", path.display()));
            }
            std::path::Component::Normal(s) => {
                let s = s.to_string_lossy();
                if s.starts_with('.') && s.len() > 1 && !s.starts_with("..") {
                    // allow hidden files like .convergio, .worktrees
                }
                if s.contains('\0') {
                    return Err("null byte in path component".into());
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_dir_is_absolute_or_fallback() {
        let dir = convergio_data_dir();
        let name = dir.file_name().unwrap().to_str().unwrap();
        assert!(
            name == "Convergio" || name == ".convergio",
            "unexpected dir name: {name}"
        );
    }

    #[test]
    fn project_output_dir_structure() {
        let out = project_output_dir("my-app");
        assert!(out.ends_with("projects/my-app/output"));
    }
}
