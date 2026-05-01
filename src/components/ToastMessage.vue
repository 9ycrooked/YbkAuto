<script setup lang="ts">
import { ref, watch, onUnmounted } from "vue";

const props = defineProps<{
  message: string;
  type?: "error" | "success" | "info";
  visible: boolean;
  duration?: number;
}>();

const emit = defineEmits<{
  close: [];
}>();

const isShown = ref(false);
let timer: ReturnType<typeof setTimeout> | null = null;

watch(
  () => props.visible,
  (val) => {
    if (val) {
      isShown.value = true;
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => {
        isShown.value = false;
        emit("close");
      }, props.duration ?? 4000);
    } else {
      isShown.value = false;
      if (timer) clearTimeout(timer);
    }
  },
);

onUnmounted(() => {
  if (timer) clearTimeout(timer);
});
</script>

<template>
  <Teleport to="body">
    <Transition name="toast">
      <div
        v-if="isShown"
        :class="['toast', `toast--${type ?? 'error'}`]"
        role="alert"
        aria-live="assertive"
      >
        <span class="toast__icon">
          <svg v-if="type === 'success'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>
          <svg v-else-if="type === 'info'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </span>
        <span class="toast__message">{{ message }}</span>
        <button class="toast__close" @click="isShown = false; emit('close')" aria-label="关闭">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.toast {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 9999;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  max-width: min(380px, calc(100vw - 32px));
  min-height: 44px;
  padding: 0 12px;
  border-radius: 12px;
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: 0 2px 12px rgba(var(--shadow-color), calc(var(--shadow-strength) * 2));
}

.toast__icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.toast__message {
  flex: 1;
  font-size: 0.875rem;
  font-weight: 500;
  line-height: 1.4;
  color: var(--text);
}

.toast__close {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-4);
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}

.toast__close:hover {
  background: var(--surface-hover);
  color: var(--text-2);
}

.toast--error .toast__icon {
  color: var(--error);
}

.toast--success .toast__icon {
  color: var(--success);
}

.toast--info .toast__icon {
  color: var(--accent);
}

.toast-enter-active {
  animation: toastIn 0.22s var(--ease-cinema) forwards;
}

.toast-leave-active {
  animation: toastOut 0.18s var(--ease) forwards;
}

@keyframes toastIn {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}

@keyframes toastOut {
  from {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
  to {
    opacity: 0;
    transform: translateX(-50%) translateY(-6px);
  }
}

@media (prefers-reduced-motion: reduce) {
  .toast-enter-active,
  .toast-leave-active {
    animation-duration: 0.01ms;
  }
}
</style>
