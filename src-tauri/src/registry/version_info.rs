use std::path::Path;

/// Best-effort read of the Windows PE version resource of a file.
///
/// This performs read-only metadata inspection via the Win32 version
/// information APIs (`version.dll`); the target executable is **never**
/// executed. Any failure - missing resource, unsupported file, API error -
/// returns `None` and must never block or fail the calling workflow.
#[cfg(windows)]
pub fn read_file_version(path: &Path) -> Option<String> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
    };

    fn read_inner(path: &Path) -> Option<String> {
        let wide: Vec<u16> = path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let file_name = PCWSTR(wide.as_ptr());
        unsafe {
            let size = GetFileVersionInfoSizeW(file_name, None);
            if size == 0 {
                return None;
            }
            let mut buffer = vec![0u8; size as usize];
            GetFileVersionInfoW(file_name, None, size, buffer.as_mut_ptr() as *mut _).ok()?;
            let mut info_ptr: *mut VS_FIXEDFILEINFO = std::ptr::null_mut();
            let mut info_len: u32 = 0;
            let ok = VerQueryValueW(
                buffer.as_ptr() as *const _,
                windows::core::w!("\\"),
                &mut info_ptr as *mut _ as *mut _,
                &mut info_len,
            );
            if !ok.as_bool() {
                return None;
            }
            if info_ptr.is_null() {
                return None;
            }
            let info: VS_FIXEDFILEINFO = std::ptr::read_unaligned(info_ptr);
            if info.dwSignature != 0xFEEF04BD {
                return None;
            }
            Some(format!(
                "{}.{}.{}.{}",
                (info.dwFileVersionMS >> 16) & 0xffff,
                info.dwFileVersionMS & 0xffff,
                (info.dwFileVersionLS >> 16) & 0xffff,
                info.dwFileVersionLS & 0xffff
            ))
        }
    }

    // Absolute belt-and-braces guarantee: version detection must never
    // block discovery/registration, not even on an unexpected panic.
    std::panic::catch_unwind(|| read_inner(path)).ok().flatten()
}

/// Non-Windows executables (ELF etc.) carry no PE version resource;
/// executing `--version` style commands is explicitly deferred to a later
/// phase, so version detection is unavailable here.
#[cfg(not(windows))]
pub fn read_file_version(_path: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_returns_none() {
        let result = read_file_version(Path::new("Z:\\definitely\\missing\\file.exe"));
        assert!(result.is_none());
    }

    #[test]
    fn empty_file_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.exe");
        std::fs::write(&path, b"").unwrap();
        assert!(read_file_version(&path).is_none());
    }

    #[cfg(windows)]
    #[test]
    fn own_executable_has_readable_version_or_none() {
        // The application binary embeds a version resource (tauri-winres);
        // reading it must either return a dotted version or None - never
        // panic and never error.
        if let Ok(exe) = std::env::current_exe() {
            let version = read_file_version(&exe);
            if let Some(v) = version {
                assert!(v.matches('.').count() >= 1, "unexpected version shape: {v}");
            }
        }
    }
}
