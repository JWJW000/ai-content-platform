//! nettop command wrapper for macOS network traffic collection
//! 
//! Parses output from `nettop -P -J bytes_in,bytes_out -x -l 1`

use super::ProcessTraffic;
use std::process::Command;

/// Collector using macOS nettop command
pub struct NettopCollector {
    interval: u32,
}

impl NettopCollector {
    pub fn new() -> Self {
        Self { interval: 1 }
    }

    /// Collect process traffic using nettop
    #[cfg(target_os = "macos")]
    pub fn collect(&self) -> Result<Vec<ProcessTraffic>, String> {
        let output = Command::new("nettop")
            .args(["-P", "-J", "bytes_in,bytes_out", "-x", "-L", "1", "-l", "1"])
            .output()
            .map_err(|e| format!("Failed to execute nettop: {}", e))?;

        if !output.status.success() {
            return Err(format!("nettop command failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_output(&stdout)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn collect(&self) -> Result<Vec<ProcessTraffic>, String> {
        Ok(Vec::new())
    }

    /// Parse nettop output
    #[cfg(target_os = "macos")]
    fn parse_output(&self, output: &str) -> Result<Vec<ProcessTraffic>, String> {
        let mut processes = Vec::new();
        
        for line in output.lines() {
            if let Some(process) = self.parse_line(line)? {
                processes.push(process);
            }
        }
        
        Ok(processes)
    }

    /// Parse a single line of nettop output
    #[cfg(target_os = "macos")]
    fn parse_line(&self, line: &str) -> Result<Option<ProcessTraffic>, String> {
        if line.trim().is_empty() {
            return Ok(None);
        }

        let json: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        let name = json.get("name")
            .or_else(|| json.get("process_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let pid = json.get("pid")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let bytes_in = json.get("bytes_in")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_bytes(s))
            .unwrap_or(0);

        let bytes_out = json.get("bytes_out")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_bytes(s))
            .unwrap_or(0);

        if name == "kernel_task" || (bytes_in == 0 && bytes_out == 0) {
            return Ok(None);
        }

        Ok(Some(ProcessTraffic {
            pid,
            name,
            bytes_in,
            bytes_out,
            icon_path: None,
        }))
    }
}

/// Parse byte string like "1.2MB" or "456KB" to u64
fn parse_bytes(s: &str) -> Option<u64> {
    let s = s.trim();
    
    if let Ok(n) = s.parse::<u64>() {
        return Some(n);
    }

    let multiplier = if s.ends_with("KB") || s.ends_with("KiB") {
        1024u64
    } else if s.ends_with("MB") || s.ends_with("MiB") {
        1024 * 1024
    } else if s.ends_with("GB") || s.ends_with("GiB") {
        1024 * 1024 * 1024
    } else if s.ends_with("B") {
        1
    } else {
        return None;
    };

    let num_str = &s[..s.len().saturating_sub(2)];
    let num: f64 = num_str.parse().ok()?;
    
    Some((num * multiplier as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        assert_eq!(parse_bytes("1024"), Some(1024));
        assert_eq!(parse_bytes("1KB"), Some(1024));
        assert_eq!(parse_bytes("1.5MB"), Some(1024 * 1024 + 512 * 1024));
    }
}
