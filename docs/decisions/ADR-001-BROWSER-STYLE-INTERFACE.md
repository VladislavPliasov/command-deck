# ADR-001: Browser-Style Interface

**Status:** Accepted  
**Decision Date:** 2026-11-03  
**Decision Owner:** Command Deck Project  
**Version:** 1.0

---

## 1. Context

Command Deck requires a desktop interface capable of managing multiple tools, terminals, projects, results, and long-running tasks.

The interface must allow users to switch between different workspaces without opening numerous independent operating-system windows.

Several interface models are possible:

- Browser-style tabs.
- IDE-style navigation.
- Arc-style vertical tabs.
- Traditional multi-window desktop application.

The selected design must support a large number of independent workspaces while remaining familiar and easy to navigate.

## 2. Decision

Command Deck will use a browser-style interface inspired by Chrome and Tor Browser.

The browser interaction model is the primary reference.

It is not a requirement to embed a complete web browser engine.

## 3. Interface Structure

The application will use:

### Top Tab Bar

- Horizontal tabs.
- New-tab button.
- Close controls.
- Tab switching.
- Pinned tabs.
- Tab grouping.
- Reordering.
- Session restoration.

### Navigation Area

- Navigation controls.
- Universal search/address bar.
- Global search access.

### Left Sidebar

- Dashboard.
- Tools.
- Projects.
- Results.
- Terminal.
- Process Manager.
- Workflows.
- Exploit Intelligence.
- Plugins.
- Settings.

### Central Workspace

The active tab displays its corresponding workspace.

### Contextual Controls

Additional controls may appear in the active workspace or an optional contextual panel.

## 4. Tab Behavior

Every tab must have its own identity and state.

Tabs may represent:

- A tool instance.
- A terminal session.
- A project.
- A result view.
- A workflow.
- An application section.

Multiple instances of the same tool must be supported.

For example, two Nmap tabs may maintain independent configurations and execution references.

## 5. Process Lifecycle

Closing a tab must not automatically terminate its associated process.

The application must distinguish between:

- Closing a workspace.
- Detaching a workspace from an execution.
- Stopping an execution.

The user must explicitly choose to stop a running process.

## 6. Session Persistence

The application should preserve:

- Open tabs.
- Pinned tabs.
- Tab order.
- Tab groups.
- Active tab.
- Workspace state where practical.
- Project context.

Session restoration must not automatically restart previously running operations.

## 7. Rejected Alternatives

### IDE-Style Interface

Rejected as the primary interface model because Command Deck is not primarily a code editor or development environment.

### Arc-Style Vertical Tabs

Rejected as the primary navigation model.

Vertical navigation may be evaluated as an optional layout feature in the future, but it must not replace the browser-style default.

### Independent OS Windows for Every Tool

Rejected as the default because it would fragment the workspace and make process and project context harder to manage.

## 8. Consequences

Advantages:

- Familiar navigation.
- Independent workspaces.
- Multiple tool instances.
- Better session management.
- Centralized application experience.

Engineering requirements:

- Tab state management.
- Workspace lifecycle management.
- Process-to-tab associations.
- Session persistence.
- Restoration logic.
- Clear separation between UI lifecycle and process lifecycle.

## 9. Acceptance Criteria

The decision is implemented when:

- Tabs are horizontal.
- Users can create and close tabs.
- Users can switch between workspaces.
- Tabs can be pinned.
- Multiple instances of a tool can coexist.
- Sessions can be restored.
- Closing a tab does not silently stop its process.
- The left sidebar provides access to major application sections.

---

**End of ADR-001**