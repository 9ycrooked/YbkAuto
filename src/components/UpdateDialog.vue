<script setup lang="ts">
defineProps<{
  open: boolean;
  forced: boolean;
  downloading: boolean;
  error: string;
  policyMessage?: string | null;
  currentVersion?: string;
  nextVersion?: string;
  notes: string;
  progressPercent: number;
  hasTotalBytes: boolean;
}>();

defineEmits<{
  dismiss: [];
  install: [];
  closeApp: [];
}>();
</script>

<template>
  <div v-if="open" class="update-backdrop">
    <section class="update-modal" role="dialog" aria-modal="true" aria-labelledby="update-modal-title">
      <div class="update-modal__header">
        <div>
          <p class="eyebrow">软件更新</p>
          <h2 id="update-modal-title">发现新版本</h2>
        </div>
        <button
          v-if="!forced"
          class="update-close"
          type="button"
          aria-label="关闭更新提示"
          :disabled="downloading"
          @click="$emit('dismiss')"
        >
          ×
        </button>
      </div>

      <p v-if="policyMessage" class="update-policy-message">{{ policyMessage }}</p>

      <div class="update-version-row">
        <span>当前版本 {{ currentVersion || "未知" }}</span>
        <span>新版本 {{ nextVersion || "未知" }}</span>
      </div>

      <pre class="update-notes">{{ notes }}</pre>

      <div v-if="downloading" class="update-progress">
        <div class="update-progress-bar" aria-hidden="true">
          <div
            class="update-progress-fill"
            :class="{ indeterminate: !hasTotalBytes }"
            :style="{ width: hasTotalBytes ? `${progressPercent}%` : '35%' }"
          ></div>
        </div>
        <p>{{ hasTotalBytes ? `${progressPercent}%` : "正在下载更新..." }}</p>
      </div>

      <p v-if="error" class="update-error-message">{{ error }}</p>

      <div class="update-actions">
        <button
          v-if="!forced"
          class="btn-secondary"
          type="button"
          :disabled="downloading"
          @click="$emit('dismiss')"
        >
          稍后
        </button>
        <button
          v-else
          class="btn-secondary"
          type="button"
          :disabled="downloading"
          @click="$emit('closeApp')"
        >
          退出
        </button>
        <button class="btn-primary" type="button" :disabled="downloading" @click="$emit('install')">
          {{ downloading ? "正在更新" : "立即更新" }}
        </button>
      </div>
    </section>
  </div>
</template>

<style scoped>
.update-backdrop {
  position: fixed;
  inset: 0;
  z-index: 100000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: rgba(0, 0, 0, 0.46);
}

.update-modal {
  width: min(520px, 100%);
  max-height: min(680px, calc(100vh - 48px));
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.18);
}

.update-modal__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.eyebrow {
  display: inline-block;
  margin: 0 0 8px;
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.14em;
}

.update-modal h2 {
  color: var(--text);
  font-size: 1.25rem;
  font-weight: 700;
  line-height: 1.3;
}

.update-close {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-3);
  font-size: 1.375rem;
  line-height: 1;
  cursor: pointer;
}

.update-close:hover:not(:disabled) {
  background: var(--surface-hover);
  color: var(--text);
}

.update-policy-message {
  color: var(--text-2);
  font-size: 0.875rem;
  line-height: 1.6;
}

.update-version-row {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 14px;
  border-radius: 12px;
  background: rgba(var(--text-rgb), 0.05);
  color: var(--text-2);
  font-size: 0.8125rem;
  font-weight: 600;
}

.update-notes {
  min-height: 120px;
  max-height: 240px;
  overflow: auto;
  margin: 0;
  padding: 14px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: var(--surface-raised);
  color: var(--text);
  font-family: var(--font-ui);
  font-size: 0.875rem;
  line-height: 1.6;
  white-space: pre-wrap;
}

.update-progress {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.update-progress p {
  color: var(--text-2);
  font-size: 0.8125rem;
}

.update-progress-bar {
  height: 8px;
  overflow: hidden;
  border-radius: 999px;
  background: rgba(var(--text-rgb), 0.08);
}

.update-progress-fill {
  height: 100%;
  border-radius: inherit;
  background: var(--accent-dim);
  transition: width 0.2s var(--ease);
}

.update-progress-fill.indeterminate {
  animation: progressPulse 1.2s ease-in-out infinite alternate;
}

@keyframes progressPulse {
  from {
    transform: translateX(-20%);
  }
  to {
    transform: translateX(220%);
  }
}

.update-error-message {
  padding: 10px 12px;
  border: 1px solid rgba(var(--error-rgb), 0.22);
  border-radius: 10px;
  background: rgba(var(--error-rgb), 0.08);
  color: var(--error);
  font-size: 0.8125rem;
  line-height: 1.5;
}

.update-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.btn-primary,
.btn-secondary {
  min-height: 40px;
  padding: 0 18px;
  border: none;
  border-radius: 9999px;
  font-family: var(--font-ui);
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
}

.btn-primary {
  background: var(--accent-dim);
  color: #ffffff;
}

.btn-secondary {
  background: rgba(var(--text-rgb), 0.06);
  color: var(--text-2);
}

.btn-primary:disabled,
.btn-secondary:disabled,
.update-close:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

@media (max-width: 640px) {
  .update-version-row,
  .update-actions {
    flex-direction: column;
  }

  .btn-primary,
  .btn-secondary {
    width: 100%;
  }
}
</style>

