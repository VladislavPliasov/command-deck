use super::model::*;
use super::storage::Storage;
use super::validation;
use super::{now_rfc3339, version_info};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RegistryService {
    storage: Storage,
}

impl RegistryService {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn list(&self) -> ToolListResult {
        let report = self.storage.list();
        let tools = report
            .profiles
            .iter()
            .map(|profile| self.summary(profile))
            .collect();
        ToolListResult {
            tools,
            warnings: report.warnings,
        }
    }

    pub fn get(&self, id: &str) -> Result<ToolProfileView, RegistryError> {
        let profile = self.storage.load(id)?;
        Ok(self.view(profile))
    }

    pub fn register(&self, req: RegisterToolRequest) -> Result<ToolProfileView, RegistryError> {
        let name = match req.name {
            Some(name) => name.trim().to_string(),
            None => file_stem_of(&req.executable_path).ok_or_else(|| {
                RegistryError::with_field(
                    "invalid_profile",
                    "cannot derive a name from the executable path; provide an explicit name",
                    "name",
                )
            })?,
        };
        validation::validate_name(&name)?;
        let id = match req.id {
            Some(id) => id,
            None => validation::derive_id(&name)?,
        };
        validation::validate_id(&id)?;

        validation::validate_path_format(&req.executable_path)?;
        let canonical = canonicalize_executable(&req.executable_path)?;

        if self.storage.exists(&id)? {
            return Err(RegistryError::with_field(
                "duplicate_id",
                format!("a tool with id '{id}' is already registered"),
                "id",
            ));
        }
        self.ensure_unique_path(&canonical, None)?;

        if let Some(desc) = &req.description {
            validation::validate_description(desc)?;
        }
        let tags = req.tags.unwrap_or_default();
        validation::validate_tags(&tags)?;
        if let Some(url) = &req.documentation_url {
            validation::validate_optional_url(url, "documentationUrl")?;
        }
        if let Some(author) = &req.author {
            validation::validate_name(&author.name).map_err(|_| {
                RegistryError::with_field(
                    "invalid_profile",
                    "author name must not be empty",
                    "author.name",
                )
            })?;
            if let Some(url) = &author.url {
                validation::validate_optional_url(url, "author.url")?;
            }
        }

        let now = now_rfc3339();
        let version = version_info::read_file_version(Path::new(&canonical)).map(|value| {
            VersionInfo {
                value,
                source: VersionSource::FileVersion,
            }
        });
        let profile = ToolProfile {
            schema_version: SCHEMA_VERSION.to_string(),
            id,
            name,
            description: req.description,
            category: req.category.unwrap_or(Category::Other),
            tags,
            integration_type: req.integration_type.unwrap_or(IntegrationType::Cli),
            executable: ExecutableInfo {
                name: file_name_of(&canonical),
                path: canonical,
            },
            version,
            discovery: DiscoveryInfo {
                source: DiscoverySource::Manual,
                discovered_at: now.clone(),
            },
            verification: VerificationInfo {
                status: VerificationStatus::Unverified,
                source: "manual-import".to_string(),
                tool_version: None,
            },
            platforms: default_platforms(),
            commands: None,
            documentation_url: req.documentation_url,
            author: req.author,
            created_at: now.clone(),
            updated_at: now,
        };
        validation::validate_profile(&profile)?;
        self.storage.save(&profile)?;
        Ok(self.view(profile))
    }

