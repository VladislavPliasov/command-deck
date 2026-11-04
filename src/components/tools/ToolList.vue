<script setup lang="ts">
import { computed } from "vue";
import { useToolStore } from "../../stores/tools";
import type { InstallStatus, VerificationStatus } from "../../types/tool";

const store = useToolStore();

const query = computed({
  get: () => store.query,
  set: (value: string) => {
    store.query = value;
  },
});

const installLabel: Record<InstallStatus, string> = {
  installed: "Installed",
  "not-found": "Not found",
  unknown: "Unknown",
};

const verificationLabel: Record<VerificationStatus, string> = {
  unverified: "Unverified",
  "partially-verified": "Partially verified",
  verified: "Verified",
  "manually-configured": "Manually configured",
  incompatible: "Incompatible",
};

defineExpose({ installLabel, verificationLabel });
</script>

<template>
  <nav class="tool-list">
    <input
      v-model="query"
      class="search"
      type="search"
      placeholder="Search tools..."
      aria-label="Search tools"
    />
    <p v-if="store.items.length && !store.filtered.length" class="hint">
      No tools match "{{ store.query }}".
    </p>
    <ul>
      <li
        v-for="tool in store.filtered"
        :key="tool.id"
        :class="{ selected: tool.id === store.selectedId }"
        @click="store.select(tool.id)"
      >
        <span class="row-name">{{ tool.name }}</span>
        <span class="row-meta">
          <span class="badge">{{ tool.category }}</span>
          <span class="badge" :class="'install-' + tool.installStatus">
            {{ installLabel[tool.installStatus] }}
          </span>
        </span>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.tool-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 16rem;
  max-width: 20rem;
  border-right: 1px solid #ddd;
  padding-right: 0.75rem;
}

.search {
  width: 100%;
  padding: 0.45rem 0.6rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.9rem;
}

.hint {
  color: #777;
  font-size: 0.85rem;
  margin: 0.25rem 0;
}

ul {
  list-style: none;
  margin: 0;
  padding: 0;
  overflow-y: auto;
}

li {
  padding: 0.5rem 0.6rem;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

li:hover {
  background: #f2f2f2;
}

li.selected {
  background: #e4ecfb;
}

.row-name {
  font-weight: 600;
  font-size: 0.92rem;
}

.row-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
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

.install-unknown {
  background: #eee8d8;
  color: #6d5b1c;
}
</style>
