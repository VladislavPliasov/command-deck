# ADR-002: Tool Registry Storage

**Status:** Accepted
**Decision Date:** 2026-11-04
**Decision Owner:** Command Deck Project
**Version:** 1.0

---

## 1. Context

P0-03 introduces the Tool Registry: local registration of external tool
profiles (name, category, executable path, verification status, ...).

The registry must persist locally, support CRUD, and remain small enough
for one developer to maintain. Two storage options were evaluated:

- SQLite (rusqlite) with a tools table.
- One JSON file per tool profile in the Tauri app data directory.

SQLite brings transactions and querying, but also a new dependency,
migrations, and schema-management machinery that the current dataset
(dozens to low hundreds of profiles) does not need. Tool profiles are
already defined as JSON documents (see schemas/tool-profile.schema.json),
which makes per-tool files a natural fit for future community/catalog
profile import and human inspection.

## 2. Decision

Tool profiles are stored as one JSON file per tool at:

    <tauri app_data_dir()>/registry/<id>.json

- Windows: `%APPDATA%\org.commanddeck.app\registry\`
- Linux (future target): `~/.local/share/org.commanddeck.app/registry/`

Implementation notes:

- The `<id>` is validated as a Windows-safe filename before any
  filesystem operation (charset, length, no trailing dot, no reserved
  device names such as CON/PRN/AUX/NUL/COM1-9/LPT1-9, no path traversal).
- Writes are atomic: content is written to `<id>.json.tmp` and renamed
  over the target. Failed writes leave the previous profile intact
  (covered by tests in `src-tauri/src/registry/storage.rs`).
- Corrupt or invalid files are skipped and reported as load warnings;
  they never break the registry.
- SQLite is deferred to the phase that stores executions, raw output,
  and structured results, where querying and large data justify it.

Schema consistency strategy (no `jsonschema` dependency):

- `schemas/tool-profile.schema.json` is the published contract.
- `schemas/examples/tool-profile.example.json` is validated by the Rust
  validator in tests and must deserialize into the Rust model.
- A golden serialization test pins the model's JSON field names.
- Rule: schema edits must update the Rust model, the schema file, and
  the fixtures together in one change. The `jsonschema` crate was
  evaluated and rejected for this purpose (149-crate dependency tree for
  a single conformance test).

## 3. Consequences

- Profiles are human-readable, diffable, and trivially exportable.
- No new runtime dependencies; storage lives behind
  `src-tauri/src/registry/storage.rs` so a later SQLite swap is local.
- Known limitation (accepted for P0-03): multiple concurrent application
  instances are not prevented. Two instances writing the same registry
  can lose updates (last writer wins per file). A single-instance guard
  or file locking is intentionally deferred; this limitation must be
  revisited before any multi-instance or sync feature.
- Locally registered profiles always carry an absolute executable path
  (catalog profiles with unknown paths are a future schema extension).

## 4. Related P0-03 policies

- Manual import supports only `.exe` and `.com` executables; discovery
  returns only those types (no `.bat`/`.cmd`/`.ps1`).
- Duplicate ids and duplicate canonical executable paths are rejected.
- Windows PE version metadata is read best-effort, read-only, and never
  blocks registration or discovery; discovered executables are never
  executed.

---

**End of ADR-002**
