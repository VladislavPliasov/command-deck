//! Unix-specific discovery details (future Kali/Linux path).
//!
//! Executable-ness is determined by the executable permission bit rather
//! than a file extension. Version detection for ELF binaries (which carry
//! no PE version resource) is explicitly deferred; `--version` style
//! execution must not happen in this phase.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn path_separator() -> char {
    ':'
}

pub fn is_executable_file(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(meta) => meta.is_file() && (meta.permissions().mode() & 0o111) != 0,
        Err(_) => false,
    }
}
