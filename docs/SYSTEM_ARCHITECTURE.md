# Command Deck — System Architecture

**Document Version:** 1.0  
**Status:** Proposed Architecture — Pending Technical Audit

---

## 1. Purpose

This document defines the proposed technical architecture of Command Deck.

It establishes subsystem boundaries, responsibilities, and communication principles.

The technology choices in this document are proposals, not final commitments.

A technical feasibility audit must be completed before implementation.

## 2. Architecture Goals

The architecture must support:

- Local-first operation.
- Linux-first development.
- Reliable process execution.
- Multiple concurrent tasks.
- Interactive terminals.
- Universal tool integration.
- Extensible profiles.
- Structured result processing.
- Project management.
- Offline operation.
- Plugin extensibility.
- Future platform expansion.

## 3. Proposed Technology Stack

Candidate technologies:

| Layer | Proposed Technology |
|---|---|
| Desktop framework | Tauri 2 |
| Frontend | Vue 3 |
| Language | TypeScript |
| Backend | Rust |
| Database | SQLite |
| Tool profiles | JSON or YAML |
| Terminal | PTY integration |
| Build system | To be selected after audit |

These choices must be validated against Kali Linux, terminal requirements, process management, packaging, and plugin isolation.

## 4. High-Level Architecture

The application should be divided into the following logical subsystems:

1. Application Shell.
2. UI Layer.
3. Application State Manager.
4. Tool Registry.
5. Profile Manager.
6. Command Builder.
7. Validation Engine.
8. Execution Engine.
9. Terminal Manager.
10. Process Manager.
11. Tool Adapters.
12. Result Parsers.
13. Results Center.
14. Project Manager.
15. Scope Manager.
16. Workflow Engine.
17. Rule Engine.
18. Exploit Intelligence.
19. Plugin Manager.
20. Database Layer.
21. Session Manager.

## 5. Application Shell

Responsibilities:

- Application lifecycle.
- Window management.
- Global navigation.
- Tab management.
- Keyboard shortcuts.
- Session restoration.
- Application-wide events.

The shell should remain independent of individual tool implementations.

## 6. UI Layer

Responsibilities:

- Rendering workspaces.
- Displaying tool profiles.
- Command configuration.
- Displaying execution state.
- Showing results.
- User interaction.
- Notifications.

The UI must not directly execute arbitrary system commands.

Execution requests must pass through the backend execution subsystem.

## 7. Backend Layer

The backend should own privileged system integration and core application services.

Responsibilities:

- Process execution.
- Filesystem access.
- Database operations.
- PTY management.
- Validation.
- Process monitoring.
- Scope enforcement.
- Plugin permission enforcement.

The frontend should communicate through a defined and restricted command/event interface.

## 8. Tool Registry and Profile Manager

The Registry stores tool metadata.

The Profile Manager loads, validates, versions, and manages tool profiles.

Tool profiles should be data-driven.

Specialized behavior should be implemented through adapters rather than unrestricted profile scripting.

## 9. Command Builder and Validator

The Command Builder converts user selections into a structured execution request.

The Validator checks:

- Required arguments.
- Types.
- Constraints.
- Conflicts.
- Version compatibility.
- Scope requirements.
- Execution prerequisites.

The result should be a validated execution request, not an arbitrary shell string.

## 10. Execution Engine

The Execution Engine owns process lifecycle.

Responsibilities:

- Process creation.
- Argument handling.
- Working directory.
- Environment configuration.
- stdout and stderr capture.
- PTY sessions.
- Cancellation.
- Timeouts.
- Exit status.
- Resource monitoring.
- Execution events.

The engine must isolate process management from UI lifecycle.

## 11. Terminal Manager

The Terminal Manager provides interactive PTY sessions.

It must support:

- Shell creation.
- Input.
- Output.
- Resize events.
- Signal handling.
- Session lifecycle.
- Working directory state.

The terminal must be implemented as a real PTY-backed terminal, not a text area that imitates shell output.

## 12. Process Manager

The Process Manager maintains application-level execution records.

It should distinguish:

- Process state.
- Tool state.
- Tab state.
- Workflow state.

Closing a UI tab must not automatically imply process termination.

## 13. Result Processing Pipeline

Suggested flow:

Tool Execution → Raw Output Capture → Parser → Structured Results → Database → Results Center → Rule Engine

Original output must be retained.

Parser failure must not prevent storage of raw output.

Structured results must retain provenance.

## 14. Project and Scope Services

The Project Manager stores project data.

