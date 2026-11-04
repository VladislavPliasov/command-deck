<script setup lang="ts">
import { reactive, ref } from "vue";
import { resolveDiscovery, toRegistryError } from "../../services/registry";
import { useToolStore } from "../../stores/tools";
import {
  CATEGORY_VALUES,
  type Category,
  type DiscoveryResult,
  type IntegrationType,
  type RegistryErrorShape,
} from "../../types/tool";

const emit = defineEmits<{ close: [] }>();
const store = useToolStore();

const form = reactive({
  executablePath: "",
  name: "",
  integrationType: "cli" as IntegrationType,
  category: "other" as Category,
  tags: "",
  description: "",
  documentationUrl: "",
});

const submitting = ref(false);
const formError = ref<RegistryErrorShape | null>(null);
const discoverQuery = ref("");
const discoverBusy = ref(false);
const discoverResult = ref<DiscoveryResult | null>(null);
const discoverError = ref<string | null>(null);

function fieldError(field: string): string | null {
  return formError.value?.field === field ? formError.value.message : null;
}

async function discover() {
  discoverBusy.value = true;
  discoverError.value = null;
  discoverResult.value = null;
  try {
    discoverResult.value = await resolveDiscovery(discoverQuery.value);
  } catch (e) {
    discoverError.value = toRegistryError(e).message;
  } finally {
    discoverBusy.value = false;
  }
}

function useMatch(path: string) {
  form.executablePath = path;
}

async function submit() {
  submitting.value = true;
  formError.value = null;
  try {
    await store.register({
      executablePath: form.executablePath,
      name: form.name || undefined,
      integrationType: form.integrationType,
      category: form.category,
      tags: form.tags
        .split(",")
        .map((t) => t.trim())
        .filter((t) => t.length > 0),
      description: form.description || undefined,
      documentationUrl: form.documentationUrl || undefined,
    });
    emit("close");
  } catch (e) {
    formError.value = toRegistryError(e);
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <div class="dialog" role="dialog" aria-label="Import tool">
      <h3>Import tool</h3>
      <p class="note">
        Register a tool by its executable path. Only existing
        <code>.exe</code> / <code>.com</code> files are supported. Nothing is
        executed during import.
      </p>

      <details class="discover">
        <summary>Find an executable on PATH</summary>
        <div class="discover-row">
          <input
            v-model="discoverQuery"
            placeholder="e.g. nmap"
            @keyup.enter="discover"
          />
          <button type="button" :disabled="discoverBusy" @click="discover">
            {{ discoverBusy ? "Searching..." : "Find" }}
          </button>
        </div>
        <p v-if="discoverError" class="error">{{ discoverError }}</p>
        <template v-if="discoverResult">
          <p class="discover-status">
            Status: <strong>{{ discoverResult.status }}</strong>
          </p>
          <ul class="discover-matches">
            <li
              v-for="m in discoverResult.matches"
              :key="m.path"
              @click="useMatch(m.path)"
            >
              <code>{{ m.path }}</code>
              <span v-if="m.fileVersion"> — {{ m.fileVersion }}</span>
            </li>
          </ul>
          <p
            v-if="discoverResult.status === 'invalid-name'"
            class="error"
          >
            Invalid name. Use a bare executable name (optionally with a
            <code>.exe</code> or <code>.com</code> extension), no path
            separators.
          </p>
        </template>
      </details>

      <form @submit.prevent="submit">
        <label>
          Executable path *
          <input
            v-model="form.executablePath"
            required
            placeholder="C:\\Tools\\nmap\\nmap.exe"
          />
          <span v-if="fieldError('executablePath')" class="error">
            {{ fieldError("executablePath") }}
          </span>
        </label>
        <label>
          Name (defaults to the file name)
          <input v-model="form.name" />
          <span v-if="fieldError('name')" class="error">
            {{ fieldError("name") }}
          </span>
        </label>
        <div class="grid2">
          <label>
            Integration type
            <select v-model="form.integrationType">
              <option value="cli">cli</option>
              <option value="gui">gui</option>
            </select>
          </label>
          <label>
            Category
            <select v-model="form.category">
              <option v-for="c in CATEGORY_VALUES" :key="c" :value="c">
                {{ c }}
              </option>
            </select>
          </label>
        </div>
        <label>
          Tags (comma-separated)
          <input v-model="form.tags" />
        </label>
        <label>
          Description
          <textarea v-model="form.description" rows="2" />
        </label>
        <label>
          Documentation URL
          <input v-model="form.documentationUrl" placeholder="https://..." />
          <span v-if="fieldError('documentationUrl')" class="error">
            {{ fieldError("documentationUrl") }}
          </span>
        </label>
        <p v-if="formError && !formError.field" class="error">
          {{ formError.message }}
        </p>
        <div class="actions">
          <button type="submit" :disabled="submitting">
            {{ submitting ? "Importing..." : "Import" }}
          </button>
          <button type="button" @click="emit('close')">Cancel</button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 20;
}

.dialog {
  background: #fff;
  border-radius: 6px;
  padding: 1.25rem 1.5rem;
  width: min(34rem, 92vw);
  max-height: 88vh;
  overflow-y: auto;
}

h3 {
  margin: 0 0 0.5rem;
}

.note {
  font-size: 0.85rem;
  color: #555;
  margin-top: 0;
}

.discover {
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 0.5rem 0.75rem;
  margin-bottom: 0.9rem;
  font-size: 0.88rem;
}

.discover-row {
  display: flex;
  gap: 0.5rem;
  margin: 0.5rem 0;
}

.discover-row input {
  flex: 1;
}

.discover-status {
  margin: 0.25rem 0;
}

.discover-matches {
  margin: 0.25rem 0 0;
  padding-left: 1.1rem;
}

.discover-matches li {
  cursor: pointer;
  margin: 0.15rem 0;
}

.discover-matches li:hover {
  text-decoration: underline;
}

form {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

label {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.85rem;
  font-weight: 600;
  color: #333;
}

input,
select,
textarea {
  padding: 0.4rem 0.55rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 400;
}

.grid2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.6rem;
}

.error {
  color: #8a1f1f;
  font-size: 0.82rem;
  font-weight: 400;
  margin: 0;
}

.actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
  margin-top: 0.35rem;
}

button {
  padding: 0.45rem 1rem;
  border: 1px solid #bbb;
  border-radius: 4px;
  background: #f7f7f7;
  cursor: pointer;
}

button[type="submit"] {
  background: #2f5fc4;
  border-color: #2f5fc4;
  color: #fff;
}
</style>
