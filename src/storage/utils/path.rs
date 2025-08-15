// Path helper utilities shared across storage operations
use std::path::Path;

/// Build a remote path by joining base and file name.
pub fn build_remote_path(base: &str, file_name: &str) -> String {
    Path::new(base)
        .join(file_name)
        .to_string_lossy()
        .to_string()
}

/// Get relative path string between a full path and base path.
pub fn get_relative_path(full_path: &str, base_path: &str) -> String {
    if full_path == base_path {
        // For single-file case, return the file name to avoid empty relative path
        return Path::new(full_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
    }

    // Strip a prefix from the given path safely
    full_path
        .strip_prefix(base_path)
        .unwrap_or(full_path)
        .trim_start_matches('/')
        .to_string()
}

/// Normalize path by removing leading slash if present
pub fn normalize_path(path: &str) -> &str {
    if path.starts_with('/') {
        &path[1..]
    } else {
        path
    }
}

/// Get relative path string considering the root directory between a full path and base path.
pub fn get_root_relative_path(full_path: &str, base_path: &str) -> String {
    let full_path = Path::new(normalize_path(full_path));
    let base_path = Path::new(normalize_path(base_path));
    
    if full_path == base_path {
        // For single-file case, return the file name to avoid empty relative path
        return Path::new(full_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
    }

    full_path
        .strip_prefix(base_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| {
            full_path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default()
        })
}