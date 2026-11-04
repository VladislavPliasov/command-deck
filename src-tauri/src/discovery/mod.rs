use crate::registry::model::{
    DiscoveryMatch, DiscoveryResult, DiscoveryStatus,
};
use std::path::{Path, PathBuf};

#[cfg(unix)]
mod unix_ext;
#[cfg(windows)]
mod windows_ext;

#[cfg(unix)]
use unix_ext as platform;
#[cfg(windows)]
use windows_ext as platform;

const MAX_NAME_LEN: usize = 255;

/// Resolves an executable name against the directories in `PATH`.
///
/// P0-03 policy: only `.exe` and `.com` files are discovered and returned
/// (`.bat`, `.cmd`, `.ps1` and other script types are intentionally not
/// supported). Discovered files are never executed; only file metadata is
/// read (plus best-effort PE version resources on Windows).
pub fn resolve_name(name: &str) -> DiscoveryResult {
    let query = name.to_string();
    let Some((stem, explicit_ext)) = split_name(name) else {
        return DiscoveryResult {
            query,
            status: DiscoveryStatus::InvalidName,
            matches: vec![],
        };
    };
    let exts: Vec<&str> = match explicit_ext {
        Some(_) => vec![""],
        None => vec!["exe", "com"],
    };
    let dirs = path_dirs();
    let mut matches = resolve_in(&dirs, stem, &exts);
    for m in &mut matches {
        m.file_version = crate::registry::version_info::read_file_version(Path::new(&m.path));
    }
    let status = match matches.len() {
        0 => DiscoveryStatus::NotFound,
        1 => DiscoveryStatus::Found,
        _ => DiscoveryStatus::Ambiguous,
    };
    DiscoveryResult {
        query,
        status,
        matches,
    }
}

/// Pure lookup used by `resolve_name` (and tests): searches the given
/// directories for `stem` + each extension. Tolerates missing/unreadable
/// directories. Never executes anything.
pub fn resolve_in(dirs: &[PathBuf], stem: &str, exts: &[&str]) -> Vec<DiscoveryMatch> {
    let mut out: Vec<DiscoveryMatch> = vec![];
    for dir in dirs {
        for ext in exts {
            let file_name = if ext.is_empty() {
                stem.to_string()
            } else {
                format!("{stem}.{ext}")
            };
            let path = dir.join(&file_name);
            if !platform::is_executable_file(&path) {
                continue;
            }
            let canonical = dunce::canonicalize(&path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            let duplicate = out.iter().any(|m| {
                if cfg!(windows) {
                    m.path.eq_ignore_ascii_case(&canonical)
                } else {
                    m.path == canonical
                }
            });
            if duplicate {
                continue;
            }
            let extension = Path::new(&file_name)
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            out.push(DiscoveryMatch {
                path: canonical,
                file_name,
                extension,
                file_version: None,
            });
        }
    }
    out
}

/// Validates a discovery query and splits it into a stem plus an explicit
/// extension when one was given. Returns `None` for invalid names; this
/// runs before any filesystem access.
fn split_name(name: &str) -> Option<(&str, Option<&str>)> {
    if name.ends_with('.') || name.ends_with(' ') {
        return None;
    }
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_NAME_LEN {
        return None;
    }
    // Reject anything that could address a location rather than a file
    // name in PATH directories, or that is illegal on Windows.
    for c in trimmed.chars() {
        if matches!(
            c,
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'
        ) || c.is_control()
        {
            return None;
        }
    }
    match trimmed.rfind('.') {
        Some(idx) if idx > 0 && idx < trimmed.len() - 1 => {
            let ext = &trimmed[idx + 1..];
            let ext_lower = ext.to_ascii_lowercase();
            if ext_lower == "exe" || ext_lower == "com" {
                Some((&trimmed[..idx], Some(ext)))
            } else {
                None // only .exe and .com are supported (P0-03 decision)
            }
        }
        _ => Some((trimmed, None)),
    }
}

fn path_dirs() -> Vec<PathBuf> {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(platform::path_separator())
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dir(files: &[&str]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for f in files {
            std::fs::write(dir.path().join(f), b"dummy").unwrap();
        }
        dir
    }

    #[test]
    fn invalid_names_are_rejected_before_filesystem_access() {
        for name in [
            "",
            "   ",
            "a/b.exe",
            "a\\b.exe",
            "C:\\Windows\\cmd.exe", // absolute path, not a bare name
            "..",
            "trailing.",
            "trailing ",
            "tool.bat",  // unsupported extension
            "tool.cmd",  // unsupported extension
            "tool.ps1",  // unsupported extension
            "tool.txt",  // unsupported extension
            "a:b.exe",   // reserved/illegal
        ] {
            let result = resolve_name(name);
            assert_eq!(
                result.status,
                DiscoveryStatus::InvalidName,
                "{name} should be invalid"
            );
            assert!(result.matches.is_empty());
        }
    }

    #[test]
    fn finds_only_exe_and_com_files() {
        // correction 1: .bat/.cmd/.ps1 must never be returned
        let dir = make_dir(&[
            "nmap.exe", "nmap.com", "nmap.bat", "nmap.cmd", "nmap.ps1", "nmap.txt",
        ]);
        let matches = resolve_in(&[dir.path().to_path_buf()], "nmap", &["exe", "com"]);
        let mut names: Vec<&str> = matches.iter().map(|m| m.file_name.as_str()).collect();
        names.sort_unstable();
        assert_eq!(names, vec!["nmap.com", "nmap.exe"]);
    }

    #[test]
    fn explicit_extension_matches_exactly() {
        let dir = make_dir(&["tool.exe", "tool.com"]);
        let matches = resolve_in(&[dir.path().to_path_buf()], "tool.exe", &[""]);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].file_name, "tool.exe");
    }

    #[test]
    fn multiple_directories_yield_ambiguous_matches() {
        let dir_a = make_dir(&["tool.exe"]);
        let dir_b = make_dir(&["tool.exe"]);
        let matches = resolve_in(
            &[dir_a.path().to_path_buf(), dir_b.path().to_path_buf()],
            "tool",
            &["exe", "com"],
        );
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn missing_files_and_dirs_are_tolerated() {
        let dir = make_dir(&[]);
        let matches = resolve_in(
            &[
                PathBuf::from("Z:\\definitely\\missing"),
                dir.path().to_path_buf(),
            ],
            "nothing",
            &["exe", "com"],
        );
        assert!(matches.is_empty());
    }

    #[test]
    fn duplicate_canonical_matches_are_deduplicated() {
        let dir = make_dir(&["tool.exe"]);
        // same directory listed twice (plus a trailing-dot alias if the
        // platform canonicalizes it to the same location)
        let matches = resolve_in(
            &[dir.path().to_path_buf(), dir.path().to_path_buf()],
            "tool",
            &["exe"],
        );
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn dummy_files_have_no_version() {
        let dir = make_dir(&["tool.exe"]);
        let matches = resolve_in(&[dir.path().to_path_buf()], "tool", &["exe"]);
        assert!(matches[0].file_version.is_none());
    }
}
