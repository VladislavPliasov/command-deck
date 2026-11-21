<script setup lang="ts">
import { reactive, ref, watch } from "vue";
import { useToolStore } from "../../stores/tools";
import { toRegistryError } from "../../services/registry";
import CommandMetadataEditor from "./CommandMetadataEditor.vue";
import ToolCommands from "./ToolCommands.vue";
import {
  CATEGORY_VALUES,
  type Category,
  type IntegrationType,
  type RegistryErrorShape,
} from "../../types/tool";

const store = useToolStore();
const editing = ref(false);
const saving = ref(false);
const formError = ref<RegistryErrorShape | null>(null);
const editorRef = ref<InstanceType<typeof CommandMetadataEditor> | null>(null);
const editorJson = ref("");

const form = reactive({
  name: "",
  description: "",
  category: "other" as Category,
  tags: "",
  integrationType: "cli" as IntegrationType,
  documentationUrl: "",
  authorName: "",
  authorUrl: "",
  executablePath: "",
});

watch(
  () => store.selected,
  (profile) => {
    editing.value = false;
    formError.value = null;
    editorJson.value = profile?.commands
      ? JSON.stringify(profile.commands, null, 2)
      : "";
    if (profile) {
      form.name = profile.name;
      form.description = profile.description ?? "";
      form.category = profile.category;
      form.tags = profile.tags.join(", ");
      form.integrationType = profile.integrationType;
      form.documentationUrl = profile.documentationUrl ?? "";
      form.authorName = profile.author?.name ?? "";
      form.authorUrl = profile.author?.url ?? "";
      form.executablePath = profile.executable.path;
    }
  },
  { immediate: true },
);

function parseTags(): string[] {
  return form.tags
    .split(",")
    .map((t) => t.trim())
    .filter((t) => t.length > 0);
}

async function save() {
  if (!store.selected) {
    return;
  }
  const prepared = editorRef.value?.prepare() ?? { ok: true };
  if (!prepared.ok) {
    // JSON syntax error: keep the entered text, stay in the editor.
    return;
  }
  saving.value = true;
  formError.value = null;
  try {
    await store.update(store.selected.id, {
      name: form.name,
      description: form.description,
      category: form.category,
      tags: parseTags(),
      integrationType: form.integrationType,
      documentationUrl: form.documentationUrl,
      author: form.authorName
        ? { name: form.authorName, url: form.authorUrl || undefined }
        : undefined,
      executablePath: form.executablePath,
      // Only the commands metadata field; omitted entirely when unchanged.
      ...(prepared.commands !== undefined ? { commands: prepared.commands } : {}),
    });
    editing.value = false;
  } catch (e) {
    const err = toRegistryError(e);
    const isCommandError =
      (err.field && err.field.startsWith("commands")) ||
      /not allowed for kind|missing field `token`|duplicate field/.test(err.message);
    if (isCommandError) {
      editorRef.value?.setError(err);
    } else {
      formError.value = err;
    }
  } finally {
    saving.value = false;
  }
}

async function remove() {
  if (!store.selected) {
    return;
  }
  const ok = window.confirm(
    `Remove tool "${store.selected.name}" from the registry?`,
  );
  if (!ok) {
    return;
  }
  try {
    await store.remove(store.selected.id);
  } catch (e) {
    formError.value = toRegistryError(e);
  }
}
</script>

