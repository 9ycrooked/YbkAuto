<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { useSessionStore } from "../stores/session";

const sessionStore = useSessionStore();
const cardsRevealed = ref(false);

const courses = computed(() => sessionStore.courses);

let revealFallback: ReturnType<typeof setTimeout> | null = null;

const triggerCardReveal = () => {
  cardsRevealed.value = false;
  if (revealFallback) clearTimeout(revealFallback);
  nextTick(() => {
    requestAnimationFrame(() => {
      cardsRevealed.value = true;
    });
    revealFallback = setTimeout(() => {
      if (!cardsRevealed.value) cardsRevealed.value = true;
    }, 120);
  });
};

const handleSpotlightMove = (e: MouseEvent) => {
  const card = e.currentTarget as HTMLDivElement;
  const rect = card.getBoundingClientRect();
  card.style.setProperty("--mx", `${((e.clientX - rect.left) / rect.width) * 100}%`);
  card.style.setProperty("--my", `${((e.clientY - rect.top) / rect.height) * 100}%`);
};

watch(
  courses,
  () => {
    if (courses.value.length > 0) triggerCardReveal();
  },
  { immediate: true },
);

const onRefresh = () => {
  sessionStore.refreshDashboard().then(() => {
    triggerCardReveal();
  });
};
</script>

<template>
  <div class="dashboard-layout">
    <header class="dashboard-header">
      <p class="eyebrow">课程概览</p>
    </header>
    <p v-if="sessionStore.dashboardError" class="banner banner--error">
      {{ sessionStore.dashboardError }}
    </p>

    <div class="summary-strip">
      <div class="summary-item summary-item--has-action">
        <div class="summary-item-row">
          <span class="summary-label">课程数量</span>
        </div>
        <span class="summary-value">{{ courses.length }}</span>
        <button
          class="btn-icon magnetic-btn summary-action-btn"
          :disabled="sessionStore.isRefreshing"
          :title="sessionStore.isRefreshing ? '刷新中...' : '刷新数据'"
          @click="onRefresh"
        >
          <svg
            :class="['refresh-icon', { spinning: sessionStore.isRefreshing }]"
            width="18"
            height="18"
            viewBox="0 0 18 18"
            fill="none"
          >
            <path
              d="M14.5 9a5.5 5.5 0 01-10.4 2.5M3.5 9a5.5 5.5 0 0110.4-2.5"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
            />
            <path
              d="M14.5 3.5v3h-3M3.5 14.5v-3h3"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </button>
      </div>
      <div class="summary-item">
        <div class="summary-item-row">
          <span class="summary-label">资源状态</span>
        </div>
        <span class="summary-value"
          >已完成 {{ sessionStore.totalCompletedResources }} /
          未完成 {{ sessionStore.totalIncompleteResources }}</span
        >
      </div>
    </div>

    <div v-if="courses.length" class="course-grid">
      <article
        v-for="(course, index) in courses"
        :key="course.clazzCourseId"
        :class="['course-card', 'spotlight-card', { 'in-view': cardsRevealed }]"
        :style="{ transitionDelay: cardsRevealed ? `${Math.min(index * 0.06, 0.42)}s` : '0s' }"
        @mousemove="handleSpotlightMove"
      >
        <div class="course-card__top">
          <div>
            <p class="course-class">{{ course.className ?? "" }}</p>
            <h2>{{ course.courseName }}</h2>
            <p class="course-teacher">{{ course.teacherName }}</p>
          </div>
          <span class="course-state">{{ course.courseStatus }}</span>
        </div>
        <div class="course-card__body">
          <div class="info-row">
            <span class="info-label">资源状态</span>
            <span class="resource-pill"
              >已完成 {{ course.resourceState.completed }} /
              未完成 {{ course.resourceState.incomplete }}</span>
          </div>
        </div>
      </article>
    </div>
    <section v-else class="state-card state-card--compact">
      <h2>还没有课程数据</h2>
      <p class="state-copy">
        当前账号没有返回课程列表，或者数据暂时还没同步出来。
      </p>
    </section>
  </div>
</template>

