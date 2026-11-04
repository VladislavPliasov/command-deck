use super::model::{RegistryError, ToolProfile};
use super::validation;
use std::fs;
use std::path::{Path, PathBuf};

/// Per-tool JSON file persistence under a single registry directory.
///
/// Each profile is stored as `<id>.json` (id is validated as a safe
/// filename before any filesystem operation). Writes are atomic:
/// content goes to `<id>.json.tmp` first and is then renamed over the
/// target. Known limitation (see ADR-002): concurrent writers in
/// multiple application instances can lose updates; single-instance
/// locking is intentionally out of scope for P0-03.
#[derive(Debug, Clone)]
pub struct Storage {
    dir: PathBuf,
}

#[derive(Debug, Default)]
pub struct LoadReport {
    pub profiles: Vec<ToolProfile>,
    pub warnings: Vec<String>,
}

impl Storage {
    pub fn new(dir: PathBuf) -> Result<Self, RegistryError> {
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    #[allow(dead_code)] // used by tests and future tooling
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Resolves the storage path for an id. Validates the id first so no
    /// filesystem operation can ever be reached with an unsafe name.
    fn path_for(&self, id: &str) -> Result<PathBuf, RegistryError> {
        validation::validate_id(id)?;
        Ok(self.dir.join(format!("{id}.json")))
    }

    pub fn exists(&self, id: &str) -> Result<bool, RegistryError> {
        Ok(self.path_for(id)?.exists())
    }

    pub fn load(&self, id: &str) -> Result<ToolProfile, RegistryError> {
        let path = self.path_for(id)?;
        let raw = fs::read_to_string(&path).map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                RegistryError::new("not_found", format!("tool '{id}' is not registered"))
            } else {
                RegistryError::from(err)
            }
        })?;
        serde_json::from_str(&raw)
            .map_err(|err| RegistryError::new("io", format!("stored profile '{id}' is corrupt: {err}")))
    }

    /// Loads every `*.json` profile in the registry directory. Corrupt or
    /// invalid files are skipped and reported as warnings; they never
    /// prevent the remaining profiles from loading.
    pub fn list(&self) -> LoadReport {
        let mut report = LoadReport::default();
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(err) => {
                report
                    .warnings
                    .push(format!("cannot read registry directory: {err}"));
                return report;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let is_json = path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false);
            if !is_json {
                continue;
            }
            let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else {
                continue;
            };
            match self.load(&stem) {
                Ok(profile) => report.profiles.push(profile),
                Err(err) => report.warnings.push(format!("{stem}: {err}")),
            }
        }
        report.profiles.sort_by(|a, b| a.id.cmp(&b.id));
        report
    }

    pub fn save(&self, profile: &ToolProfile) -> Result<(), RegistryError> {
        let path = self.path_for(&profile.id)?;
        let json = serde_json::to_string_pretty(profile)
            .map_err(|err| RegistryError::new("io", format!("failed to serialize profile: {err}")))?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, json.as_bytes())?;
        fs::rename(&tmp, &path).map_err(|err| {
            let _ = fs::remove_file(&tmp);
            RegistryError::from(err)
        })
    }

    pub fn remove(&self, id: &str) -> Result<(), RegistryError> {
        let path = self.path_for(id)?;
        if !path.exists() {
            return Err(RegistryError::new(
                "not_found",
                format!("tool '{id}' is not registered"),
            ));
        }
        fs::remove_file(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::model::*;

    fn sample_profile(id: &str, name: &str) -> ToolProfile {
        ToolProfile {
            schema_version: SCHEMA_VERSION.to_string(),
            id: id.to_string(),
            name: name.to_string(),
            description: Some("desc".to_string()),
            category: Category::Scanning,
            tags: vec!["t1".to_string()],
            integration_type: IntegrationType::Cli,
            executable: ExecutableInfo {
                name: "tool.exe".to_string(),
                path: "C:\\Tools\\tool.exe".to_string(),
            },
            version: None,
            discovery: DiscoveryInfo {
                source: DiscoverySource::Manual,
                discovered_at: "2026-11-03T00:00:00Z".to_string(),
            },
            verification: VerificationInfo {
                status: VerificationStatus::Unverified,
                source: "manual-import".to_string(),
            },
            platforms: vec![Platform::Windows],
            documentation_url: None,
            author: None,
            created_at: "2026-11-03T00:00:00Z".to_string(),
            updated_at: "2026-11-03T00:00:00Z".to_string(),
        }
    }

    fn tmp_files(dir: &Path) -> Vec<String> {
        fs::read_dir(dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".tmp"))
            .collect()
    }

    #[test]
    fn save_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        let profile = sample_profile("nmap", "Nmap");
        storage.save(&profile).unwrap();
        let loaded = storage.load("nmap").unwrap();
        assert_eq!(loaded, profile);
    }

    #[test]
    fn list_returns_profiles_sorted_and_skips_non_json() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("zeta", "Zeta")).unwrap();
        storage.save(&sample_profile("alpha", "Alpha")).unwrap();
        fs::write(storage.dir().join("notes.txt"), "x").unwrap();
        fs::write(storage.dir().join("leftover.json.tmp"), "{").unwrap();
        let report = storage.list();
        let ids: Vec<&str> = report.profiles.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, vec!["alpha", "zeta"]);
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn corrupt_file_yields_warning_not_error() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("good", "Good")).unwrap();
        fs::write(storage.dir().join("bad.json"), "{ not json").unwrap();
        let report = storage.list();
        assert_eq!(report.profiles.len(), 1);
        assert_eq!(report.profiles[0].id, "good");
        assert_eq!(report.warnings.len(), 1);
        assert!(report.warnings[0].contains("bad"));
    }

    #[test]
    fn remove_deletes_and_reports_missing() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("nmap", "Nmap")).unwrap();
        storage.remove("nmap").unwrap();
        let err = storage.load("nmap").expect_err("should be gone");
        assert_eq!(err.code, "not_found");
        let err = storage.remove("nmap").expect_err("second remove");
        assert_eq!(err.code, "not_found");
    }

    #[test]
    fn update_replaces_profile_atomically() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("nmap", "Nmap")).unwrap();
        let mut updated = sample_profile("nmap", "Nmap Updated");
        updated.created_at = "2026-11-03T00:00:00Z".to_string();
        storage.save(&updated).unwrap();
        let loaded = storage.load("nmap").unwrap();
        assert_eq!(loaded.name, "Nmap Updated");
        // exactly one profile file, no temp leftovers
        let files: Vec<String> = fs::read_dir(storage.dir())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(files, vec!["nmap.json"]);
        assert!(tmp_files(storage.dir()).is_empty());
    }

    #[cfg(windows)]
    #[test]
    fn failed_write_preserves_previous_profile() {
        // Failure injection: make the target file read-only so the atomic
        // rename cannot replace it. The previous profile must remain
        // intact and no temp file may be left behind.
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("nmap", "Original")).unwrap();

        let path = storage.dir().join("nmap.json");
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&path, perms).unwrap();

        let mut attempt = sample_profile("nmap", "Replacement");
        attempt.name = "Replacement".to_string();
        let err = storage.save(&attempt).expect_err("replace must fail");
        assert_eq!(err.code, "io");

        // previous valid profile is intact
        let loaded = storage.load("nmap").unwrap();
        assert_eq!(loaded.name, "Original");
        assert!(tmp_files(storage.dir()).is_empty());

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&path, perms).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn failed_write_preserves_previous_profile() {
        use std::os::unix::fs::PermissionsExt;
        // Failure injection: make the registry directory read-only so the
        // write cannot complete. Previous profile must remain intact.
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(&sample_profile("nmap", "Original")).unwrap();

        let mut perms = fs::metadata(storage.dir()).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(storage.dir(), perms).unwrap();

        let err = storage
            .save(&sample_profile("nmap", "Replacement"))
            .expect_err("write must fail");
        assert_eq!(err.code, "io");
        assert!(tmp_files(storage.dir()).is_empty());

        let mut perms = fs::metadata(storage.dir()).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(storage.dir(), perms).unwrap();

        let loaded = storage.load("nmap").unwrap();
        assert_eq!(loaded.name, "Original");
    }

    #[test]
    fn invalid_ids_are_rejected_before_any_filesystem_operation() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        for id in ["../evil", "..", "con", "has/slash", "trailing."] {
            let mut profile = sample_profile("valid", "V");
            profile.id = id.to_string();
            let err = storage.save(&profile).expect_err(id);
            assert_eq!(err.code, "invalid_profile");
            let err = storage.load(id).expect_err(id);
            assert_eq!(err.code, "invalid_profile");
            assert!(storage.exists(id).is_err());
        }
        // nothing was written anywhere (registry dir is still empty)
        assert_eq!(fs::read_dir(storage.dir()).unwrap().count(), 0);
    }

    #[test]
    fn missing_load_is_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        let err = storage.load("nope").expect_err("missing");
        assert_eq!(err.code, "not_found");
    }
}
