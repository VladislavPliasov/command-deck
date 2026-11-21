<script setup lang="ts">
import type { ParameterSpec } from "../../types/tool";

const props = defineProps<{ param: ParameterSpec }>();

const kindLabels: Record<string, string> = {
  flag: "Boolean flag",
  option: "Option with value",
  positional: "Positional argument",
  custom: "Custom (structure not modeled)",
};
</script>

<template>
  <section class="param-detail">
    <h4>{{ props.param.label }}</h4>
    <p class="id-line">
      <code>{{ props.param.id }}</code>
      <span class="badge">{{ kindLabels[props.param.kind] ?? props.param.kind }}</span>
      <span v-if="props.param.required" class="badge required">required</span>
      <span v-if="props.param.repeatable" class="badge">repeatable</span>
    </p>

    <dl>
      <dt>Flag / token</dt>
      <dd>
        <code v-if="props.param.token">{{ props.param.token }}</code>
        <span v-else>—</span>
        <template v-if="props.param.aliases?.length">
          (aliases: <code v-for="a in props.param.aliases" :key="a">{{ a }}</code
          >)
        </template>
      </dd>
      <dt>Description</dt>
      <dd>{{ props.param.description || "—" }}</dd>
      <dt>Syntax</dt>
      <dd>
        <code v-if="props.param.syntax">{{ props.param.syntax }}</code>
        <span v-else>—</span>
      </dd>
      <dt>Expected value type</dt>
      <dd>
        {{ props.param.value ? props.param.value.valueType : "none (presence flag)" }}
        <template v-if="props.param.value?.optional"> (value may be omitted)</template>
        <template v-if="props.param.value?.multiple">
          (one occurrence accepts multiple values)</template>
      </dd>
      <dt>Accepted values</dt>
      <dd>
        <template v-if="props.param.value?.choices?.length">
          <code v-for="c in props.param.value.choices" :key="c" class="choice">{{ c }}</code>
        </template>
        <span v-else>—</span>
      </dd>
      <dt>Default value</dt>
      <dd>
        <code v-if="props.param.value?.default">{{ props.param.value.default }}</code>
        <span v-else>—</span>
      </dd>
      <dt>Requirements</dt>
      <dd>
        <template v-if="props.param.requires.length">
          requires
          <code v-for="r in props.param.requires" :key="r" class="choice">{{ r }}</code>
        </template>
        <span v-else>—</span>
      </dd>
      <dt>Conflicts</dt>
      <dd>
        <template v-if="props.param.conflicts.length">
          conflicts with
          <code v-for="c in props.param.conflicts" :key="c" class="choice">{{ c }}</code>
        </template>
        <span v-else>—</span>
      </dd>
      <dt>Examples</dt>
      <dd>
        <template v-if="props.param.examples.length">
          <div v-for="e in props.param.examples" :key="e"><code>{{ e }}</code></div>
        </template>
        <span v-else>—</span>
      </dd>
      <dt>Documented version constraints (syntax not validated)</dt>
      <dd>
        <template v-if="props.param.versionConstraints.length">
          <div v-for="v in props.param.versionConstraints" :key="v">{{ v }}</div>
        </template>
        <span v-else>—</span>
      </dd>
    </dl>
  </section>
</template>

<style scoped>
.param-detail {
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 0.75rem 1rem;
  margin-top: 0.75rem;
}

h4 {
  margin: 0 0 0.25rem;
  font-size: 1rem;
}

.id-line {
  display: flex;
  gap: 0.4rem;
  align-items: center;
  margin: 0 0 0.5rem;
  color: #555;
  font-size: 0.82rem;
}

.badge {
  font-size: 0.72rem;
  padding: 0.1rem 0.45rem;
  border-radius: 999px;
  background: #eee;
  color: #444;
}

.badge.required {
  background: #fde8c8;
  color: #7a4c00;
}

dl {
  display: grid;
  grid-template-columns: max-content 1fr;
  gap: 0.3rem 1.15rem;
  margin: 0;
  font-size: 0.86rem;
}

dt {
  font-weight: 600;
  color: #444;
}

dd {
  margin: 0;
  word-break: break-word;
}

.choice {
  margin-right: 0.45rem;
  background: #f2f2f2;
  padding: 0.05rem 0.35rem;
  border-radius: 3px;
}
</style>
