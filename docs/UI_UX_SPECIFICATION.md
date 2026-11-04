# Command Deck — UI/UX Specification

**Document Version:** 1.0  
**Status:** Initial Design Specification

---

## 1. Design Objective

Command Deck must provide a professional, information-dense graphical environment for cybersecurity operations.

The interface should combine the usability of a modern web browser with the functionality of a professional cybersecurity workstation.

The primary interface reference is Chrome / Tor Browser.

The design must not imitate Arc or a traditional IDE as its primary navigation model.

## 2. Visual Direction

Requirements:

- Dark theme by default.
- Professional appearance.
- High information density.
- Clear visual hierarchy.
- Consistent spacing.
- Readable typography.
- Minimal decorative elements.
- Clear process and status indicators.
- Responsive layout.

The interface must prioritize usability over visual effects.

## 3. Main Application Layout

The main window consists of four primary regions:

1. Top tab bar.
2. Navigation and omnibox area.
3. Left sidebar.
4. Central workspace.

Optional right-side panels may display contextual information, settings, or tool-specific controls.

## 4. Top Tab Bar

The tab bar must behave similarly to Chrome.

Required functionality:

- Create a new tab.
- Close a tab.
- Switch between tabs.
- Pin tabs.
- Reorder tabs.
- Restore recently closed tabs.
- Restore the previous session.
- Group tabs.
- Display running or busy states.
- Support multiple instances of the same utility.

Each tab must have an independent identity.

A tab should store:

- Tab ID.
- Workspace type.
- Tool ID, when applicable.
- Project context.
- Configuration state.
- Execution references.
- Navigation state.
- Restoration metadata.

Closing a tab must not silently terminate an associated process.

The application must distinguish between closing a view and stopping an execution.

## 5. Navigation Area

The navigation area should contain:

- Back navigation.
- Forward navigation.
- Refresh or reload where applicable.
- Universal search/address bar.

The omnibox must support searching for:

- Tools.
- Commands.
- CVEs.
- Exploits.
- Projects.
- Results.
- Documentation.
- Workflows.

It may also support direct navigation to known application sections.

## 6. Left Sidebar

The sidebar provides access to primary application areas.

Suggested sections:

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

The sidebar must support:

- Collapsing.
- Expansion.
- Clear section indicators.
- Optional favorites.
- Contextual shortcuts.

The sidebar should not replace the top horizontal tab model.

## 7. Dashboard

The Dashboard is the main overview, not a tool catalog.

Suggested widgets:

- Active Processes.
- Recent Tasks.
- Recent Results.
- Vulnerability Overview.
- Favorite Tools.
- System Resources.
- Notifications.
- Quick Actions.

Widgets should be modular and configurable.

## 8. Tool Workspace

When a tool is opened, it must appear inside Command Deck as a separate tab.

A tool workspace may contain:

- Tool name and version.
- Availability status.
- Command Builder.
- Tool-specific settings.
- Execution controls.
- Output panel.
- Results panel.
- Documentation.
- Execution history.

Multiple instances of the same tool must be allowed.

For example, users may open two Nmap workspaces with different configurations.

## 9. Command Builder Interface

The Command Builder should follow a parameter-oriented interaction model.

The user selects a tool first.

The interface then displays available commands, subcommands, options, and arguments.

When a parameter is selected or focused, the interface should display:

- Parameter name.
- Flag.
- Description.
- Syntax.
- Expected value type.
- Accepted values.
- Default value, if known.
- Requirements.
- Conflicts.
- Examples.
- Version compatibility.

The user configures parameters step by step.

The interface must show the resulting command preview.

Before execution, the user must be able to inspect the complete command and confirm it.

The interface must provide a manual editing mode.

## 10. Terminal Workspace

The terminal must be a separate application tab.

It must provide a real interactive shell, not a simulated terminal.

Requirements:

- Input and output.
- Interactive programs.
- Multiple terminal sessions.
- Working directory display.
- Process interruption.
- Scrollback.
- Copy and paste.
- Terminal resizing.

## 11. Results Center

Results must be organized in a structured interface.

Suggested views:

- All Results.
- By Project.
- By Tool.
- By Task.
- Hosts.
- Services.
- Vulnerabilities.
- Evidence.
- Raw Output.

Users must be able to open a result and navigate to its source execution.

Original output must remain accessible.

## 12. Projects Interface

Each project should have its own workspace.

Suggested sections:

- Overview.
- Scope.
- Targets.
- Tasks.
- Results.
- Vulnerabilities.
- Evidence.
- Notes.
- Workflows.
- Reports.

Project context should be available to relevant tool tabs.

## 13. Process Manager

The Process Manager should display:

- Process name.
- Associated tool.
- Project.
- Start time.
- Current status.
- CPU usage.
- Memory usage.
- Execution duration.
- Exit code.
- Available controls.

Users should be able to inspect logs and stop supported executions.

## 14. Exploit Intelligence Interface

The interface should support:

- CVE search.
- SearchSploit results.
- External references.
- GitHub PoC references.
- Compatibility information.
- Requirements.
- Source information.
- Manual freshness checks.

Potential vulnerability matches must be visually distinguished from verified findings.

## 15. Plugin Manager

The Plugin Manager should provide:

- Installed plugins.
- Available plugins.
- Search.
- Plugin details.
- Permissions.
- Installation.
- Updates.
- Removal.
- Enable and disable controls.

Plugin permissions must be visible before installation or activation.

## 16. Notifications

Notifications should communicate:

- Process completion.
- Process failure.
- Execution errors.
- Database updates.
- Plugin updates.
- Workflow state changes.
- Scope validation failures.

Notifications should be informative and non-intrusive.

## 17. Interaction Principles

- Important actions must have visible feedback.
- Destructive actions require confirmation.
- Running processes must be distinguishable from inactive tools.
- Errors must explain what happened and, where possible, how to resolve them.
- No important action should depend exclusively on color.
- Long-running tasks must not freeze the interface.
- The user must retain control over execution.

## 18. Responsive Behavior

The first release should prioritize desktop Linux environments.

The layout must remain usable when the window is resized.

Mobile support is not a first-release requirement.

## 19. Accessibility

Consider:

- Keyboard navigation.
- Tooltips.
- Clear focus indicators.
- Readable text.
- Adjustable font sizes.
- Accessible status indicators.

## 20. Design Acceptance Criteria

The UI foundation is acceptable when:

- Tabs can be created, closed, reordered, and restored.
- Multiple workspaces can remain open.
- The sidebar navigates between application sections.
- The omnibox provides global search.
- Tools open inside application tabs.
- Closing a tab does not accidentally terminate an unrelated process.
- Long-running tasks do not block interface interaction.
- The interface remains usable at common desktop resolutions.

---

**End of UI/UX Specification**