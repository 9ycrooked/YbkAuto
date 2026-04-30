<script setup lang="ts">
defineProps<{
  label?: string;
  loading?: boolean;
  disabled?: boolean;
}>();

defineEmits<{
  click: [];
}>();
</script>

<template>
  <button class="btn" type="submit" :disabled="disabled || loading">
    <span class="btn-text">{{ loading ? "登录中..." : label ?? "登录" }}</span>
    <span class="btn-icon" aria-hidden="true">
      <svg v-if="!loading" width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M3 8h10M9 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <svg v-else class="spinner" width="16" height="16" viewBox="0 0 16 16" fill="none">
        <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="2" stroke-dasharray="32" stroke-linecap="round"/>
      </svg>
    </span>
  </button>
</template>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  min-height: 44px;
  padding: 0 14px;
  border: none;
  border-radius: 12px;
  background: var(--accent-dim);
  color: #ffffff;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition:
    transform 0.15s var(--ease),
    background 0.15s var(--ease),
    box-shadow 0.15s var(--ease),
    opacity 0.15s var(--ease);
  box-shadow: 0 4px 16px rgba(var(--accent-rgb), 0.3);
}

.btn:hover:not(:disabled) {
  background: var(--accent-hover-dim);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(var(--accent-rgb), 0.35);
}

.btn:active:not(:disabled) {
  transform: translateY(0);
}

.btn:disabled {
  cursor: not-allowed;
  opacity: 0.55;
  box-shadow: none;
}

.btn-text {
  flex: 1;
  text-align: center;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border-radius: 7px;
  background: rgba(255, 255, 255, 0.15);
  flex-shrink: 0;
}

.btn-icon .spinner {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>