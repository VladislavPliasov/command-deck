# Command Deck — Security Model

**Document Version:** 1.0  
**Status:** Initial Security Requirements

---

## 1. Purpose

This document defines the security principles and controls governing Command Deck itself.

Command Deck is a cybersecurity workstation that may launch powerful tools.

The application must therefore protect the host system, preserve user control, and provide clear authorization boundaries for security operations.

## 2. Core Security Principles

- Least privilege.
- Explicit user control.
- Scope validation.
- Transparent execution.
- Process isolation where practical.
- Permission-based extensions.
- No hidden execution.
- No silent privilege escalation.
- Auditability.
- Secure data handling.

## 3. Application Privileges

Command Deck must not run as root by default.

The normal application process should operate under the current user account.

Privileged operations must use a separate, controlled authorization mechanism.

The application must request elevated permissions only when an operation genuinely requires them.

## 4. Execution Security

The Execution Engine must use structured process execution.

The preferred implementation is a process API such as Rust `std::process::Command`.

Arguments must be passed as separate values.

The application must not construct shell commands by concatenating untrusted user input.

Shell execution should be limited to cases where a real shell is explicitly required, such as the integrated terminal.

## 5. Command Preview and Confirmation

Before executing a generated command, the user must be able to inspect:

- Executable.
- Complete arguments.
- Working directory.
- Relevant environment settings.
- Target.
- Project context.
- Scope validation result.
- Privilege requirements.

The user must explicitly confirm execution.

The application must not hide important command arguments.

## 6. Scope Manager

The Scope Manager defines which targets and operations are authorized within a project.

Possible scope entries:

- IP addresses.
- CIDR ranges.
- Domains.
- Explicit exclusions.
- Local laboratory environments.
- Permitted operation categories.
- Time restrictions.
- Authorization notes.

Scope entries must be editable and reviewable.

## 7. Scope Validation

Applicable active operations must be checked against the project scope.

The validation process should consider:

- Requested target.
- Resolved target where relevant.
- Exclusions.
- Operation category.
- Scope validity period.
- Required authorization state.

A target discovered during reconnaissance must not automatically become an authorized target.

Ambiguous or incomplete scope information should result in a blocked or manually reviewed operation rather than an assumed authorization.

## 8. Dangerous Operations

Operations with significant impact must receive additional confirmation.

Examples may include:

- Destructive testing.
- Service disruption.
- Password attacks.
- Exploitation.
- Privilege escalation.
- Post-exploitation.
- Actions affecting production systems.

The confirmation should explain the operation and its potential impact.

The application should support configurable restrictions for high-risk operations.

## 9. Process Management

Command Deck must track processes it launches.

It should maintain:

- Process identifier.
- Executable.
- Arguments.
- Start time.
- Project association.
- Execution status.
- Exit code.
- Logs.

The application should provide controlled termination.

It must not silently kill unrelated system processes.

## 10. Resource Controls

The Execution Engine should support:

- Execution timeouts.
- Configurable concurrency.
- Resource monitoring.
- Output size limits.
- Cancellation.
- Queue management.

The application must protect itself from uncontrolled resource consumption.

## 11. Terminal Security

The integrated terminal is a full shell environment.

It must clearly communicate that terminal commands are executed with the current user's privileges.

The terminal must not pretend that arbitrary shell commands are covered by the Command Builder's scope validation.

For this reason, project scope controls should be clearly distinguished from direct shell access.

## 12. Plugin Security

Plugins must declare their permissions.

Potential permissions:

- Execute external processes.
- Read selected project data.
- Write selected project data.
- Access network resources.
- Register tools.
- Register parsers.
- Register workflows.
- Add interface components.
- Access selected files.

The user must be able to inspect permissions.

Plugins must not receive unrestricted access by default.

The architecture should support isolation or sandboxing where practical.

A plugin's declared permissions must be enforced by the host, not merely displayed in its metadata.

## 13. Profile Security

Tool profiles may influence command construction.

Therefore:

- Profiles must be schema-validated.
- Invalid profiles must not be activated.
- External profiles must be treated as untrusted input.
- Executable paths must be checked.
- Unexpected command behavior must be visible.
- Profile changes must be reviewable.

Importing a profile must not automatically execute its commands.

## 14. Data Security

Project data may contain sensitive security information.

The application should:

- Store data locally by default.
- Restrict access through operating-system permissions.
- Avoid unnecessary telemetry.
- Avoid transmitting project data without explicit user action.
- Protect secrets where they are stored.
- Avoid writing credentials into ordinary logs.
- Allow users to export and delete their data.

Encryption at rest should be evaluated for sensitive data and secrets.

## 15. Network Security

External API integrations must use secure transport where available.

The application should:

- Identify external data sources.
- Make network operations visible.
- Avoid unnecessary background requests.
- Allow users to disable optional online integrations.
- Validate downloaded profiles and plugin metadata.
- Avoid treating external content as trusted instructions.

## 16. Auditability

The application should maintain an execution history.

Relevant information:

- User-initiated action.
- Tool.
- Target.
- Project.
- Arguments.
- Scope validation.
- Start and completion times.
- Exit status.
- Relevant logs.

Sensitive values should be redacted where appropriate.

## 17. Security Failure Behavior

When a required security control fails, the application must fail safely.

Examples:

- Invalid scope: block the applicable operation.
- Missing required permission: request authorization or stop.
- Invalid profile: reject activation.
- Execution failure: report the error.
- Plugin permission violation: deny the operation.
- Unknown command grammar: require manual configuration and confirmation.

## 18. Security Acceptance Criteria

The initial security foundation is acceptable when:

- The application does not require root for normal operation.
- Commands are executed through structured argument handling.
- Users can inspect commands before execution.
- Applicable operations are checked against project scope.
- Processes are tracked.
- Privileged operations require controlled authorization.
- Plugins cannot silently obtain unrestricted permissions.
- Original execution logs are preserved.
- Sensitive project data is not transmitted without authorization.

---

**End of Security Model**