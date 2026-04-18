//! Utility functions

use std::fmt::Write;

/// Format bytes to human-readable string (KB, MB, GB, etc.)
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_idx = 0;
    
    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{:.0}{}", value, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", value, UNITS[unit_idx])
    }
}

/// Format bytes per second to human-readable string
pub fn format_speed(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0B");
        assert_eq!(format_bytes(1024), "1.0KB");
        assert_eq!(format_bytes(1536), "1.5KB");
        assert_eq!(format_bytes(1048576), "1.0MB");
        assert_eq!(format_bytes(1073741824), "1.0GB");
    }
    
    #[test]
    fn test_format_speed() {
        assert_eq!(format_speed(1024), "1.0KB/s");
        assert_eq!(format_speed(1048576), "1.0MB/s");
    }
}
