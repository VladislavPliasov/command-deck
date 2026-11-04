use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: &str = "1.0";

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
