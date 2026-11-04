# Command Deck — Tool Integration Specification

**Document Version:** 1.0  
**Status:** Initial Specification

---

## 1. Purpose

This document defines how Command Deck discovers, imports, configures, executes, and manages external cybersecurity tools.

The integration system must be extensible and must not depend on a fixed catalog of supported applications.

## 2. Supported Integration Types

Command Deck should recognize several integration types.

### 2.1 CLI Tools

Examples:

- Nmap.
- Gobuster.
- Nikto.
- Hydra.
- John the Ripper.
- Hashcat.
- SQLmap.
- SearchSploit.

CLI tools are the primary target for generic integration.

### 2.2 GUI Applications

Examples:

- Wireshark.
- GParted.
- Autopsy.
- Burp Suite.

GUI applications must not be forced into a command-line parameter interface.

Depending on the application, integration may consist of:

- Launching the native application.
- Opening a dedicated workspace.
- Managing its execution lifecycle.
- Connecting supported import/export workflows.
- Providing a specialized adapter.

### 2.3 Scripts

Supported script ecosystems may include:

- Python.
- Bash.
- Go.
- Rust.
- Other executable formats available on the host system.

Scripts should be integrated through their executable entry point and a corresponding profile or adapter.

### 2.4 Custom Applications

Users must be able to import compatible applications manually.

## 3. Discovery Methods

Three integration methods are required.

### 3.1 Automatic Discovery

The application searches configured executable locations and known system paths.

Discovery should identify:

- Executable name.
- Executable path.
- Version, if available.
- Possible tool category.
- Available documentation.
- Possible integration type.

Discovery must not execute arbitrary tools with unknown arguments.

### 3.2 Manual Import

Users can specify:

- Executable path.
- Tool name.
- Category.
- Version.
- Description.
- Integration method.
- Optional profile.
- Optional documentation references.

The application should validate that the selected executable exists and can be accessed.

### 3.3 Community Profiles

Users can import profiles created by other users or maintainers.

Profiles must be validated before activation.

Community profiles must not be treated as automatically trustworthy.

## 4. Automatic Profile Generation

Command Deck should attempt to generate a tool profile using available information.

Possible information sources:

1. Existing official metadata.
2. Tool-specific documentation.
3. `--help`.
4. `-h`.
5. Manual pages.
6. Version commands.
7. Known community profiles.
8. Tool-specific metadata.
9. Specialized adapters.

Not all tools support the same help syntax.

The system must never assume that every application supports `-h`, `--help`, or `-help`.

### 4.1 Discovery Process

Suggested process:

1. Locate the executable.
2. Identify the file type where possible.
3. Determine the application version.
4. Identify known tool signatures.
5. Search for existing profiles.
6. Retrieve available documentation.
7. Attempt safe metadata extraction.
8. Parse discovered commands and options.
9. Build a candidate profile.
10. Validate the profile.
11. Present the candidate to the user.
12. Allow manual corrections.
13. Save the profile.

### 4.2 Confidence and Verification

Generated profiles must contain a verification status.

Suggested statuses:

- Unverified.
- Partially Verified.
- Verified.
- Manually Configured.
- Incompatible.

Automatically extracted information must not be represented as verified without testing.

The system should indicate which parameters were extracted and which were manually confirmed.

## 5. Tool Profile Structure

A tool profile should describe the executable and its command grammar.

Suggested conceptual structure:

```json
{
  "schemaVersion": "1.0",
  "toolId": "example-tool",
  "name": "Example Tool",
  "description": "Tool description",
  "category": "reconnaissance",
  "integrationType": "cli",
  "executable": {
    "name": "example",
    "path": null,
    "versionCommand": ["--version"]
  },
  "verification": {
    "status": "unverified",
    "source": "automatic-discovery"
  },
  "commands": [],
  "documentation": [],
  "parsers": [],
  "compatibility": {
    "platforms": ["linux"]
  }
}
```

This is an illustrative format, not a finalized schema.

The actual schema must be versioned and validated.

## 6. Parameter Metadata

Each option should describe its behavior.

Suggested fields:

```json
{
  "id": "target",
  "flag": "-t",
  "label": "Target",
  "description": "Target to process",
  "syntax": "-t <target>",
  "valueType": "string",
  "requiredValue": true,
  "choices": [],
  "default": null,
  "repeatable": false,
  "position": "option",
  "conflicts": [],
  "requires": [],
  "examples": [],
  "versionConstraints": []
}
```

Possible value types:

- Boolean.
- String.
- Integer.
- Decimal.
- Enumeration.
- File path.
- Directory path.
- IP address.
- CIDR.
- Domain.
- Port.
- List.
- Custom.

The profile format must also represent positional arguments, nested subcommands, mutually exclusive groups, conditional requirements, and order-sensitive syntax.

## 7. Command Grammar

The integration system must not assume that every CLI uses the same syntax.

It must account for:

- Different flag prefixes.
- Long and short options.
- Positional arguments.
- Subcommands.
- Nested subcommands.
- Repeated parameters.
- Options requiring multiple values.
- Options that consume a variable number of values.
- Order-dependent arguments.
- Special quoting requirements.
- Environment variables.
- Configuration files.
- Interactive execution.

The profile must define how arguments are constructed.

## 8. Command Builder Integration

A supported CLI profile must provide sufficient information for the Command Builder.

The Builder should:

1. Display available commands.
2. Display available options.
3. Explain selected parameters.
4. Collect values.
5. Validate the configuration.
6. Generate the argument list.
7. Display the final command preview.
8. Request user confirmation.
9. Pass the structured execution request to the Execution Engine.

The Builder must preserve manual editing as an alternative.

## 9. Execution Integration

A tool profile or adapter should define:

- Executable.
- Arguments.
- Working directory.
- Environment variables.
- Execution mode.
- PTY requirements.
- Timeout behavior.
- Output handling.
- Termination behavior.

The Execution Engine must retain control over process lifecycle.

## 10. Output Parsing

Tool integrations may provide parser definitions.

Parsers should:

- Recognize known output formats.
- Extract structured entities.
- Preserve original output.
- Report parsing errors.
- Associate findings with the source execution.

A parser must not silently invent missing information.

## 11. Version Compatibility

Profiles must support version constraints.

The application should distinguish between:

- Supported version.
- Partially supported version.
- Unknown version.
- Incompatible version.

Options unavailable in the installed version must not be presented as verified supported options.

## 12. Manual Integration Mode

When automatic discovery cannot understand a tool, the user must still be able to execute it.

Manual integration should allow:

- Executable selection.
- Free-form argument configuration.
- Working directory selection.
- Environment configuration.
- Output capture.
- Execution control.

This mode is essential for universal compatibility.

## 13. Tool Catalog

The catalog should organize tools by security domain.

Users must be able to:

- Search.
- Filter.
- Browse categories.
- View installation status.
- Import tools.
- Update profiles.
- Remove custom integrations.
- Mark favorites.

## 14. Acceptance Criteria

The integration subsystem is acceptable when:

- It discovers known installed executables.
- It supports manual import.
- It loads validated profiles.
- It distinguishes verified and unverified metadata.
- It can construct correct argument lists for supported tools.
- It preserves manual execution for unsupported tools.
- It does not require every application to support a common help command.
- It does not force GUI applications into CLI integration.

---

**End of Tool Integration Specification**