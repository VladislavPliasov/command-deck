use super::model::{
    RegistryError, ToolProfile, SCHEMA_VERSION,
};
use std::path::Path;

const MAX_ID_LEN: usize = 64;
const MAX_NAME_LEN: usize = 200;
const MAX_DESCRIPTION_LEN: usize = 4000;
const MAX_TAGS: usize = 16;
const MAX_TAG_LEN: usize = 32;

const RESERVED_DEVICE_NAMES: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

pub const ALLOWED_EXECUTABLE_EXTENSIONS: [&str; 2] = ["exe", "com"];

/// Validates a tool id for use as a registry filename on all supported
/// platforms. Must be called before any filesystem operation on the id.
pub fn validate_id(id: &str) -> Result<(), RegistryError> {
    if id.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "id must not be empty",
            "id",
        ));
    }
    if id.len() > MAX_ID_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("id must be at most {MAX_ID_LEN} characters"),
            "id",
        ));
    }
    let mut first = true;
    for c in id.chars() {
        let ok = c.is_ascii_lowercase()
            || c.is_ascii_digit()
            || c == '.'
            || c == '_'
            || c == '-';
        if !ok {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "id may contain only lowercase letters, digits, '.', '_' and '-'",
                "id",
            ));
        }
        if first && !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "id must start with a lowercase letter or digit",
                "id",
            ));
        }
        first = false;
    }
    if id.ends_with('.') || id.ends_with(' ') {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "id must not end with a dot or space",
            "id",
        ));
    }
    // Windows reserved device names also apply to the base name of
    // "NAME.ext" (e.g. "con.json" is reserved).
    let base = id.split('.').next().unwrap_or(id).to_ascii_uppercase();
    if RESERVED_DEVICE_NAMES.contains(&base.as_str()) {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("id must not use the reserved device name {base}"),
            "id",
        ));
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), RegistryError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "name must not be empty",
            "name",
        ));
    }
    if trimmed.len() > MAX_NAME_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("name must be at most {MAX_NAME_LEN} characters"),
            "name",
        ));
    }
    Ok(())
}

pub fn validate_description(description: &str) -> Result<(), RegistryError> {
    if description.len() > MAX_DESCRIPTION_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("description must be at most {MAX_DESCRIPTION_LEN} characters"),
            "description",
        ));
    }
    Ok(())
}

pub fn validate_tags(tags: &[String]) -> Result<(), RegistryError> {
    if tags.len() > MAX_TAGS {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_TAGS} tags are allowed"),
            "tags",
        ));
    }
    for tag in tags {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "tags must not be empty",
                "tags",
            ));
        }
        if trimmed.len() > MAX_TAG_LEN {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("each tag must be at most {MAX_TAG_LEN} characters"),
                "tags",
            ));
        }
    }
    Ok(())
}

/// Accepts only absolute http(s) URLs, e.g. for documentation links.
pub fn validate_optional_url(url: &str, field: &str) -> Result<(), RegistryError> {
    let lower = url.trim().to_ascii_lowercase();
    let rest = if let Some(r) = lower.strip_prefix("https://") {
        r
    } else if let Some(r) = lower.strip_prefix("http://") {
        r
    } else {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "URL must start with http:// or https://",
            field,
        ));
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    if host.is_empty() || host.contains(' ') {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "URL must contain a valid host",
            field,
        ));
    }
    Ok(())
}

/// Format-level executable path validation (no filesystem access).
/// Existence and file-type checks are performed separately by the service.
pub fn validate_path_format(path: &str) -> Result<(), RegistryError> {
    if path.trim().is_empty() {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path must not be empty",
            "executablePath",
        ));
    }
    if path.contains('\0') {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path contains invalid characters",
            "executablePath",
        ));
    }
    let p = Path::new(path);
    if !p.is_absolute() {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path must be an absolute path",
            "executablePath",
        ));
    }
    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !ALLOWED_EXECUTABLE_EXTENSIONS.contains(&ext.as_str()) {
        return Err(RegistryError::with_field(
            "invalid_path",
            "only .exe and .com executables are supported",
            "executablePath",
        ));
    }
    Ok(())
}

