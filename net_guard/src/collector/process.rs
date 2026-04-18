//! Process information module for macOS
//! 
//! Uses libproc to get process details and App bundle paths.

use std::path::PathBuf;

/// Process information structure
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub bundle_path: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
}

/// Get process information for a given PID
/// 
/// On macOS, tries to find the App bundle path and icon for a process.
pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    #[cfg(target_os = "macos")]
    {
        get_process_info_macos(pid)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pid;
        None
    }
}

#[cfg(target_os = "macos")]
fn get_process_info_macos(pid: u32) -> Option<ProcessInfo> {
    use std::ffi::CString;
    use std::mem::MaybeUninit;
    
    // Get process name using libproc
    let mut name_buf = [0u8; 256];
    let name_len = unsafe {
        libc::proc_pidpath(pid as libc::pid_t, name_buf.as_mut_ptr() as *mut libc::c_void, name_buf.len() as u32)
    };
    
    if name_len == 0 {
        return None;
    }
    
    let path = PathBuf::from(String::from_utf8_lossy(&name_buf[..name_len as usize]));
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Try to find bundle path for .app processes
    let bundle_path = if path.extension().map(|e| e == "app").unwrap_or(false) {
        Some(path.clone())
    } else {
        // Look for .app in parent directories
        let parent = path.parent()?;
        parent.file_name()
            .and_then(|n| n.to_str())
            .filter(|n| n.ends_with(".app"))
            .map(|n| parent.join(n))
    };
    
    // Try to get icon path
    let icon_path = bundle_path.as_ref()
        .and_then(|bp| bp.join("Contents/Resources/AppIcon.icns").exists())
        .then(|| bundle_path.clone().map(|p| p.join("Contents/Resources/AppIcon.icns")))
        .flatten();
    
    Some(ProcessInfo {
        pid,
        name,
        bundle_path,
        icon_path,
    })
}

/// Get list of running processes with network activity
pub fn get_active_processes() -> Vec<u32> {
    #[cfg(target_os = "macos")]
    {
        get_active_processes_macos()
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Vec::new()
    }
}

#[cfg(target_os = "macos")]
fn get_active_processes_macos() -> Vec<u32> {
    // Use ps to get list of processes with network sockets
    let output = std::process::Command::new("ps")
        .args(["-ax", "-o", "pid"])
        .output();
    
    match output {
        Ok(out) => {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .skip(1) // Skip header
                .filter_map(|l| l.trim().parse::<u32>().ok())
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
