# ADR-003: Tool Command Metadata

**Status:** Accepted
**Decision Date:** 2026-11-06
**Decision Owner:** Command Deck Project
**Version:** 1.0

---

## 1. Context

P0-04 adds structured command and parameter metadata to Tool Profiles
(schema 1.1) as the foundation for the future Command Builder. TOOL_INTEGRATION
(section 5-7) reserves a `commands` field and sketches parameter metadata, but
no finalized grammar model exists. This ADR records the approved model,
validation policy, trust rules, and schema/versioning decisions.

## 2. Data model (schema 1.1)

- `commands: CommandSpec[]` - optional; omitted entirely when absent
  (never serialized as `null`). `Some([])` is structurally distinct from
  absent and is stored as `[]`.
- `CommandSpec`: id (slug), token (**required-nullable**: missing is a
  deserialization error; explicit `null` = direct/executable-level mode,
  allowed only on top-level entries and at most once; nested subcommands
  require a non-null string token), name, summary?, description?,
  syntaxExamples[], versionConstraints[], parameters[], subcommands[].
- `ParameterSpec`: id, kind (`flag|option|positional|custom`), label,
  token? (optional field; flag/option require it semantically, positional
  forbids it), aliases?, description?, syntax?, value?, required, repeatable,
  requires[], conflicts[], examples[], versionConstraints[], order?
  (required and unique per command for positionals).
- `ValueSpec`: valueType (13 values from TOOL_INTEGRATION section 6),
  choices[] (only for `enum`; non-empty and unique), default?, multiple
  (one occurrence accepts multiple values), optional (a valued option may be
  given without its value).
- `verification.toolVersion?`: the version the metadata was originally
  authored/verified against. Historical reference only; never a compatibility
  guarantee. Never auto-populated, overwritten, or cleared by application
  logic.

Field semantics: `required` = the parameter must be supplied;
`repeatable` = it may occur multiple times; `value.multiple` = one occurrence
accepts multiple values; `value.optional` = the value may be omitted.

## 3. Parameter semantics matrix (S1-S10)

S1 flags never carry a value block; S2 flags may repeat; S3 options require a
value block; S4 `value.optional` only on options; S5 `value.multiple` and
`value.optional` are mutually exclusive; S6 `repeatable`+`value.multiple`
allowed on options; S7 `repeatable` forbidden on positionals (use
`value.multiple`); S8 positional `value` + unique `order` required; S9
`choices` non-empty if and only if `valueType: "enum"`; S10 `custom` requires
`syntax` or `description`.

## 4. Custom parameter contract (closed)

Allowed fields: id, kind, label, token, syntax, description, examples,
versionConstraints, required, repeatable, requires, conflicts. Forbidden:
`value`, `aliases`, `order` (and every unknown field).

Enforcement is two-stage by design:
1. Deserialization (hand-written `ParameterSpec` visitor): unknown fields -
   keys outside the ParameterSpec field set - are rejected with an error
   naming the key. Non-custom parameters tolerate unknown fields (forward
   compatibility). Duplicate JSON keys are rejected at every level
   (`duplicate field` errors); input is deliberately NOT buffered through
   serde_json::Value/Map, which would silently deduplicate keys.
2. Semantic validation: the known-but-forbidden fields (`value`, `aliases`,
   `order`) are rejected with field paths (e.g.
   `commands[0].parameters[2].value`), because they are regular model fields
   that deserialize normally.

Unknown value shapes belong in `valueType: "custom"` on an option/positional;
`kind: "custom"` is reserved for parameters whose structure cannot be modeled
(config files, environment variables, interactive behavior - TOOL_INTEGRATION
section 7).

## 5. Version semantics

Three distinct concepts:
1. `verification.toolVersion` - metadata reference version (authored or
   verified against). Historical record.
2. `profile.version` - registered/detected executable version.
3. `versionConstraints[]` - display-only per-command/per-parameter notes
   (basic shape validation only: non-empty, <= 64 chars, <= 8 entries; no
   parser, no semantic-version enforcement, no compatibility checks).

The UI labels keep these separate and never imply compatibility.

## 6. Trust and verification

- No UI affordance can promote `verification.status` in P0-04.
- Command metadata changes are detected by deterministic structural equality
  (Rust `PartialEq` on the serialized structure: array order significant,
  strings exact, `None` != `Some([])`, no normalization).
- On change: `verified` and `partially-verified` downgrade to `unverified`;
  `manually-configured`, `incompatible`, and `unverified` are preserved.
  Unchanged metadata preserves any status. Non-command field edits never
  affect status. `toolVersion` is preserved through all updates and
  downgrades.

## 7. Schema versioning and stamping

- `schemaVersion` matches `^1\.[0-9]+$`; readers accept any 1.x; writers
  stamp `1.1` on every explicit profile write (register and update). Reads
  never write. Absent optional fields remain absent unless explicitly
  supplied. Structural JSON compatibility is guaranteed; byte-identical
  re-serialization is not.
- The published schema stays on JSON Schema draft 2020-12 (which supports
  `contains`/`maxContains`, used for the single direct-mode entry rule).
  Keywords unsupported by the declared draft are never used.

## 8. Schema <-> Rust consistency

The JSON Schema encodes what it can express: types, required fields, enums,
patterns, lengths, the S1-S10 conditionals, the closed custom contract
(conditional + `additionalProperties: false`), per-array caps, and the
direct-mode count. Rust validation is the authoritative superset and also
enforces what JSON Schema cannot: tree-wide id uniqueness, sibling
token/alias uniqueness, `requires`/`conflicts` reference integrity,
positional `order` uniqueness, `choices` uniqueness, nesting depth <= 5,
tree-total command cap (<= 200), and required-nullable `token` at
deserialization. A drift-guard test asserts the schema's custom `then`
branch matches the Rust allowlist.

## 9. Consequences

- The metadata model covers the Command Builder's documented grammar
  requirements without implementing execution; profiles remain
  human-readable JSON files per ADR-002.
- Two serialization rules must be remembered: `CommandSpec.token` is always
  emitted (`null` or string), while `ParameterSpec.token` and other optional
  fields are omitted when absent.
- Deferred: command execution/argv assembly, visual Command Builder,
  automatic `--help` parsing/extraction, per-parameter provenance,
  versionConstraints parsing, verification promotion UI, `toolVersion`
  capture UI, mutually exclusive groups, value-dependent conditions.

---

**End of ADR-003**
