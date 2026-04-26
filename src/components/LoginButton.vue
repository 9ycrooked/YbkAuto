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
  <button
    class="btn"
    type="submit"
    :disabled="disabled || loading"
    @click="$emit('click')"
  >
    <span class="btn-text">{{ loading ? "登录中..." : label ?? "登录" }}</span>
    <span class="btn-icon" aria-hidden="true">></span>
  </button>
</template>

<style scoped>
.btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  width: 160px;
  min-height: 46px;
  padding: 0 12px 0 18px;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, #5661f6, #7b61ff);
  color: #fff;
  cursor: pointer;
  box-shadow: 0 12px 28px rgba(86, 97, 246, 0.35);
  transition:
    transform 0.18s ease,
    box-shadow 0.18s ease,
    opacity 0.18s ease;
}

.btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 16px 32px rgba(86, 97, 246, 0.42);
}

.btn:active:not(:disabled) {
  transform: translateY(0);
}

.btn:disabled {
  cursor: not-allowed;
  opacity: 0.72;
  box-shadow: none;
}

.btn-text {
  font-size: 14px;
  font-weight: 600;
}

.btn-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.18);
  font-size: 15px;
  font-weight: 700;
}
</style>
