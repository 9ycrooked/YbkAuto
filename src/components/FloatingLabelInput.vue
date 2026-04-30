<script setup lang="ts">
const props = defineProps<{
  label: string
  type?: string
}>()

const modelValue = defineModel<string>({ default: "" })

const labelChars = props.label.split('')
</script>

<template>
  <div class="field">
    <input
      :type="type || 'text'"
      required
      v-model="modelValue"
      autocomplete="off"
    >
    <label>
      <span
        v-for="(char, index) in labelChars"
        :key="index"
        :style="{ transitionDelay: `${index * 60}ms` }"
      >
        {{ char }}
      </span>
    </label>
  </div>
</template>

<style scoped>
.field {
  position: relative;
  margin: 14px 0 28px;
  width: 100%;
}

.field input {
  background-color: transparent;
  border: none;
  border-bottom: 2px var(--border) solid;
  display: block;
  width: 100%;
  padding: 10px 0;
  font-size: 16px;
  color: var(--text);
}

.field input:focus {
  outline: none;
  border-bottom-color: var(--accent);
}

.field label {
  position: absolute;
  top: 10px;
  left: 0;
  pointer-events: none;
}

.field label span {
  display: inline-block;
  font-size: 15px;
  font-weight: 500;
  min-width: 5px;
  color: var(--text-3);
  transition: 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.field input:focus + label span,
.field input:valid + label span {
  color: var(--accent);
  transform: translateY(-26px);
}
</style>