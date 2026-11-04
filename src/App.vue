<script setup lang="ts">
import AppInfoPanel from "./components/AppInfoPanel.vue";
import ToolsView from "./views/ToolsView.vue";
</script>

<template>
  <main class="shell">
    <header class="app-header">
      <h1>Command Deck</h1>
      <p class="subtitle">Universal Cybersecurity Workstation</p>
    </header>

    <ToolsView />

    <footer class="app-footer">
      <AppInfoPanel />
    </footer>
  </main>
</template>

<style scoped>
.shell {
  max-width: 68rem;
  margin: 2rem auto;
  padding: 0 1.5rem 2.5rem;
  font-family: system-ui, sans-serif;
  color: #1a1a1a;
  display: flex;
  flex-direction: column;
  gap: 1.15rem;
}

.app-header h1 {
  font-size: 1.5rem;
  margin-bottom: 0.15rem;
}

.subtitle {
  margin: 0;
  color: #666;
  font-size: 0.9rem;
}

.app-footer {
  opacity: 0.85;
  transform: scale(0.92);
  transform-origin: top left;
}
</style>
