<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface AppInfo {
  name: string;
  version: string;
  os: string;
  arch: string;
}

const info = ref<AppInfo | null>(null);
const error = ref<string | null>(null);

onMounted(async () => {
  try {
    info.value = await invoke<AppInfo>("app_info");
  } catch (e) {
    error.value = String(e);
  }
});
</script>

<template>
  <section>
    <h2>Backend IPC check — <code>app_info</code></h2>
    <p v-if="error" class="error">invoke("app_info") failed: {{ error }}</p>
    <dl v-else-if="info">
      <div>
        <dt>Name</dt>
        <dd>{{ info.name }}</dd>
      </div>
      <div>
        <dt>Version</dt>
        <dd>{{ info.version }}</dd>
      </div>
      <div>
        <dt>OS</dt>
        <dd>{{ info.os }}</dd>
      </div>
      <div>
        <dt>Arch</dt>
        <dd>{{ info.arch }}</dd>
      </div>
    </dl>
    <p v-else>Waiting for Rust backend&hellip;</p>
  </section>
</template>

<style scoped>
section {
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 1rem 1.25rem;
}

h2 {
  font-size: 1rem;
  margin-top: 0;
}

dl {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.35rem 1.25rem;
  margin-bottom: 0;
}

dt {
  font-weight: 600;
  color: #444;
}

dd {
  margin: 0;
  font-family: ui-monospace, monospace;
}

.error {
  color: #b00020;
  font-family: ui-monospace, monospace;
}
</style>
