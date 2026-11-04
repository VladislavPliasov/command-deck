// Type mirrors for the Rust DTOs in src-tauri/src/registry/model.rs.
// The Rust serde model is the source of truth; keep names in sync.

export type IntegrationType = "cli" | "gui";

export type Category =
  | "Reconnaissance"
  | "Scanning"
  | "Web Application Security"
  | "Exploitation"
  | "Password Security Testing"
  | "Wireless Security"
  | "Network Security"
  | "Privilege Escalation"
  | "Post-Exploitation"
  | "Active Directory and Windows Security"
  | "Reverse Engineering"
  | "Malware Analysis"
  | "Digital Forensics"
  | "Social Engineering"
  | "Defensive Security"
  | "other";

// Exact values from PRODUCT_SPECIFICATION.md section 5, plus "other".
// Must stay identical to Category::ALL in src-tauri/src/registry/model.rs.
export const CATEGORY_VALUES: Category[] = [
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
];

export type Platform = "windows" | "linux" | "macos";

export type VersionSource = "manual" | "file-version";

export interface VersionInfo {
  value: string;
  source: VersionSource;
}

export type DiscoverySource = "manual" | "path-scan" | "import";

export interface DiscoveryInfo {
  source: DiscoverySource;
  discoveredAt: string;
}

export type VerificationStatus =
  | "unverified"
  | "partially-verified"
  | "verified"
  | "manually-configured"
  | "incompatible";

export interface VerificationInfo {
  status: VerificationStatus;
  source: string;
}

export interface ExecutableInfo {
  name: string;
  path: string;
}

export interface AuthorInfo {
  name: string;
  url?: string;
}

export interface ToolProfile {
  schemaVersion: string;
  id: string;
  name: string;
  description?: string;
  category: Category;
  tags: string[];
  integrationType: IntegrationType;
  executable: ExecutableInfo;
  version?: VersionInfo;
  discovery: DiscoveryInfo;
  verification: VerificationInfo;
  platforms: Platform[];
  documentationUrl?: string;
  author?: AuthorInfo;
  createdAt: string;
  updatedAt: string;
}

export type InstallStatus = "installed" | "not-found" | "unknown";

// Rust serializes ToolProfileView flattened (profile fields + installStatus).
export interface ToolProfileView extends ToolProfile {
  installStatus: InstallStatus;
}

export interface ToolSummaryView {
  id: string;
  name: string;
  category: Category;
  integrationType: IntegrationType;
  executableName: string;
  installStatus: InstallStatus;
  verificationStatus: VerificationStatus;
}

export interface ToolListResult {
  tools: ToolSummaryView[];
  warnings: string[];
}

export interface RegisterToolRequest {
  id?: string;
  name?: string;
  executablePath: string;
  integrationType?: IntegrationType;
  category?: Category;
  tags?: string[];
  description?: string;
  documentationUrl?: string;
  author?: AuthorInfo;
}

export interface UpdateToolRequest {
  name?: string;
  description?: string;
  category?: Category;
  tags?: string[];
  integrationType?: IntegrationType;
  documentationUrl?: string;
  author?: AuthorInfo;
  executablePath?: string;
}

export interface RegistryErrorShape {
  code: string;
  message: string;
  field?: string;
}

export interface DiscoveryMatch {
  path: string;
  fileName: string;
  extension: string;
  fileVersion?: string;
}

export type DiscoveryStatus = "found" | "not-found" | "ambiguous" | "invalid-name";

export interface DiscoveryResult {
  query: string;
  status: DiscoveryStatus;
  matches: DiscoveryMatch[];
}