The Scope Manager validates applicable execution requests.

These services must be accessible to workflows and direct tool execution.

## 15. Workflow Architecture

A workflow consists of stages with defined:

- Inputs.
- Outputs.
- Configuration.
- Dependencies.
- Execution conditions.
- Approval requirements.

Workflow stages should use the same validation and execution infrastructure as direct tool execution.

A workflow must not have a separate mechanism that bypasses security controls.

## 16. Rule Engine

The Rule Engine evaluates structured data against deterministic rules.

It should be:

- Local-first.
- Explainable.
- Versioned.
- Testable.
- Independent of mandatory external AI services.

Rules should reference the data and conditions that triggered them.

## 17. Exploit Intelligence Architecture

Potential data sources:

- Local databases.
- SearchSploit.
- NVD.
- GitHub.
- External APIs.
- Metasploit references.

Each source should have a separate integration layer.

Source data must preserve attribution and timestamps where available.

## 18. Plugin Architecture

The Plugin Manager should separate:

- Plugin metadata.
- Plugin permissions.
- Plugin lifecycle.
- Plugin UI integration.
- Tool profile registration.
- Parser registration.
- Workflow registration.

The architecture must define a stable plugin API before opening the ecosystem to third-party developers.

Arbitrary plugin code should not execute inside the trusted core without an explicit and controlled permission model.

## 19. Database

SQLite is the proposed initial database.

Potential data groups:

- Tools.
- Tool profiles.
- Projects.
- Targets.
- Scope entries.
- Executions.
- Process metadata.
- Raw outputs.
- Structured results.
- Vulnerabilities.
- Evidence.
- Workflows.
- Workflow executions.
- Plugins.
- Sessions.
- Settings.

The database schema must be versioned and support migrations.

Large raw output should be stored using a strategy that avoids unnecessarily bloating frequently accessed records.

## 20. Suggested Repository Structure

The following is a starting proposal. It must be adjusted after the architecture audit.

```text
command-deck/
├── README.md
├── docs/
│   ├── PRODUCT_SPECIFICATION.md
│   ├── UI_UX_SPECIFICATION.md
│   ├── CORE_FEATURES.md
│   ├── TOOL_INTEGRATION.md
│   ├── SECURITY_MODEL.md
│   ├── SYSTEM_ARCHITECTURE.md
│   ├── DEVELOPMENT_ROADMAP.md
│   └── decisions/
├── src/
│   ├── app/
│   ├── components/
│   ├── layouts/
│   ├── views/
│   ├── stores/
│   ├── services/
│   └── types/
├── src-tauri/
│   └── src/
│       ├── commands/
│       ├── execution/
│       ├── terminal/
│       ├── process/
│       ├── validation/
│       ├── tools/
│       ├── parsers/
│       ├── projects/
│       ├── workflows/
│       ├── security/
│       ├── database/
│       └── main.rs
├── catalog/
├── schemas/
├── plugins/
├── tests/
└── scripts/
```

## 21. Communication Principles

Subsystems should communicate through explicit interfaces.

Avoid:

- Direct database access from unrelated UI components.
- Circular dependencies.
- Tool-specific logic inside the application shell.
- Unrestricted plugin access.
- Duplicated process management.
- Duplicated scope validation.
- Shell command construction through string concatenation.

## 22. Testing Strategy

The architecture must support:

- Unit tests.
- Integration tests.
- Profile validation tests.
- Command construction tests.
- Execution tests.
- Parser tests.
- Database migration tests.
- Workflow tests.
- Security tests.
- UI tests.

Tool execution tests should use controlled local fixtures or authorized laboratory targets.

## 23. Technical Audit Requirements

Before implementation, verify:

1. Tauri 2 compatibility with the target Kali environment.
2. PTY implementation options.
3. Process lifecycle management.
4. Concurrent process handling.
5. Frontend/backend event transport.
6. SQLite access and migration strategy.
7. Plugin isolation options.
8. Filesystem permissions.
9. Linux packaging.
10. Dependency licensing.
11. Resource consumption.
12. Future portability.

The audit must identify technical risks and validate the proposed stack.

## 24. Architecture Acceptance Criteria

The architecture is ready for implementation when:

- Core subsystem boundaries are defined.
- Process execution design is validated.
- PTY integration is validated.
- Data storage is designed.
- Tool profile schema is defined.
- Security enforcement points are identified.
- Plugin trust boundaries are defined.
- Build and packaging are tested on Kali Linux.

---

**End of System Architecture**