//! Windows-specific discovery details.
//!
//! P0-03 policy intentionally discovers only `.exe` and `.com` files;
//! PATHEXT is deliberately NOT consulted so script types (`.bat`, `.cmd`,
//! `.ps1`, ...) can never appear in results. PE version metadata is read
//! via `crate::registry::version_info` (read-only, never executes).

use std::path::Path;

pub fn path_separator() -> char {
    ';'
}

pub fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}