    pub fn update(
        &self,
        id: &str,
        req: UpdateToolRequest,
    ) -> Result<ToolProfileView, RegistryError> {
        let mut profile = self.storage.load(id)?;

        if let Some(name) = req.name {
            let name = name.trim().to_string();
            validation::validate_name(&name)?;
            profile.name = name;
        }
        if let Some(desc) = req.description {
            // empty string clears the optional description
            let desc = desc.trim().to_string();
            if desc.is_empty() {
                profile.description = None;
            } else {
                validation::validate_description(&desc)?;
                profile.description = Some(desc);
            }
        }
        if let Some(category) = req.category {
            profile.category = category;
        }
        if let Some(tags) = req.tags {
            validation::validate_tags(&tags)?;
            profile.tags = tags;
        }
        if let Some(integration_type) = req.integration_type {
            profile.integration_type = integration_type;
        }
        if let Some(url) = req.documentation_url {
            let url = url.trim().to_string();
            if url.is_empty() {
                profile.documentation_url = None;
            } else {
                validation::validate_optional_url(&url, "documentationUrl")?;
                profile.documentation_url = Some(url);
            }
        }
        if let Some(author) = req.author {
            validation::validate_name(&author.name).map_err(|_| {
                RegistryError::with_field(
                    "invalid_profile",
                    "author name must not be empty",
                    "author.name",
                )
            })?;
            if let Some(url) = &author.url {
                validation::validate_optional_url(url, "author.url")?;
            }
            profile.author = Some(author);
        }
        if let Some(path) = req.executable_path {
            validation::validate_path_format(&path)?;
            let canonical = canonicalize_executable(&path)?;
            self.ensure_unique_path(&canonical, Some(&profile.id))?;
            profile.version =
                version_info::read_file_version(Path::new(&canonical)).map(|value| VersionInfo {
                    value,
                    source: VersionSource::FileVersion,
                });
            profile.executable = ExecutableInfo {
                name: file_name_of(&canonical),
                path: canonical,
            };
        }

        if let Some(commands) = req.commands {
            let previous = profile.commands.clone();
            // Deterministic structural equality (PartialEq): array order is
            // significant, strings compare exactly, None != Some([]).
            let changed = previous.as_ref() != Some(&commands);
            profile.commands = Some(commands);
            if changed {
                // D1: command metadata changes invalidate verification for
                // `verified` and `partially-verified` profiles only.
                if matches!(
                    profile.verification.status,
                    VerificationStatus::Verified | VerificationStatus::PartiallyVerified
                ) {
                    profile.verification.status = VerificationStatus::Unverified;
                }
            }
        }

        // Stamping policy: every explicit profile write stamps the current
        // schema version. Reads never write. verification.toolVersion is
        // never written here (nor anywhere else in P0-04).
        profile.schema_version = SCHEMA_VERSION.to_string();
        profile.updated_at = now_rfc3339();
        validation::validate_profile(&profile)?;
        self.storage.save(&profile)?;
        Ok(self.view(profile))
    }

    pub fn remove(&self, id: &str) -> Result<(), RegistryError> {
        self.storage.remove(id)
    }

    fn view(&self, profile: ToolProfile) -> ToolProfileView {
        let install_status = self.install_status(&profile.executable.path);
        ToolProfileView {
            profile,
            install_status,
        }
    }

    fn summary(&self, profile: &ToolProfile) -> ToolSummaryView {
        ToolSummaryView {
            id: profile.id.clone(),
            name: profile.name.clone(),
            category: profile.category,
            integration_type: profile.integration_type,
            executable_name: profile.executable.name.clone(),
            install_status: self.install_status(&profile.executable.path),
            verification_status: profile.verification.status,
        }
    }

    fn install_status(&self, path: &str) -> InstallStatus {
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_file() => InstallStatus::Installed,
            Ok(_) => InstallStatus::NotFound,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => InstallStatus::NotFound,
            Err(_) => InstallStatus::Unknown,
        }
    }

    /// Duplicate executable paths are rejected (P0-03 decision): two
    /// profiles may not point at the same canonical file.
    fn ensure_unique_path(
        &self,
        canonical: &str,
        exclude_id: Option<&str>,
    ) -> Result<(), RegistryError> {
        for profile in self.storage.list().profiles {
            if Some(profile.id.as_str()) == exclude_id {
                continue;
            }
            let same = if cfg!(windows) {
                profile.executable.path.eq_ignore_ascii_case(canonical)
            } else {
                profile.executable.path == canonical
            };
            if same {
                return Err(RegistryError::with_field(
                    "duplicate_executable",
                    format!(
                        "this executable is already registered as tool '{}'",
                        profile.id
                    ),
                    "executablePath",
                ));
            }
        }
        Ok(())
    }
}

