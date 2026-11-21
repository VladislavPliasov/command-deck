<script setup lang="ts">
import { ref, watch } from "vue";
import type { CommandSpec, RegistryErrorShape } from "../../types/tool";

const props = defineProps<{
  /** Pretty-printed commands JSON, or "" when the profile has none. */
  initialJson: string;
}>();

export interface Prepared {
  ok: boolean;
  /** undefined = leave command metadata unchanged */
  commands?: CommandSpec[];
  error?: string;
}

const text = ref("");
const syntaxError = ref<string | null>(null);
const serverError = ref<RegistryErrorShape | null>(null);

watch(
  () => props.initialJson,
  (value) => {
    text.value = value;
    syntaxError.value = null;
    serverError.value = null;
  },
  { immediate: true },
);

/**
 * Validates the editor content and returns what should be sent to the
 * backend. Never discards the user's text. Only the `commands` field is
 * affected: undefined means "unchanged"; [] clears the metadata.
 */
function prepare(): Prepared {
  syntaxError.value = null;
  serverError.value = null;
  const raw = text.value.trim();
  let current: CommandSpec[];
  try {
    current = raw === "" ? [] : JSON.parse(raw);
    if (!Array.isArray(current)) {
      throw new Error("top-level value must be an array of commands");
    }
  } catch (e) {
    syntaxError.value = `JSON syntax error: ${e instanceof Error ? e.message : String(e)}`;
    return { ok: false, error: syntaxError.value };
  }
  // Structural change detection mirrors the Rust rule: None (no metadata)
  // is distinct from Some([]).
  const initial: CommandSpec[] | null =
    props.initialJson.trim() === "" ? null : JSON.parse(props.initialJson);
  const initialSentinel = initial === null ? "\u0000none" : JSON.stringify(initial);
  const currentSentinel = JSON.stringify(current);
  if (initialSentinel === currentSentinel) {
    return { ok: true }; // unchanged - do not touch the field
  }
  return { ok: true, commands: current };
}

function setError(error: RegistryErrorShape | null) {
  serverError.value = error;
}

defineExpose({ prepare, setError });
</script>

<template>
  <details class="command-metadata">
    <summary>Command metadata (JSON)</summary>
    <p class="help">
      Optional structured command/parameter metadata (schema 1.1). Use
      <code>[]</code> to clear it. Each parameter of kind
      <code>custom</code> may only contain: id, kind, label, token, syntax,
      description, examples, versionConstraints, required, repeatable,
      requires, conflicts. Nested subcommands need a non-null string token;
      <code>"token": null</code> marks the top-level direct mode. Invalid JSON
      or metadata is rejected and your text is preserved.
    </p>
    <textarea
      v-model="text"
      rows="12"
      spellcheck="false"
      placeholder="[]"
    ></textarea>
    <p v-if="syntaxError" class="error syntax">{{ syntaxError }}</p>
    <p v-if="serverError" class="error validation">
      Validation error<span v-if="serverError.field">
        — <code>{{ serverError.field }}</code></span
      >: {{ serverError.message }}
    </p>
  </details>
</template>

<style scoped>
.command-metadata {
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 0.55rem 0.75rem;
}

summary {
  font-weight: 600;
  cursor: pointer;
  font-size: 0.9rem;
}

.help {
  font-size: 0.8rem;
  color: #666;
  margin: 0.5rem 0;
}

textarea {
  width: 100%;
  font-family: ui-monospace, monospace;
  font-size: 0.78rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 0.5rem;
  box-sizing: border-box;
}

.error {
  font-size: 0.82rem;
  margin: 0.45rem 0 0;
  padding: 0.45rem 0.65rem;
  border-radius: 4px;
}

.error.syntax {
  background: #fdecec;
  color: #8a1f1f;
}

.error.validation {
  background: #fdf3d7;
  color: #6d5b1c;
}
</style>
