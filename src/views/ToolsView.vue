<script setup lang="ts">
import { onMounted, ref } from "vue";
import ToolDetail from "../components/tools/ToolDetail.vue";
import ToolImportDialog from "../components/tools/ToolImportDialog.vue";
import ToolList from "../components/tools/ToolList.vue";
import { useToolStore } from "../stores/tools";

const store = useToolStore();
const showImport = ref(false);

onMounted(() => {
  store.load();
});
</script>

<template>
  <section class="tools-view">
    <header class="toolbar">
      <h2>Tools</h2>
      <span class="count">{{ store.items.length }} registered</span>
      <button class="primary" @click="showImport = true">Import tool</button>
    </header>

    <div v-if="store.warnings.length" class="warnings">
      <strong>Some stored profiles could not be loaded:</strong>
      <ul>
        <li v-for="w in store.warnings" :key="w">{{ w }}</li>
      </ul>
    </div>

    <p v-if="store.error" class="error">{{ store.error }}</p>

    <div v-if="!store.items.length && !store.loading" class="empty-state">
      <p>No tools registered yet.</p>
      <p>
        Import a tool by its executable path, or search PATH from the import
        dialog.
      </p>
      <button class="primary" @click="showImport = true">Import tool</button>
    </div>

    <div v-else class="panes">
      <ToolList />
      <ToolDetail />
    </div>

    <ToolImportDialog v-if="showImport" @close="showImport = false" />
  </section>
</template>

<style scoped>
.tools-view {
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 1rem 1.25rem;
  min-height: 24rem;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  margin-bottom: 0.85rem;
}

.toolbar h2 {
  margin: 0;
  font-size: 1.15rem;
}

.count {
  color: #777;
  font-size: 0.85rem;
  flex: 1;
}

button {
  padding: 0.45rem 0.95rem;
  border: 1px solid #bbb;
  border-radius: 4px;
  background: #f7f7f7;
  cursor: pointer;
}

button.primary {
  background: #2f5fc4;
  border-color: #2f5fc4;
  color: #fff;
}

.warnings {
  background: #fdf3d7;
  border: 1px solid #e3c76a;
  border-radius: 4px;
  padding: 0.55rem 0.85rem;
  margin-bottom: 0.75rem;
  font-size: 0.85rem;
}

.warnings ul {
  margin: 0.35rem 0 0;
  padding-left: 1.15rem;
}

.error {
  background: #fdecec;
  color: #8a1f1f;
  padding: 0.55rem 0.85rem;
  border-radius: 4px;
  font-size: 0.88rem;
}

.empty-state {
  text-align: center;
  padding: 3rem 1rem;
  color: #666;
}

.panes {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}
</style>