pub fn validate_schema_version(version: &str) -> Result<(), RegistryError> {
    if version != SCHEMA_VERSION {
        return Err(RegistryError::new(
            "unsupported_schema_version",
            format!("unsupported profile schemaVersion {version}, expected {SCHEMA_VERSION}"),
        ));
    }
    Ok(())
}

/// Full validation of a stored or incoming profile.
pub fn validate_profile(profile: &ToolProfile) -> Result<(), RegistryError> {
    validate_schema_version(&profile.schema_version)?;
    validate_id(&profile.id)?;
    validate_name(&profile.name)?;
    if let Some(desc) = &profile.description {
        validate_description(desc)?;
    }
    validate_tags(&profile.tags)?;
    validate_path_format(&profile.executable.path)?;
    if profile.executable.name.trim().is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "executable name must not be empty",
            "executable.name",
        ));
    }
    if profile.platforms.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "at least one platform is required",
            "platforms",
        ));
    }
    if let Some(url) = &profile.documentation_url {
        validate_optional_url(url, "documentationUrl")?;
    }
    if let Some(author) = &profile.author {
        if author.name.trim().is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "author name must not be empty",
                "author.name",
            ));
        }
        if let Some(url) = &author.url {
            validate_optional_url(url, "author.url")?;
        }
    }
    if profile.created_at.trim().is_empty() || profile.updated_at.trim().is_empty() {
        return Err(RegistryError::new(
            "invalid_profile",
            "createdAt and updatedAt must not be empty",
        ));
    }
    Ok(())
}

