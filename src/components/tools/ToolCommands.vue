<script setup lang="ts">
import { computed, ref } from "vue";
import type { CommandSpec, ParameterSpec } from "../../types/tool";
import ParameterDetail from "./ParameterDetail.vue";

const props = defineProps<{
  commands: CommandSpec[];
  executableName: string;
}>();

interface Row {
  key: string;
  depth: number;
  command: CommandSpec;
}

const rows = computed<Row[]>(() => {
  const out: Row[] = [];
  const walk = (commands: CommandSpec[], prefix: string, depth: number) => {
    commands.forEach((command, index) => {
      const key = prefix ? `${prefix}.${index}` : `${index}`;
      out.push({ key, depth, command });
      walk(command.subcommands, key, depth + 1);
    });
  };
  walk(props.commands, "", 0);
  return out;
});

const selectedKey = ref<string | null>(null);
const selectedParam = ref<ParameterSpec | null>(null);

const selected = computed<CommandSpec | null>(
  () => rows.value.find((r) => r.key === selectedKey.value)?.command ?? null,
);

function select(row: Row) {
  selectedKey.value = row.key;
  selectedParam.value = null;
}

function labelFor(command: CommandSpec): string {
  return command.token === null
    ? `${props.executableName} (direct)`
    : command.name;
}

function tokenFor(command: CommandSpec): string {
  return command.token ?? "(direct)";
}
</script>

<template>
  <div class="commands">
    <div class="command-list">
      <p class="hint">Select a command to inspect its parameters.</p>
      <ul>
        <li
          v-for="row in rows"
          :key="row.key"
          :class="{ selected: row.key === selectedKey }"
          :style="{ paddingLeft: `${0.6 + row.depth * 1.1}rem` }"
          @click="select(row)"
        >
          <span class="cmd-name">{{ labelFor(row.command) }}</span>
          <span class="cmd-token"><code>{{ tokenFor(row.command) }}</code></span>
          <span class="badge">{{ row.command.parameters.length }} params</span>
        </li>
      </ul>
    </div>

    <div class="command-detail">
      <template v-if="selected">
        <h4>
          {{ selected.name }}
          <code class="cmd-token-inline">{{ tokenFor(selected) }}</code>
        </h4>
        <p v-if="selected.summary" class="summary">{{ selected.summary }}</p>
        <p v-if="selected.description" class="description">{{ selected.description }}</p>

        <div v-if="selected.syntaxExamples.length" class="examples">
          <strong>Syntax examples</strong>
          <div v-for="e in selected.syntaxExamples" :key="e"><code>{{ e }}</code></div>
        </div>

        <p v-if="selected.versionConstraints.length" class="constraints">
          <strong>Documented version constraints (syntax not validated):</strong>
          {{ selected.versionConstraints.join("; ") }}
        </p>

        <table v-if="selected.parameters.length">
          <thead>
            <tr>
              <th>Parameter</th>
              <th>Kind</th>
              <th>Token</th>
              <th>Required</th>
              <th>Value type</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="p in selected.parameters"
              :key="p.id"
              :class="{ selected: selectedParam?.id === p.id }"
              @click="selectedParam = p"
            >
              <td>{{ p.label }}</td>
              <td><span class="badge">{{ p.kind }}</span></td>
              <td><code v-if="p.token">{{ p.token }}</code><span v-else>—</span></td>
              <td>{{ p.required ? "yes" : "no" }}</td>
              <td>{{ p.value ? p.value.valueType : "—" }}</td>
            </tr>
          </tbody>
        </table>
        <p v-else class="hint">No parameters documented for this command.</p>

        <ParameterDetail v-if="selectedParam" :param="selectedParam" />
      </template>
      <p v-else class="hint">
        {{ rows.length ? "No command selected." : "No command metadata." }}
      </p>
    </div>
  </div>
</template>

<style scoped>
.commands {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.command-list {
  min-width: 15rem;
  max-width: 19rem;
  border-right: 1px solid #ddd;
  padding-right: 0.75rem;
}

.command-list ul {
  list-style: none;
  margin: 0.35rem 0 0;
  padding: 0;
}

.command-list li {
  padding: 0.4rem 0.55rem;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
  align-items: center;
  font-size: 0.88rem;
}

.command-list li:hover {
  background: #f2f2f2;
}

.command-list li.selected {
  background: #e4ecfb;
}

.cmd-name {
  font-weight: 600;
}

.command-detail {
  flex: 1;
  min-width: 0;
}

.command-detail h4 {
  margin: 0 0 0.25rem;
  font-size: 1.05rem;
}

.cmd-token-inline {
  font-size: 0.85rem;
  color: #555;
  margin-left: 0.5rem;
}

.summary,
.description {
  margin: 0.15rem 0;
  font-size: 0.88rem;
  color: #444;
}

.examples,
.constraints {
  font-size: 0.85rem;
  margin: 0.5rem 0;
}

.badge {
  font-size: 0.72rem;
  padding: 0.1rem 0.45rem;
  border-radius: 999px;
  background: #eee;
  color: #444;
}

.hint {
  color: #777;
  font-size: 0.85rem;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.85rem;
  margin-top: 0.5rem;
}

th,
td {
  text-align: left;
  padding: 0.35rem 0.5rem;
  border-bottom: 1px solid #e5e5e5;
}

th {
  color: #555;
  font-weight: 600;
}

tbody tr {
  cursor: pointer;
}

tbody tr:hover {
  background: #f6f6f6;
}

tbody tr.selected {
  background: #e4ecfb;
}
</style>
