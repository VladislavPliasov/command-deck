use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: &str = "1.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntegrationType {
    Cli,
    Gui,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Reconnaissance")]
    Reconnaissance,
    #[serde(rename = "Scanning")]
    Scanning,
    #[serde(rename = "Web Application Security")]
    WebApplicationSecurity,
    #[serde(rename = "Exploitation")]
    Exploitation,
    #[serde(rename = "Password Security Testing")]
    PasswordSecurityTesting,
    #[serde(rename = "Wireless Security")]
    WirelessSecurity,
    #[serde(rename = "Network Security")]
    NetworkSecurity,
    #[serde(rename = "Privilege Escalation")]
    PrivilegeEscalation,
    #[serde(rename = "Post-Exploitation")]
    PostExploitation,
    #[serde(rename = "Active Directory and Windows Security")]
    ActiveDirectoryAndWindowsSecurity,
    #[serde(rename = "Reverse Engineering")]
    ReverseEngineering,
    #[serde(rename = "Malware Analysis")]
    MalwareAnalysis,
    #[serde(rename = "Digital Forensics")]
    DigitalForensics,
    #[serde(rename = "Social Engineering")]
    SocialEngineering,
    #[serde(rename = "Defensive Security")]
    DefensiveSecurity,
    #[serde(rename = "other")]
    Other,
}