<style scoped>
.dashboard-layout {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.dashboard-header {
  padding: 0;
}

.eyebrow {
  display: inline-block;
  margin: 0 0 8px;
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.14em;
}

.summary-strip {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 2px;
  background: var(--border);
  border-radius: 16px;
  overflow: hidden;
  border: 1px solid var(--border);
  opacity: 0;
  transform: translateY(-8px);
  animation: summaryIn 0.6s 0.15s var(--ease-cinema) forwards;
}

@keyframes summaryIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.summary-item {
  background: var(--surface);
  padding: 18px 22px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.summary-item--has-action {
  position: relative;
}

.summary-action-btn {
  position: absolute;
  top: 18px;
  right: 22px;
}

.summary-item-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.summary-label {
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.summary-value {
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
  line-height: 1.2;
}

.banner {
  padding: 10px 14px;
  border-radius: 12px;
  font-size: 0.875rem;
}

.banner--error {
  background: rgba(var(--error-rgb), 0.1);
  border: 1px solid rgba(var(--error-rgb), 0.25);
  color: var(--error);
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-3);
  cursor: pointer;
  transition: all 0.2s var(--ease);
  flex-shrink: 0;
}

.btn-icon:hover:not(:disabled) {
  background: var(--surface-hover);
  color: var(--accent);
}

.magnetic-btn {
  transition:
    background 0.2s var(--ease),
    color 0.2s var(--ease),
    transform 0.3s var(--ease-cinema);
}

.magnetic-btn:hover:not(:disabled) {
  transform: scale(1.15);
}

.refresh-icon {
  display: block;
}

.refresh-icon.spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.course-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.course-card {
  --mx: 50%;
  --my: 50%;
  position: relative;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
  opacity: 0;
  transform: translateY(16px);
  transition:
    opacity 0.6s var(--ease-cinema),
    transform 0.6s var(--ease-cinema),
    box-shadow 0.25s var(--ease),
    border-color 0.2s var(--ease);
  overflow: hidden;
}

.course-card::before {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  opacity: 0;
  background: radial-gradient(
    600px circle at var(--mx) var(--my),
    rgba(var(--accent-rgb), 0.08) 0%,
    transparent 60%
  );
  transition: opacity 0.3s var(--ease);
  pointer-events: none;
  z-index: 0;
}

.course-card:hover::before {
  opacity: 1;
}

.course-card > * {
  position: relative;
  z-index: 1;
}

.course-card.in-view {
  opacity: 1;
  transform: translateY(0);
}

.course-card:hover {
  transform: translateY(-3px);
  border-color: var(--border-strong);
  box-shadow:
    0 8px 24px rgba(var(--shadow-color), calc(var(--shadow-strength) * 2)),
    0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}

.course-card__top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 18px;
}

.course-class {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-3);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  margin-bottom: 4px;
}

.course-card__top h2 {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text);
  line-height: 1.3;
  letter-spacing: -0.01em;
}

.course-teacher {
  margin-top: 6px;
  font-size: 0.8125rem;
  color: var(--text-2);
}

.course-state {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 26px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1;
  background: var(--accent-soft);
  color: var(--accent);
  flex-shrink: 0;
}

.course-card__body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.info-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-2);
  flex-shrink: 0;
}

.checkin-note {
  font-size: 0.8125rem;
  color: var(--text-2);
  line-height: 1.5;
}

.muted {
  color: var(--text-3) !important;
}

.status-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 24px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1;
  letter-spacing: 0.01em;
  flex-shrink: 0;
}

.status-chip--success {
  background: rgba(var(--success-rgb), 0.12);
  color: var(--success);
}

.status-chip--warning {
  background: var(--accent-soft);
  color: var(--accent);
}

.status-chip--muted {
  background: rgba(var(--text-rgb), 0.07);
  color: var(--text-2);
}

.resource-pill {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 28px;
  padding: 0 12px;
  border-radius: 999px;
  font-size: 0.8125rem;
  font-weight: 500;
  background: rgba(var(--text-rgb), 0.06);
  color: var(--text-2);
}

.state-card {
  width: min(460px, 100%);
  margin: 80px auto 0;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 28px;
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}

.state-card--compact {
  width: 100%;
  margin: 0;
}

.state-card h2 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text);
}

.state-copy {
  margin-top: 8px;
  color: var(--text-2);
}

@media (max-width: 900px) {
  .summary-strip {
    grid-template-columns: 1fr;
  }

  .course-card__top,
  .info-row {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
