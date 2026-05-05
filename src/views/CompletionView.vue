<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSessionStore } from "../stores/session";
import AnimatedList from "../components/AnimatedList.vue";
import ToastMessage from "../components/ToastMessage.vue";
import type { CourseSummary, CompletionResult } from "../types/login";

const sessionStore = useSessionStore();
const completing = ref(false);
const completionCourseIds = ref<Set<string>>(new Set());
const toastVisible = ref(false);
const toastMessage = ref("");
const toastType = ref<"success" | "error">("success");

const sortedCourses = computed(() => sessionStore.sortedCourses);
const allCoursesSelected = computed(
  () =>
    sortedCourses.value.length > 0 &&
    completionCourseIds.value.size === sortedCourses.value.length,
);

const toggleCourse = (clazzCourseId: string) => {
  const next = new Set(completionCourseIds.value);
  if (next.has(clazzCourseId)) {
    next.delete(clazzCourseId);
  } else {
    next.add(clazzCourseId);
  }
  completionCourseIds.value = next;
};

const onAnimatedItemSelected = (item: { id: string }) => {
  toggleCourse(item.id);
};

const toggleAllCourses = () => {
  if (allCoursesSelected.value) {
    completionCourseIds.value = new Set();
  } else {
    completionCourseIds.value = new Set(
      sortedCourses.value.map((c: CourseSummary) => c.clazzCourseId),
    );
  }
};

const handleComplete = async () => {
  completing.value = true;
  try {
    const courseIds = Array.from(completionCourseIds.value);
    let overall = { total: 0, completed: 0, failed: [] as string[] };

    for (const ccid of courseIds) {
      const result = await invoke<CompletionResult>("complete_course_resources", { ccid });
      overall.total += result.total;
      overall.completed += result.completed;
      overall.failed.push(...result.failed);
    }

    if (overall.total === 0) {
      toastType.value = "success";
      toastMessage.value = "完成已选资源！";
    } else if (overall.completed === overall.total) {
      toastType.value = "success";
      toastMessage.value = "成功完成!";
    } else if (overall.completed > 0) {
      toastType.value = "success";
      toastMessage.value = `完成 ${overall.completed}/${overall.total} 个资源`;
    } else {
      toastType.value = "error";
      toastMessage.value = "未成功，请重试！";
    }

    toastVisible.value = true;

    await sessionStore.refreshDashboard();
  } catch (error) {
    toastType.value = "error";
    toastMessage.value = `操作失败: ${error}`;
    toastVisible.value = true;
  } finally {
    completing.value = false;
    completionCourseIds.value = new Set();
  }
};
</script>

<template>
  <div class="completion-wrapper">
    <div class="completion-layout">
      <header class="completion-header">
        <p class="eyebrow">资源完成</p>
        <p class="completion-desc">选择要处理的课程，一键标记资源完成。</p>
      </header>
      <div class="completion-body">
        <div class="course-select-panel">
          <div class="course-select-header">
            <h3 class="course-select-title">选择课程</h3>
            <label class="course-select-all">
              <input
                type="checkbox"
                :checked="allCoursesSelected"
                :indeterminate="
                  completionCourseIds.size > 0 && !allCoursesSelected
                "
                @change="toggleAllCourses"
              />
              <span>全选</span>
            </label>
            <span class="course-count-badge"
              >{{ completionCourseIds.size }} /
              {{ sortedCourses.length }}</span
            >
          </div>
          <div class="course-select-body">
            <AnimatedList
              v-if="sortedCourses.length"
              :items="
                sortedCourses.map((c: CourseSummary) => ({
                  id: c.clazzCourseId,
                  label: c.courseName,
                  subLabel: c.teacherName,
                }))
              "
              :selected-ids="completionCourseIds"
              :show-gradients="true"
              :enable-arrow-navigation="true"
              @item-selected="onAnimatedItemSelected"
            />
            <p v-else class="completion-empty">暂无可选课程</p>
          </div>
        </div>
        <div class="completion-action-panel">
          <button
            class="btn btn-primary btn-complete"
            :disabled="completionCourseIds.size === 0 || completing"
            @click="handleComplete"
          >
            <svg
              :class="{ spinning: completing }"
              width="18"
              height="18"
              viewBox="0 0 18 18"
              fill="none"
              style="display: block"
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
            {{ completing ? "处理中..." : "一键完成资源" }}
          </button>
        </div>
      </div>
    </div>

    <ToastMessage
      :visible="toastVisible"
      :message="toastMessage"
      :type="toastType"
      @close="toastVisible = false"
    />
  </div>
</template>

<style scoped>
.completion-layout {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.completion-header {
  padding: 0;
}

.completion-desc {
  font-size: 0.875rem;
  color: var(--text-2);
  margin-top: 4px;
  line-height: 1.5;
}

.eyebrow {
  display: inline-block;
  margin: 0 0 8px;
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.14em;
}

.completion-body {
  display: grid;
  grid-template-columns: 1fr 280px;
  gap: 24px;
  align-items: start;
}

.course-select-panel {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  overflow: hidden;
}

.course-select-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
}

.course-select-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text);
  flex: 1;
}

.course-select-all {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-2);
  cursor: pointer;
  user-select: none;
}

.course-select-all input[type="checkbox"] {
  accent-color: var(--accent);
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.course-count-badge {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-3);
  background: rgba(var(--text-rgb), 0.06);
  padding: 2px 10px;
  border-radius: 999px;
}

.course-select-body {
  position: relative;
  min-height: 200px;
}

.completion-empty {
  padding: 40px 20px;
  text-align: center;
  font-size: 0.875rem;
  color: var(--text-3);
}

.completion-action-panel {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  position: sticky;
  top: 0;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  min-height: 36px;
  padding: 0 16px;
  border: none;
  border-radius: 9999px;
  font-family: var(--font-ui);
  font-size: 0.875rem;
  font-weight: 500;
  line-height: 1;
  cursor: pointer;
  user-select: none;
  white-space: nowrap;
  transition:
    background 0.18s var(--ease),
    border-color 0.12s var(--ease),
    transform 0.22s var(--ease),
    box-shadow 0.22s var(--ease);
}

.btn:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}

.btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  transform: none !important;
}

.btn-primary {
  background: var(--accent-dim);
  color: #ffffff;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover-dim);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(var(--accent-rgb), 0.12);
}

.btn-complete {
  width: 100%;
  min-height: 44px;
  gap: 8px;
  font-size: 0.9375rem;
  font-weight: 600;
}

.spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 900px) {
  .completion-body {
    grid-template-columns: 1fr;
  }

  .completion-action-panel {
    position: static;
  }
}
</style>