impl Category {
    // Category list shared with tests and future tooling; values must
    // match PRODUCT_SPECIFICATION.md section 5 verbatim.
    #[allow(dead_code)]
    pub const ALL: [Category; 16] = [
        Category::Reconnaissance,
        Category::Scanning,
        Category::WebApplicationSecurity,
        Category::Exploitation,
        Category::PasswordSecurityTesting,
        Category::WirelessSecurity,
        Category::NetworkSecurity,
        Category::PrivilegeEscalation,
        Category::PostExploitation,
        Category::ActiveDirectoryAndWindowsSecurity,
        Category::ReverseEngineering,
        Category::MalwareAnalysis,
        Category::DigitalForensics,
        Category::SocialEngineering,
        Category::DefensiveSecurity,
        Category::Other,
    ];

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Reconnaissance => "Reconnaissance",
            Category::Scanning => "Scanning",
            Category::WebApplicationSecurity => "Web Application Security",
            Category::Exploitation => "Exploitation",
            Category::PasswordSecurityTesting => "Password Security Testing",
            Category::WirelessSecurity => "Wireless Security",
            Category::NetworkSecurity => "Network Security",
            Category::PrivilegeEscalation => "Privilege Escalation",
            Category::PostExploitation => "Post-Exploitation",
            Category::ActiveDirectoryAndWindowsSecurity => {
                "Active Directory and Windows Security"
            }
            Category::ReverseEngineering => "Reverse Engineering",
            Category::MalwareAnalysis => "Malware Analysis",
            Category::DigitalForensics => "Digital Forensics",
            Category::SocialEngineering => "Social Engineering",
            Category::DefensiveSecurity => "Defensive Security",
            Category::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    Windows,
    Linux,
    Macos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VersionSource {
    Manual,
    FileVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub value: String,
    pub source: VersionSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiscoverySource {
    Manual,
    PathScan,
    Import,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryInfo {
    pub source: DiscoverySource,
    pub discovered_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationStatus {
    Unverified,
    PartiallyVerified,
    Verified,
    ManuallyConfigured,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationInfo {
    pub status: VerificationStatus,
    pub source: String,
    /// Version the command metadata was originally authored/verified
    /// against. Historical reference only - not a compatibility claim.
    /// Never auto-populated, overwritten, or cleared by application logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutableInfo {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProfile {
    pub schema_version: String,
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub category: Category,
    #[serde(default)]
    pub tags: Vec<String>,
    pub integration_type: IntegrationType,
    pub executable: ExecutableInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<VersionInfo>,
    pub discovery: DiscoveryInfo,
    pub verification: VerificationInfo,
    pub platforms: Vec<Platform>,
    /// Structured command/parameter metadata (schema 1.1). Omitted entirely
    /// when absent - never serialized as null. `Some([])` serializes as
    /// an empty array and is structurally distinct from `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<CommandSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<AuthorInfo>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallStatus {
    Installed,
    NotFound,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolProfileView {
    #[serde(flatten)]
    pub profile: ToolProfile,
    pub install_status: InstallStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolSummaryView {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub integration_type: IntegrationType,
    pub executable_name: String,
    pub install_status: InstallStatus,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolListResult {
    pub tools: Vec<ToolSummaryView>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterToolRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    pub executable_path: String,
    #[serde(default)]
    pub integration_type: Option<IntegrationType>,
    #[serde(default)]
    pub category: Option<Category>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub documentation_url: Option<String>,
    #[serde(default)]
    pub author: Option<AuthorInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateToolRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<Category>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub integration_type: Option<IntegrationType>,
    #[serde(default)]
    pub documentation_url: Option<String>,
    #[serde(default)]
    pub author: Option<AuthorInfo>,
    #[serde(default)]
    pub executable_path: Option<String>,
    /// `None` = leave command metadata unchanged; `Some(v)` replaces it
    /// (`Some([])` clears it). An explicit JSON `null` is treated as
    /// unchanged.
    #[serde(default)]
    pub commands: Option<Vec<CommandSpec>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParameterKind {
    Flag,
    Option,
    Positional,
    Custom,
}

/// Value types, verbatim from TOOL_INTEGRATION.md section 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValueType {
    Boolean,
    String,
    Integer,
    Decimal,
    Enum,
    File,
    Directory,
    Ip,
    Cidr,
    Domain,
    Port,
    List,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSpec {
    pub value_type: ValueType,
    #[serde(default)]
    pub choices: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub optional: bool,
}

/// Fields allowed on `kind: "custom"` parameters (closed contract).
pub const CUSTOM_ALLOWED_FIELDS: [&str; 12] = [
    "id",
    "kind",
    "label",
    "token",
    "syntax",
    "description",
    "examples",
    "versionConstraints",
    "required",
    "repeatable",
    "requires",
    "conflicts",
];

/// Known ParameterSpec fields that are forbidden on `kind: "custom"`.
/// They deserialize normally (they are regular fields of the model) and are
/// rejected by semantic validation with field paths - see validation.rs.
pub const CUSTOM_SEMANTIC_FIELDS: [&str; 3] = ["value", "aliases", "order"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub id: String,
    pub kind: ParameterKind,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueSpec>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub repeatable: bool,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub conflicts: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub version_constraints: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

// Manual Deserialize for ParameterSpec - deliberate, documented design:
//
// 1. Duplicate JSON keys are rejected (`duplicate_field`), matching the
//    behavior of serde's derived impls. This is why the input is NOT
//    buffered through serde_json::Value/Map (which silently dedups keys):
//    fields are consumed directly from MapAccess into typed slots, so
//    nested types keep their derived duplicate rejection as well.
// 2. Unknown fields are tolerated for non-custom parameters (forward
//    compatibility within schema major version 1) but rejected for
//    `kind: "custom"` parameters, naming the offending key.
// 3. The known-but-forbidden fields for custom parameters (value, aliases,
//    order) are captured here and rejected later by semantic validation
//    with field paths (see CUSTOM_SEMANTIC_FIELDS).
// 4. `token` is required-nullable: a missing `token` is an error
//    ("missing field `token`"), explicit null is accepted.
impl<'de> Deserialize<'de> for ParameterSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ParameterSpecVisitor)
    }
}

struct ParameterSpecVisitor;

impl<'de> serde::de::Visitor<'de> for ParameterSpecVisitor {
    type Value = ParameterSpec;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct ParameterSpec")
    }

    fn visit_map<A>(self, mut map: A) -> Result<ParameterSpec, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error as DeError;

        let mut seen: Vec<String> = Vec::new();
        let mut unknown: Vec<String> = Vec::new();
        let mut id: Option<String> = None;
        let mut kind: Option<ParameterKind> = None;
        let mut label: Option<String> = None;
        let mut token: Option<String> = None;
        let mut aliases: Option<Vec<String>> = None;
        let mut description: Option<String> = None;
        let mut syntax: Option<String> = None;
        let mut value: Option<ValueSpec> = None;
        let mut required: bool = false;
        let mut repeatable: bool = false;
        let mut requires: Vec<String> = Vec::new();
        let mut conflicts: Vec<String> = Vec::new();
        let mut examples: Vec<String> = Vec::new();
        let mut version_constraints: Vec<String> = Vec::new();
        let mut order: Option<i64> = None;

        while let Some(key) = map.next_key::<String>()? {
            if seen.iter().any(|k| k == &key) {
                // Same message text as serde's derived duplicate-field
                // errors (duplicate_field() takes a &'static str, which a
                // dynamic key cannot provide).
                return Err(A::Error::custom(format!("duplicate field `{key}`")));
            }
            seen.push(key.clone());
            match key.as_str() {
                "id" => id = Some(map.next_value()?),
                "kind" => kind = Some(map.next_value()?),
                "label" => label = Some(map.next_value()?),
                "token" => token = map.next_value::<Option<String>>()?,
                "aliases" => aliases = Some(map.next_value()?),
                "description" => description = Some(map.next_value()?),
                "syntax" => syntax = Some(map.next_value()?),
                "value" => value = Some(map.next_value()?),
                "required" => required = map.next_value()?,
                "repeatable" => repeatable = map.next_value()?,
                "requires" => requires = map.next_value()?,
                "conflicts" => conflicts = map.next_value()?,
                "examples" => examples = map.next_value()?,
                "versionConstraints" => version_constraints = map.next_value()?,
                "order" => order = map.next_value::<Option<i64>>()?,
                _ => {
                    unknown.push(key);
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let id = id.ok_or_else(|| A::Error::missing_field("id"))?;
        let kind = kind.ok_or_else(|| A::Error::missing_field("kind"))?;
        let label = label.ok_or_else(|| A::Error::missing_field("label"))?;

        // Closed custom contract, stage 1: every key outside the allowlist
        // and outside the semantic carve-out (value/aliases/order) is
        // rejected here. Any future ParameterSpec field not classified in
        // either list is rejected at deserialization by default.
        if kind == ParameterKind::Custom {
            if let Some(bad) = unknown.iter().find(|k| {
                !CUSTOM_ALLOWED_FIELDS.contains(&k.as_str())
                    && !CUSTOM_SEMANTIC_FIELDS.contains(&k.as_str())
            }) {
                return Err(A::Error::custom(format!(
                    "field `{bad}` is not allowed for kind \"custom\""
                )));
            }
        }

        Ok(ParameterSpec {
            id,
            kind,
            label,
            token,
            aliases,
            description,
            syntax,
            value,
            required,
            repeatable,
            requires,
            conflicts,
            examples,
            version_constraints,
            order,
        })
    }
}

/// Deserializer for `CommandSpec.token`: the field must be present, and may
/// be a string or explicit `null` (direct/executable-level mode). A missing
/// field is an error.
fn de_required_nullable_token<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandSpec {
    pub id: String,
    /// Literal argv token for this (sub)command. `null` marks the
    /// direct/executable-level mode and is allowed only on top-level
    /// entries (enforced by validation.rs). Always serialized.
    #[serde(deserialize_with = "de_required_nullable_token")]
    pub token: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub syntax_examples: Vec<String>,
    #[serde(default)]
    pub version_constraints: Vec<String>,
    #[serde(default)]
    pub parameters: Vec<ParameterSpec>,
    #[serde(default)]
    pub subcommands: Vec<CommandSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryMatch {
    pub path: String,
    pub file_name: String,
    pub extension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiscoveryStatus {
    Found,
    NotFound,
    Ambiguous,
    InvalidName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryResult {
    pub query: String,
    pub status: DiscoveryStatus,
    pub matches: Vec<DiscoveryMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl RegistryError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            field: None,
        }
    }

    pub fn with_field(code: &str, message: impl Into<String>, field: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            field: Some(field.to_string()),
        }
    }
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RegistryError {}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        RegistryError::new("io", format!("filesystem operation failed: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn param_json(extra: &str) -> String {
        format!(
            r#"{{"id":"p","kind":"custom","label":"P","token":null,"syntax":"s"{extra}}}"#
        )
    }

    #[test]
    fn command_token_missing_is_rejected() {
        let err = serde_json::from_str::<CommandSpec>(
            r#"{"id":"c","name":"C","parameters":[],"subcommands":[]}"#,
        )
        .expect_err("missing token");
        assert!(err.to_string().contains("missing field `token`"), "{err}");
    }

    #[test]
    fn command_token_null_and_string_are_accepted() {
        let direct: CommandSpec = serde_json::from_str(
            r#"{"id":"root","token":null,"name":"Direct","parameters":[],"subcommands":[]}"#,
        )
        .unwrap();
        assert!(direct.token.is_none());
        let sub: CommandSpec = serde_json::from_str(
            r#"{"id":"s","token":"scan","name":"Scan","parameters":[],"subcommands":[]}"#,
        )
        .unwrap();
        assert_eq!(sub.token.as_deref(), Some("scan"));
    }

    #[test]
    fn command_token_serialized_explicitly() {
        let mut cmd: CommandSpec = serde_json::from_str(
            r#"{"id":"root","token":null,"name":"Direct"}"#,
        )
        .unwrap();
        let value = serde_json::to_value(&cmd).unwrap();
        assert_eq!(value.get("token"), Some(&serde_json::Value::Null));
        cmd.token = Some("scan".to_string());
        let value = serde_json::to_value(&cmd).unwrap();
        assert_eq!(value.get("token").and_then(|v| v.as_str()), Some("scan"));
    }

    #[test]
    fn parameter_token_is_optional_and_nullable() {
        // ParameterSpec.token is optional (absent for positionals/customs);
        // kinds that require a token are enforced by semantic validation.
        // Only CommandSpec.token is required-nullable at deserialization.
        let missing: ParameterSpec =
            serde_json::from_str(r#"{"id":"p","kind":"custom","label":"P"}"#).unwrap();
        assert!(missing.token.is_none());
        let nul: ParameterSpec =
            serde_json::from_str(&param_json("")).unwrap();
        assert!(nul.token.is_none());
        let text: ParameterSpec = serde_json::from_str(
            r#"{"id":"p","kind":"flag","label":"P","token":"-v"}"#,
        )
        .unwrap();
        assert_eq!(text.token.as_deref(), Some("-v"));
    }

    #[test]
    fn duplicate_keys_are_rejected_for_custom_parameters() {
        let err = serde_json::from_str::<ParameterSpec>(&param_json(r#","label":"Other""#))
            .expect_err("duplicate label");
        assert!(err.to_string().contains("duplicate field"), "{err}");
    }

    #[test]
    fn duplicate_keys_are_rejected_for_non_custom_parameters() {
        let err = serde_json::from_str::<ParameterSpec>(
            r#"{"id":"p","kind":"flag","label":"P","token":"-a","token":"-b"}"#,
        )
        .expect_err("duplicate token");
        assert!(err.to_string().contains("duplicate field"), "{err}");
    }

    #[test]
    fn duplicate_keys_are_rejected_in_nested_value_spec() {
        let err = serde_json::from_str::<ParameterSpec>(
            r#"{"id":"p","kind":"option","label":"P","token":"-t","value":{"valueType":"string","valueType":"integer"}}"#,
        )
        .expect_err("duplicate valueType");
        assert!(err.to_string().contains("duplicate field"), "{err}");
    }

    #[test]
    fn duplicate_keys_are_rejected_in_command_spec() {
        let err = serde_json::from_str::<CommandSpec>(
            r#"{"id":"c","token":"a","token":"b","name":"C"}"#,
        )
        .expect_err("duplicate token");
        assert!(err.to_string().contains("duplicate field"), "{err}");
    }

    #[test]
    fn unknown_field_on_custom_parameter_is_rejected_at_deserialization() {
        let err = serde_json::from_str::<ParameterSpec>(&param_json(r#","mystery":1"#))
            .expect_err("unknown key");
        assert!(
            err.to_string().contains("field `mystery` is not allowed for kind \"custom\""),
            "{err}"
        );
    }

    #[test]
    fn unknown_field_on_non_custom_parameter_is_tolerated() {
        let param: ParameterSpec = serde_json::from_str(
            r#"{"id":"p","kind":"flag","label":"P","token":"-v","futureField":true}"#,
        )
        .unwrap();
        assert_eq!(param.id, "p");
        assert_eq!(param.kind, ParameterKind::Flag);
        assert_eq!(param.token.as_deref(), Some("-v"));
    }

    #[test]
    fn known_forbidden_fields_deserialize_for_semantic_rejection() {
        // value/aliases/order on custom parameters must reach semantic
        // validation (field-path errors) - therefore they must NOT be
        // rejected during deserialization.
        let param: ParameterSpec = serde_json::from_str(&param_json(
            r#","value":{"valueType":"string"},"aliases":["-x"],"order":2"#,
        ))
        .unwrap();
        assert!(param.value.is_some());
        assert_eq!(param.aliases, Some(vec!["-x".to_string()]));
        assert_eq!(param.order, Some(2));
    }

    #[test]
    fn none_optionals_are_omitted_not_null() {
        let profile = minimal_profile();
        let value = serde_json::to_value(&profile).unwrap();
        assert!(value.get("commands").is_none());
        let verification = value.get("verification").unwrap();
        assert!(verification.get("toolVersion").is_none());
    }

    #[test]
    fn legacy_v10_profile_loads_and_omits_v11_fields() {
        const LEGACY: &str = r#"{
            "schemaVersion": "1.0",
            "id": "legacy-tool",
            "name": "Legacy",
            "category": "Scanning",
            "tags": [],
            "integrationType": "cli",
            "executable": { "name": "legacy.exe", "path": "C:\\Tools\\legacy.exe" },
            "discovery": { "source": "manual", "discoveredAt": "2026-01-01T00:00:00Z" },
            "verification": { "status": "verified", "source": "manual-import" },
            "platforms": ["windows"],
            "createdAt": "2026-01-01T00:00:00Z",
            "updatedAt": "2026-01-01T00:00:00Z"
        }"#;
        let profile: ToolProfile = serde_json::from_str(LEGACY).unwrap();
        assert!(profile.commands.is_none());
        assert!(profile.verification.tool_version.is_none());
        assert_eq!(profile.verification.status, VerificationStatus::Verified);
        assert_eq!(profile.id, "legacy-tool");
        assert_eq!(profile.schema_version, "1.0");
        // structural re-serialization omits the v1.1 fields entirely
        let value = serde_json::to_value(&profile).unwrap();
        assert!(value.get("commands").is_none());
        assert!(
            value
                .get("verification")
                .unwrap()
                .get("toolVersion")
                .is_none()
        );
    }

    #[test]
    fn nested_commands_roundtrip() {
        let json = r#"{
            "id": "root", "token": null, "name": "Direct",
            "parameters": [
                {"id":"target","kind":"positional","label":"Target","token":null,"required":true,"order":1,
                 "value":{"valueType":"string","multiple":true}}
            ],
            "subcommands": [
                {"id":"tool","token":"tool","name":"Tool",
                 "subcommands":[{"id":"install","token":"install","name":"Install","parameters":[]}]}
            ]
        }"#;
        let cmd: CommandSpec = serde_json::from_str(json).unwrap();
        assert_eq!(cmd.subcommands[0].subcommands[0].id, "install");
        let out = serde_json::to_string(&cmd).unwrap();
        let back: CommandSpec = serde_json::from_str(&out).unwrap();
        assert_eq!(cmd, back);
    }

    fn schema_json() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../schemas/tool-profile.schema.json")).unwrap()
    }

    #[test]
    fn schema_declares_draft_2020_12() {
        let schema = schema_json();
        assert_eq!(
            schema["$schema"].as_str(),
            Some("https://json-schema.org/draft/2020-12/schema")
        );
    }

    #[test]
    fn schema_expresses_custom_closed_contract() {
        // Drift guard between the Rust allowlist and the schema's
        // conditional (then) branch for kind=custom.
        let schema = schema_json();
        let param = &schema["$defs"]["parameterSpec"];
        let all_of = param["allOf"].as_array().unwrap();
        let custom = all_of
            .iter()
            .find(|c| c["if"]["properties"]["kind"]["const"] == serde_json::json!("custom"))
            .expect("custom conditional present");
        let then = &custom["then"];
        assert_eq!(then["additionalProperties"], serde_json::json!(false));
        let props = then["properties"].as_object().unwrap();
        for field in CUSTOM_ALLOWED_FIELDS {
            assert!(props.contains_key(field), "missing allowed field {field}");
        }
        for field in CUSTOM_SEMANTIC_FIELDS {
            assert_eq!(props[field], serde_json::json!(false), "{field} must be false");
        }
        assert!(then["anyOf"].as_array().is_some(), "syntax-or-description rule");
    }

    #[test]
    fn schema_limits_top_level_null_token() {
        let schema = schema_json();
        let commands = &schema["properties"]["commands"];
        assert_eq!(commands["minContains"], serde_json::json!(0));
        assert_eq!(commands["maxContains"], serde_json::json!(1));
    }

    fn minimal_profile() -> ToolProfile {
        ToolProfile {
            schema_version: SCHEMA_VERSION.to_string(),
            id: "tool".to_string(),
            name: "Tool".to_string(),
            description: None,
            category: Category::Other,
            tags: vec![],
            integration_type: IntegrationType::Cli,
            executable: ExecutableInfo {
                name: "tool.exe".to_string(),
                path: "C:\\Tools\\tool.exe".to_string(),
            },
            version: None,
            discovery: DiscoveryInfo {
                source: DiscoverySource::Manual,
                discovered_at: "2026-01-01T00:00:00Z".to_string(),
            },
            verification: VerificationInfo {
                status: VerificationStatus::Unverified,
                source: "manual-import".to_string(),
                tool_version: None,
            },
            platforms: vec![Platform::Windows],
            commands: None,
            documentation_url: None,
            author: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }
}
