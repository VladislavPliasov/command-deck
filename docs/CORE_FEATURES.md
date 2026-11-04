# Command Deck — Core Features

**Document Version:** 1.0

---

## 1. Purpose

This document defines the functional requirements of Command Deck.

It describes what each major subsystem must do, independently of the final implementation technology.

## 2. Tool Registry

The Tool Registry is the central database of integrated tools.

Each tool profile should contain:

- Unique ID.
- Name.
- Description.
- Category.
- Executable path.
- Version.
- Installation status.
- Integration type.
- Supported commands.
- Supported parameters.
- Documentation references.
- Parser references.
- Compatibility information.

The Registry must support:

- Automatic discovery.
- Manual import.
- Community profiles.
- Custom user profiles.
- Profile validation.
- Profile editing.
- Profile versioning.

## 3. Command Builder

The Command Builder converts structured user configuration into an executable command.

It must support:

- Subcommands.
- Boolean flags.
- Options with values.
- Positional arguments.
- Repeated options.
- Enumerated values.
- Paths.
- Required parameters.
- Conditional parameters.
- Conflicting parameters.
- Version constraints.
- Ordering constraints.

The command must be represented internally as an executable and a structured argument list.

The system must not rely on concatenating user input into a shell command string.

The final command preview must be readable by the user.

## 4. Command Validation

Validation must happen before execution.

Possible validation checks:

- Required parameters are present.
- Values have valid types.
- Selected choices are supported.
- Paths are valid where appropriate.
- Conflicting options are not selected.
- Required dependencies are satisfied.
- Tool versions support selected options.
- The target is within the applicable authorized scope.
- Required permissions are available.

Validation errors must identify the affected parameter and explain the problem.

## 5. Execution Engine

The Execution Engine manages external processes.

Core capabilities:

- Start processes.
- Capture stdout.
- Capture stderr.
- Support interactive PTY sessions.
- Track process IDs.
- Track execution status.
- Track exit codes.
- Stop processes.
- Enforce timeouts.
- Record execution metadata.
- Store logs.
- Monitor resource consumption.
- Handle failures.

The engine must support multiple concurrent tasks within configurable resource limits.

The UI must remain responsive while tasks execute.

## 6. Process Manager

The Process Manager provides a central view of processes launched by Command Deck.

It should support:

- Active process listing.
- Process details.
- Stop and interrupt operations.
- Resource monitoring.
- Execution history.
- Log inspection.
- Failure reporting.

The application must not silently terminate processes merely because their UI tab is closed.

## 7. Tool Adapters

Tool Adapters provide specialized integration for tools that cannot be represented by a generic command profile.

An adapter may define:

- Startup behavior.
- Argument construction.
- Environment variables.
- Interactive behavior.
- Output handling.
- Parser selection.
- Process termination behavior.
- Special configuration requirements.

Adapters should be used only when generic integration is insufficient.

## 8. Result Parsing

Each supported tool may have a dedicated parser.

The parser should convert recognized output into common data entities.

Examples:

- Host.
- IP address.
- Domain.
- Port.
- Service.
- Software version.
- CVE.
- Vulnerability.
- File.
- Evidence.

Parsing must not destroy original output.

Every structured result must retain a reference to its source execution.

Parser failures must not invalidate the original execution result.

## 9. Results Center

The Results Center stores and organizes:

- Raw output.
- Structured results.
- Execution metadata.
- Findings.
- Evidence.
- Relationships between findings.

Users should be able to:

- Search results.
- Filter results.
- Compare results.
- Navigate to source executions.
- Export selected data.
- Link findings to projects.

## 10. Project Manager

The Project Manager handles local project organization.

A project contains:

- Project metadata.
- Scope.
- Targets.
- Tasks.
- Results.
- Evidence.
- Notes.
- Workflows.
- Reports.

Project data must remain consistent even if the application closes unexpectedly.

## 11. Scope Manager

The Scope Manager stores authorization boundaries.

Possible scope elements:

- IP addresses.
- CIDR ranges.
- Domains.
- Local laboratory environments.
- Explicit exclusions.
- Permitted operation categories.
- Time restrictions.
- Notes and authorization references.

Scope validation must be performed before applicable active operations.

The system must not assume that discovering a target automatically grants authorization to test it.

## 12. Workflow Engine

The Workflow Engine connects tools and operations.

It should support:

- Sequential execution.
- Conditional stages.
- Manual approval stages.
- Data passing between stages.
- Failure handling.
- Pause and resume where supported.
- Workflow templates.
- Execution history.

Each stage must have defined inputs and outputs.

A workflow must not bypass scope checks or execution confirmations.

## 13. Rule Engine

The Rule Engine provides deterministic analysis.

A rule may contain:

- Trigger conditions.
- Input requirements.
- Matching logic.
- Confidence or certainty classification.
- Result.
- Explanation.
- Suggested next steps.

Example:

If a scan identifies a service and a version, the system may search available vulnerability data for matching software and version information.

A version match is not proof that a vulnerability is exploitable.

The Rule Engine must communicate uncertainty.

## 14. Exploit Intelligence

This subsystem provides vulnerability and exploit research.

It should support:

- CVE search.
- Local vulnerability databases.
- SearchSploit integration.
- External references.
- GitHub repository lookup.
- Metasploit references.
- Compatibility information.
- Dependency information.
- Source attribution.
- Manual freshness checks.

External sources should be optional.

## 15. Plugin Manager

The Plugin Manager handles third-party extensions.

It must support:

- Plugin discovery.
- Installation.
- Updates.
- Removal.
- Enable and disable operations.
- Permission declarations.
- Validation.
- Compatibility checks.

The core application must not automatically trust arbitrary plugin code.

## 16. Session Manager

The Session Manager preserves application state.

It should store:

- Open tabs.
- Pinned tabs.
- Tab groups.
- Active tab.
- Open workspaces.
- Relevant project context.
- UI layout.
- Restoration metadata.

Session restoration must not automatically restart potentially dangerous processes.

## 17. Search Engine

Global Search should index local application data.

Searchable entities:

- Tools.
- Commands.
- Options.
- CVEs.
- Exploit references.
- Projects.
- Results.
- Documentation.
- Workflows.

Search must work offline for indexed local data.

## 18. System Monitor

The System Monitor provides information about:

- CPU usage.
- Memory usage.
- Disk usage where relevant.
- Active Command Deck processes.
- Execution resource consumption.
- System availability.

The monitor must avoid excessive background resource consumption.

## 19. Export and Reporting

The application should support exporting selected results and project information.

Potential formats:

- JSON.
- CSV.
- Markdown.
- HTML.

Additional formats may be implemented later.

Reports must preserve source references and distinguish observed data from inferred conclusions.

## 20. Functional Priorities

The first implementation should prioritize:

1. Application shell.
2. Tab management.
3. Execution Engine.
4. Terminal.
5. Tool Registry.
6. Command Builder.
7. Process Manager.
8. Results Center.
9. Projects.
10. Scope Manager.

Advanced intelligence and marketplace features should follow after the execution foundation is reliable.

---

**End of Core Features**