<template>
  <section v-if="store.selected" class="detail">
    <header class="detail-header">
      <div>
        <h2>{{ store.selected.name }}</h2>
        <p class="id-line">
          <code>{{ store.selected.id }}</code>
          <span class="badge">{{ store.selected.category }}</span>
          <span class="badge">{{ store.selected.integrationType }}</span>
          <span class="badge" :class="'install-' + store.selected.installStatus">
            {{ store.selected.installStatus }}
          </span>
          <span class="badge">
            verification: {{ store.selected.verification.status }}
          </span>
        </p>
      </div>
      <div class="actions">
        <button v-if="!editing" @click="editing = true">Edit</button>
        <button class="danger" @click="remove">Remove</button>
      </div>
    </header>

    <p v-if="formError" class="error">
      {{ formError.message }}
      <span v-if="formError.field">({{ formError.field }})</span>
    </p>

    <template v-if="!editing">
    <dl>
      <dt>Executable</dt>
      <dd>
        <code>{{ store.selected.executable.name }}</code>
      </dd>
      <dt>Path</dt>
      <dd>
        <code>{{ store.selected.executable.path }}</code>
      </dd>
      <dt>Registered executable version</dt>
      <dd>
        {{
          store.selected.version
            ? `${store.selected.version.value} (${store.selected.version.source})`
            : "—"
        }}
      </dd>
      <dt>Description</dt>
      <dd>{{ store.selected.description || "—" }}</dd>
      <dt>Tags</dt>
      <dd>{{ store.selected.tags.join(", ") || "—" }}</dd>
      <dt>Platforms</dt>
      <dd>{{ store.selected.platforms.join(", ") }}</dd>
      <dt>Documentation</dt>
      <dd>{{ store.selected.documentationUrl || "—" }}</dd>
      <dt>Author</dt>
      <dd>
        {{
          store.selected.author
            ? store.selected.author.name +
              (store.selected.author.url ? ` (${store.selected.author.url})` : "")
            : "—"
        }}
      </dd>
      <dt>Discovered</dt>
      <dd>
        {{ store.selected.discovery.source }} at
        {{ store.selected.discovery.discoveredAt }}
      </dd>
      <dt>Verification</dt>
      <dd>
        {{ store.selected.verification.status }} ({{
          store.selected.verification.source
        }})
      </dd>
      <dt>Metadata reference version</dt>
      <dd>
        {{
          store.selected.verification.toolVersion
            ? `${store.selected.verification.toolVersion} — authored/verified against (historical reference; not a compatibility guarantee)`
            : "—"
        }}
      </dd>
      <dt>Created</dt>
      <dd>{{ store.selected.createdAt }}</dd>
      <dt>Updated</dt>
      <dd>{{ store.selected.updatedAt }}</dd>
    </dl>

    <section class="commands-section">
      <h3>Commands</h3>
      <ToolCommands
        v-if="store.selected.commands?.length"
        :commands="store.selected.commands"
        :executable-name="store.selected.executable.name"
      />
      <p v-else class="hint">
        No command metadata for this tool. Use Edit &rarr; Command metadata
        (JSON) to add it.
      </p>
    </section>
    </template>

    <form v-else class="edit-form" @submit.prevent="save">
      <label>
        Name
        <input v-model="form.name" required />
      </label>
      <label>
        Description
        <textarea v-model="form.description" rows="3" />
      </label>
      <label>
        Category
        <select v-model="form.category">
          <option v-for="c in CATEGORY_VALUES" :key="c" :value="c">{{ c }}</option>
        </select>
      </label>
      <label>
        Tags (comma-separated)
        <input v-model="form.tags" />
      </label>
      <label>
        Integration type
        <select v-model="form.integrationType">
          <option value="cli">cli</option>
          <option value="gui">gui</option>
        </select>
      </label>
      <label>
        Executable path (absolute, .exe or .com)
        <input v-model="form.executablePath" required />
      </label>
      <label>
        Documentation URL
        <input v-model="form.documentationUrl" placeholder="https://..." />
      </label>
      <label>
        Author name
        <input v-model="form.authorName" />
      </label>
      <label>
        Author URL
        <input v-model="form.authorUrl" placeholder="https://..." />
      </label>
      <CommandMetadataEditor ref="editorRef" :initial-json="editorJson" />
      <div class="actions">
        <button type="submit" :disabled="saving">
          {{ saving ? "Saving..." : "Save" }}
        </button>
        <button type="button" @click="editing = false">Cancel</button>
      </div>
    </form>
  </section>
  <section v-else class="detail empty">
    <p>Select a tool to see its details.</p>
  </section>
</template>

<style scoped>
.detail {
  flex: 1;
  padding: 0.25rem 0.5rem;
  overflow-y: auto;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

h2 {
  margin: 0 0 0.25rem;
  font-size: 1.25rem;
}

.id-line {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
  align-items: center;
  margin: 0;
  color: #555;
  font-size: 0.85rem;
}

.badge {
  font-size: 0.72rem;
  padding: 0.1rem 0.45rem;
  border-radius: 999px;
  background: #eee;
  color: #444;
}

.install-installed {
  background: #d8f0d8;
  color: #1c5c1c;
}

.install-not-found {
  background: #f6dede;
  color: #8a1f1f;
}

.actions {
  display: flex;
  gap: 0.5rem;
}

button {
  padding: 0.4rem 0.9rem;
  border: 1px solid #bbb;
  border-radius: 4px;
  background: #f7f7f7;
  cursor: pointer;
}

button.danger {
  background: #fdecec;
  border-color: #d99;
  color: #8a1f1f;
}

dl {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 0.35rem 1.25rem;
  margin-top: 1rem;
  font-size: 0.9rem;
}

dt {
  font-weight: 600;
  color: #444;
}

dd {
  margin: 0;
  word-break: break-all;
}

.error {
  background: #fdecec;
  color: #8a1f1f;
  padding: 0.5rem 0.75rem;
  border-radius: 4px;
  font-size: 0.88rem;
}

.edit-form {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
  margin-top: 1rem;
  max-width: 34rem;
}

.edit-form label {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.85rem;
  font-weight: 600;
  color: #333;
}

.edit-form input,
.edit-form select,
.edit-form textarea {
  padding: 0.4rem 0.55rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 400;
}

.empty {
  color: #777;
}

.commands-section {
  margin-top: 1.25rem;
}

.commands-section h3 {
  font-size: 1.05rem;
  margin: 0 0 0.5rem;
}

.hint {
  color: #777;
  font-size: 0.88rem;
}
</style>