fn canonicalize_executable(path: &str) -> Result<String, RegistryError> {
    let p = Path::new(path);
    let meta = std::fs::metadata(p).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            RegistryError::with_field(
                "invalid_path",
                "executable file does not exist",
                "executablePath",
            )
        } else {
            RegistryError::from(err)
        }
    })?;
    if !meta.is_file() {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path must point to a file",
            "executablePath",
        ));
    }
    let canonical: PathBuf = dunce::canonicalize(p)
        .map_err(|_| RegistryError::with_field("invalid_path", "cannot resolve path", "executablePath"))?;
    Ok(canonical.to_string_lossy().to_string())
}

fn file_name_of(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn file_stem_of(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .map(|n| n.to_string_lossy().trim().to_string())
        .filter(|n| !n.is_empty())
}

fn default_platforms() -> Vec<Platform> {
    if cfg!(windows) {
        vec![Platform::Windows]
    } else {
        vec![Platform::Linux]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a real dummy executable file in a temp dir (never executed).
    fn dummy_exe(dir: &Path, name: &str) -> String {
        let path = dir.join(name);
        std::fs::write(&path, b"MZ-dummy").unwrap();
        dunce::canonicalize(&path)
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    fn service_with_dir() -> (tempfile::TempDir, RegistryService) {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        (dir, RegistryService::new(storage))
    }

    fn request(path: &str) -> RegisterToolRequest {
        RegisterToolRequest {
            id: None,
            name: None,
            executable_path: path.to_string(),
            integration_type: None,
            category: None,
            tags: None,
            description: None,
            documentation_url: None,
            author: None,
        }
    }

    #[test]
    fn register_derives_id_and_defaults() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "Nmap Scanner.exe");
        let view = service.register(request(&path)).unwrap();
        assert_eq!(view.profile.id, "nmap-scanner");
        assert_eq!(view.profile.name, "Nmap Scanner");
        assert_eq!(view.profile.integration_type, IntegrationType::Cli);
        assert_eq!(view.profile.category, Category::Other);
        assert_eq!(view.profile.verification.status, VerificationStatus::Unverified);
        assert_eq!(view.profile.verification.source, "manual-import");
        assert_eq!(view.profile.discovery.source, DiscoverySource::Manual);
        assert_eq!(view.install_status, InstallStatus::Installed);
        assert!(view.profile.version.is_none()); // dummy file has no PE resource
    }

    #[test]
    fn register_honors_explicit_id_and_fields() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let mut req = request(&path);
        req.id = Some("my-tool".to_string());
        req.name = Some("My Tool".to_string());
        req.category = Some(Category::Scanning);
        req.integration_type = Some(IntegrationType::Gui);
        req.tags = Some(vec!["a".to_string()]);
        req.description = Some("d".to_string());
        req.documentation_url = Some("https://example.com".to_string());
        let view = service.register(req).unwrap();
        assert_eq!(view.profile.id, "my-tool");
        assert_eq!(view.profile.category, Category::Scanning);
        assert_eq!(view.profile.integration_type, IntegrationType::Gui);
        assert_eq!(view.profile.platforms, vec![default_platforms()[0]]);
    }

    #[test]
    fn register_rejects_duplicate_id() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let mut req = request(&path);
        req.id = Some("same".to_string());
        service.register(req.clone()).unwrap();
        let err = service.register(req).expect_err("duplicate id");
        assert_eq!(err.code, "duplicate_id");
    }

    #[test]
    fn register_rejects_duplicate_executable_path() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        service.register(request(&path)).unwrap();
        let mut req = request(&path);
        req.id = Some("second".to_string());
        req.name = Some("Second".to_string());
        let err = service.register(req).expect_err("duplicate path");
        assert_eq!(err.code, "duplicate_executable");
        assert_eq!(err.field.as_deref(), Some("executablePath"));
    }

    #[test]
    fn register_validates_paths() {
        let (dir, service) = service_with_dir();
        // missing file
        let err = service
            .register(request("C:\\does\\not\\exist.exe"))
            .expect_err("missing");
        assert_eq!(err.code, "invalid_path");
        // relative path
        let err = service.register(request("tool.exe")).expect_err("relative");
        assert_eq!(err.code, "invalid_path");
        // unsupported extension
        let bat = dir.path().join("tool.bat");
        std::fs::write(&bat, b"").unwrap();
        let err = service
            .register(request(&bat.to_string_lossy()))
            .expect_err("bat");
        assert_eq!(err.code, "invalid_path");
        // directory instead of file
        let err = service
            .register(request(&dir.path().to_string_lossy()))
            .expect_err("dir");
        assert_eq!(err.code, "invalid_path");
    }

    #[test]
    fn register_rejects_invalid_explicit_id() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let mut req = request(&path);
        req.id = Some("con".to_string());
        let err = service.register(req).expect_err("reserved id");
        assert_eq!(err.code, "invalid_profile");
    }

    #[test]
    fn update_modifies_fields_but_not_identity() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let created = service.register(request(&path)).unwrap();
        let req = UpdateToolRequest {
            name: Some("Renamed".to_string()),
            description: Some("new desc".to_string()),
            category: Some(Category::Scanning),
            tags: Some(vec!["x".to_string()]),
            integration_type: Some(IntegrationType::Gui),
            documentation_url: Some("https://example.com/x".to_string()),
            author: Some(AuthorInfo {
                name: "Me".to_string(),
                url: None,
            }),
            executable_path: None,
            commands: None,
        };
        let updated = service.update(&created.profile.id, req).unwrap();
        assert_eq!(updated.profile.name, "Renamed");
        assert_eq!(updated.profile.category, Category::Scanning);
        assert_eq!(updated.profile.id, created.profile.id);
        assert_eq!(updated.profile.created_at, created.profile.created_at);
        assert!(updated.profile.updated_at >= created.profile.updated_at);
    }

    #[test]
    fn update_path_change_revalidates_and_excludes_self() {
        let (dir, service) = service_with_dir();
        let path_a = dummy_exe(dir.path(), "a.exe");
        let path_b = dummy_exe(dir.path(), "b.exe");
        let a = service.register(request(&path_a)).unwrap();
        // changing to another existing file is fine
        let req = UpdateToolRequest {
            name: None,
            description: None,
            category: None,
            tags: None,
            integration_type: None,
            documentation_url: None,
            author: None,
            executable_path: Some(path_b.clone()),
            commands: None,
        };
        let updated = service.update(&a.profile.id, req).unwrap();
        assert_eq!(updated.profile.executable.path, path_b);
        // changing to a nonexistent file fails
        let req = UpdateToolRequest {
            executable_path: Some("C:\\missing\\x.exe".to_string()),
            ..empty_update()
        };
        let err = service.update(&a.profile.id, req).expect_err("missing");
        assert_eq!(err.code, "invalid_path");
    }

    #[test]
    fn remove_and_get_semantics() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let created = service.register(request(&path)).unwrap();
        service.get(&created.profile.id).unwrap();
        service.remove(&created.profile.id).unwrap();
        let err = service.get(&created.profile.id).expect_err("gone");
        assert_eq!(err.code, "not_found");
        let err = service.remove(&created.profile.id).expect_err("gone");
        assert_eq!(err.code, "not_found");
    }

    #[test]
    fn list_returns_summaries_with_install_status() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let created = service.register(request(&path)).unwrap();
        // remove the executable file -> install status flips to not-found
        std::fs::remove_file(&path).unwrap();
        let result = service.list();
        assert_eq!(result.tools.len(), 1);
        assert_eq!(result.tools[0].id, created.profile.id);
        assert_eq!(result.tools[0].install_status, InstallStatus::NotFound);
    }

    fn empty_update() -> UpdateToolRequest {
        UpdateToolRequest {
            name: None,
            description: None,
            category: None,
            tags: None,
            integration_type: None,
            documentation_url: None,
            author: None,
            executable_path: None,
            commands: None,
        }
    }

    fn make_profile(
        id: &str,
        status: VerificationStatus,
        tool_version: Option<&str>,
        commands: Option<Vec<CommandSpec>>,
        schema_version: &str,
    ) -> ToolProfile {
        ToolProfile {
            schema_version: schema_version.to_string(),
            id: id.to_string(),
            name: id.to_string(),
            description: Some("desc".to_string()),
            category: Category::Scanning,
            tags: vec![],
            integration_type: IntegrationType::Cli,
            executable: ExecutableInfo {
                name: "tool.exe".to_string(),
                path: "C:\\Tools\\tool.exe".to_string(),
            },
            version: None,
            discovery: DiscoveryInfo {
                source: DiscoverySource::Manual,
                discovered_at: "2026-11-04T00:00:00Z".to_string(),
            },
            verification: VerificationInfo {
                status,
                source: "manual-import".to_string(),
                tool_version: tool_version.map(String::from),
            },
            platforms: vec![Platform::Windows],
            commands,
            documentation_url: None,
            author: None,
            created_at: "2026-11-04T00:00:00Z".to_string(),
            updated_at: "2026-11-04T00:00:00Z".to_string(),
        }
    }

    fn sample_commands() -> Vec<CommandSpec> {
        serde_json::from_str(
            r#"[{"id":"root","token":null,"name":"Direct","parameters":[
                {"id":"a","kind":"flag","label":"A","token":"-a"},
                {"id":"b","kind":"flag","label":"B","token":"-b"}],"subcommands":[]}]"#,
        )
        .unwrap()
    }

    fn sample_commands_reordered() -> Vec<CommandSpec> {
        serde_json::from_str(
            r#"[{"id":"root","token":null,"name":"Direct","parameters":[
                {"id":"b","kind":"flag","label":"B","token":"-b"},
                {"id":"a","kind":"flag","label":"A","token":"-a"}],"subcommands":[]}]"#,
        )
        .unwrap()
    }

    fn service_with_stored(profile: &ToolProfile) -> (tempfile::TempDir, RegistryService, Storage) {
        let dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(dir.path().join("registry")).unwrap();
        storage.save(profile).unwrap();
        let service = RegistryService::new(Storage::new(dir.path().join("registry")).unwrap());
        (dir, service, storage)
    }

    #[test]
    fn update_sets_preserves_and_clears_commands() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let created = service.register(request(&path)).unwrap();
        assert!(created.profile.commands.is_none());

        let updated = service
            .update(
                &created.profile.id,
                UpdateToolRequest {
                    commands: Some(sample_commands()),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.commands.as_ref().unwrap().len(), 1);

        // omitted commands field leaves metadata unchanged
        let updated = service
            .update(
                &created.profile.id,
                UpdateToolRequest {
                    name: Some("Renamed".to_string()),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.name, "Renamed");
        assert_eq!(updated.profile.commands.as_ref().unwrap().len(), 1);

        // Some([]) clears and is stored as an empty array
        let updated = service
            .update(
                &created.profile.id,
                UpdateToolRequest {
                    commands: Some(vec![]),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.commands, Some(vec![]));
    }

    #[test]
    fn downgrade_matrix_when_commands_change() {
        for (status, expected) in [
            (VerificationStatus::Verified, VerificationStatus::Unverified),
            (
                VerificationStatus::PartiallyVerified,
                VerificationStatus::Unverified,
            ),
            (
                VerificationStatus::ManuallyConfigured,
                VerificationStatus::ManuallyConfigured,
            ),
            (VerificationStatus::Incompatible, VerificationStatus::Incompatible),
            (VerificationStatus::Unverified, VerificationStatus::Unverified),
        ] {
            let (_dir, service, storage) = service_with_stored(&make_profile(
                "tool",
                status,
                Some("7.95"),
                Some(sample_commands()),
                "1.1",
            ));
            let mut changed = sample_commands();
            changed[0].name = "Changed".to_string();
            let updated = service
                .update(
                    "tool",
                    UpdateToolRequest {
                        commands: Some(changed),
                        ..empty_update()
                    },
                )
                .unwrap();
            assert_eq!(updated.profile.verification.status, expected, "{status:?}");
            assert_eq!(
                updated.profile.verification.tool_version.as_deref(),
                Some("7.95"),
                "toolVersion must never be written"
            );
            let stored = storage.load("tool").unwrap();
            assert_eq!(stored.verification.status, expected);
            assert_eq!(stored.verification.tool_version.as_deref(), Some("7.95"));
        }
    }

    #[test]
    fn identical_commands_do_not_change_status() {
        let (_dir, service, _) = service_with_stored(&make_profile(
            "tool",
            VerificationStatus::Verified,
            None,
            Some(sample_commands()),
            "1.1",
        ));
        let updated = service
            .update(
                "tool",
                UpdateToolRequest {
                    commands: Some(sample_commands()),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.verification.status, VerificationStatus::Verified);
    }

    #[test]
    fn reordered_parameters_count_as_change() {
        let (_dir, service, _) = service_with_stored(&make_profile(
            "tool",
            VerificationStatus::Verified,
            None,
            Some(sample_commands()),
            "1.1",
        ));
        let updated = service
            .update(
                "tool",
                UpdateToolRequest {
                    commands: Some(sample_commands_reordered()),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.verification.status, VerificationStatus::Unverified);
    }

    #[test]
    fn clearing_absent_commands_counts_as_change() {
        // None vs Some([]) is a structural change (documented behavior).
        let (_dir, service, _) = service_with_stored(&make_profile(
            "tool",
            VerificationStatus::Verified,
            None,
            None,
            "1.1",
        ));
        let updated = service
            .update(
                "tool",
                UpdateToolRequest {
                    commands: Some(vec![]),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.verification.status, VerificationStatus::Unverified);
    }

    #[test]
    fn writes_stamp_schema_version_and_preserve_values() {
        let (dir, service, storage) = service_with_stored(&make_profile(
            "legacy",
            VerificationStatus::Unverified,
            None,
            None,
            "1.0",
        ));
        let updated = service
            .update(
                "legacy",
                UpdateToolRequest {
                    name: Some("New Name".to_string()),
                    ..empty_update()
                },
            )
            .unwrap();
        assert_eq!(updated.profile.schema_version, "1.1");
        let stored = storage.load("legacy").unwrap();
        assert_eq!(stored.schema_version, "1.1");
        assert_eq!(stored.name, "New Name");
        assert_eq!(stored.description.as_deref(), Some("desc"));
        assert!(stored.commands.is_none(), "absent fields stay absent");
        // register also stamps the current version
        let path = dummy_exe(dir.path(), "tool2.exe");
        let created = service.register(request(&path)).unwrap();
        assert_eq!(created.profile.schema_version, "1.1");
    }

    #[test]
    fn command_validation_errors_carry_field_paths() {
        let (dir, service) = service_with_dir();
        let path = dummy_exe(dir.path(), "tool.exe");
        let created = service.register(request(&path)).unwrap();
        let invalid: Vec<CommandSpec> = serde_json::from_str(
            r#"[{"id":"root","token":null,"name":"Direct","parameters":[
                {"id":"f","kind":"flag","label":"F","token":"-v","value":{"valueType":"string"}}],
                "subcommands":[]}]"#,
        )
        .unwrap();
        let err = service
            .update(
                &created.profile.id,
                UpdateToolRequest {
                    commands: Some(invalid),
                    ..empty_update()
                },
            )
            .expect_err("flag with value");
        assert_eq!(
            err.field.as_deref(),
            Some("commands[0].parameters[0].value")
        );
    }

    #[test]
    fn reads_do_not_rewrite_stored_profiles() {
        let (_dir, service, storage) = service_with_stored(&make_profile(
            "tool",
            VerificationStatus::Verified,
            Some("7.95"),
            Some(sample_commands()),
            "1.0",
        ));
        let before = std::fs::read(storage.dir().join("tool.json")).unwrap();
        service.get("tool").unwrap();
        service.list();
        let after = std::fs::read(storage.dir().join("tool.json")).unwrap();
        assert_eq!(before, after, "reads must not rewrite the file");
    }
}