/// Derives a filesystem-safe slug from a tool name. Returns an error if no
/// valid id can be derived.
pub fn derive_id(name: &str) -> Result<String, RegistryError> {
    let mut slug = String::new();
    let mut last_dash = false;
    for c in name.trim().chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "cannot derive a valid id from the given name; provide an explicit id",
            "id",
        ));
    }
    validate_id(&slug)?;
    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::model::*;

    fn valid_profile() -> ToolProfile {
        ToolProfile {
            schema_version: SCHEMA_VERSION.to_string(),
            id: "nmap".to_string(),
            name: "Nmap".to_string(),
            description: Some("Network scanner".to_string()),
            category: Category::Scanning,
            tags: vec!["network".to_string()],
            integration_type: IntegrationType::Cli,
            executable: ExecutableInfo {
                name: "nmap.exe".to_string(),
                path: "C:\\Tools\\nmap.exe".to_string(),
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
            documentation_url: Some("https://nmap.org".to_string()),
            author: None,
            created_at: "2026-11-03T00:00:00Z".to_string(),
            updated_at: "2026-11-03T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn ids_accept_valid_forms() {
        for id in ["nmap", "nmap2", "wireshark-ce", "hashcat_2", "tool.v2", "a"] {
            validate_id(id).unwrap_or_else(|e| panic!("{id} should be valid: {e}"));
        }
    }

    #[test]
    fn ids_reject_invalid_forms() {
        for id in [
            "",              // empty
            ".hidden",       // leading dot
            "-lead",         // leading dash
            "Upper",         // uppercase
            "has space",     // space
            "has/slash",     // separator
            "has\\back",     // separator
            "trailing.",     // trailing dot
            "a..b..",        // trailing dot
            "col:on",        // invalid char
        ] {
            let id: String = id.to_string();
            validate_id(&id).expect_err(&format!("{id} should be rejected"));
        }
        validate_id(&"x".repeat(65)).expect_err("too long");
    }

    #[test]
    fn ids_reject_windows_reserved_device_names() {
        for id in [
            "con", "prn", "aux", "nul", "com1", "com9", "lpt1", "lpt9",
            "con.json", // reserved base name with extension-like id
        ] {
            validate_id(id).expect_err(&format!("{id} should be rejected"));
        }
        // com0/lpt0 and com10 are not reserved
        for id in ["com0", "lpt0", "com10", "console"] {
            validate_id(id).unwrap_or_else(|e| panic!("{id} should be valid: {e}"));
        }
    }

    #[test]
    fn name_rules() {
        validate_name("Nmap").unwrap();
        validate_name("  Nmap  ").unwrap();
        validate_name("").expect_err("empty name");
        validate_name("   ").expect_err("blank name");
        validate_name(&"x".repeat(201)).expect_err("too long");
    }

    #[test]
    fn description_and_tag_rules() {
        validate_description(&"d".repeat(4000)).unwrap();
        validate_description(&"d".repeat(4001)).expect_err("too long");
        validate_tags(&[]).unwrap();
        validate_tags(&vec!["a".to_string(); 16]).unwrap();
        validate_tags(&vec!["a".to_string(); 17]).expect_err("too many");
        validate_tags(&["".to_string()]).expect_err("empty tag");
        validate_tags(&["x".repeat(33)]).expect_err("tag too long");
    }

    #[test]
    fn url_rules() {
        validate_optional_url("https://nmap.org/book", "u").unwrap();
        validate_optional_url("http://localhost:8080/docs", "u").unwrap();
        validate_optional_url("javascript:alert(1)", "u").expect_err("js scheme");
        validate_optional_url("file:///etc/passwd", "u").expect_err("file scheme");
        validate_optional_url("https://", "u").expect_err("no host");
        validate_optional_url("nmap.org", "u").expect_err("no scheme");
    }

    #[test]
    fn path_format_rules() {
        validate_path_format("C:\\Tools\\nmap.exe").unwrap();
        validate_path_format("C:\\Tools\\nmap.EXE").unwrap(); // case-insensitive ext
        validate_path_format("C:\\Tools\\cmd.com").unwrap();
        validate_path_format("C:\\Tools\\nmap.bat").expect_err("bat unsupported");
        validate_path_format("C:\\Tools\\run.ps1").expect_err("ps1 unsupported");
        validate_path_format("nmap.exe").expect_err("relative path");
        validate_path_format("").expect_err("empty");
    }

    #[test]
    fn schema_version_must_match() {
        validate_schema_version("1.0").unwrap();
        validate_schema_version("2.0").expect_err("future major");
    }

    #[test]
    fn full_profile_validation() {
        let mut p = valid_profile();
        validate_profile(&p).unwrap();
        p.id = "CON".to_string();
        validate_profile(&p).expect_err("reserved id");
    }

    #[test]
    fn derive_id_slugs_names() {
        assert_eq!(derive_id("Nmap").unwrap(), "nmap");
        assert_eq!(derive_id("John the Ripper").unwrap(), "john-the-ripper");
        assert_eq!(derive_id("  SQLMap! ").unwrap(), "sqlmap");
        assert_eq!(derive_id("Wireshark (GUI)").unwrap(), "wireshark-gui");
        derive_id("!!!").expect_err("no alnum");
    }

    #[test]
    fn canonical_profile_serialization_uses_expected_keys() {
        // Anti-drift guard between Rust model and schemas/tool-profile.schema.json:
        // if a field is renamed here, the published schema must be updated too.
        let value = serde_json::to_value(valid_profile()).unwrap();
        let obj = value.as_object().unwrap();
        let mut keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            vec![
                "category",
                "createdAt",
                "description",
                "discovery",
                "documentationUrl",
                "executable",
                "id",
                "integrationType",
                "name",
                "platforms",
                "schemaVersion",
                "tags",
                "updatedAt",
                "verification",
            ]
        );
    }

    #[test]
    fn category_values_match_product_specification() {
        let names: Vec<&str> = Category::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "Reconnaissance",
                "Scanning",
                "Web Application Security",
                "Exploitation",
                "Password Security Testing",
                "Wireless Security",
                "Network Security",
                "Privilege Escalation",
                "Post-Exploitation",
                "Active Directory and Windows Security",
                "Reverse Engineering",
                "Malware Analysis",
                "Digital Forensics",
                "Social Engineering",
                "Defensive Security",
                "other",
            ]
        );
    }
}
