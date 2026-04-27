<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

import AnimatedList from "./components/AnimatedList.vue";
import LoginForm from "./components/LoginForm.vue";
import type { CourseSummary, DashboardState, SessionState } from "./types/login";

const createGuestSession = (rememberedUsername = ""): SessionState => ({
  authenticated: false,
  rememberedUsername,
  user: null,
  dashboard: null,
});

const session = ref<SessionState>(createGuestSession());
const isBootstrapping = ref(true);
const isRefreshing = ref(false);
const bootstrapError = ref("");
const dashboardError = ref("");
const cardsRevealed = ref(false);
const currentView = ref<"dashboard" | "completion" | "profile">("dashboard");
const selectedSetting = ref<string>("about");
const completionCourseIds = ref<Set<string>>(new Set());
const completing = ref(false);

const courses = computed(() => session.value.dashboard?.courses ?? []);
const openCheckinCount = computed(
  () => courses.value.filter((course) => course.checkinState === "open").length,
);
const totalCompletedResources = computed(() =>
  courses.value.reduce(
    (count, course) => count + course.resourceState.completed,
    0,
  ),
);
const totalIncompleteResources = computed(() =>
  courses.value.reduce(
    (count, course) => count + course.resourceState.incomplete,
    0,
  ),
);
const allCoursesSelected = computed(() =>
  courses.value.length > 0 && completionCourseIds.value.size === courses.value.length,
);
const sortedCourses = computed(() =>
  [...courses.value].sort((a, b) => b.createTime.localeCompare(a.createTime)),
);

const userInitials = computed(() => {
  const name = session.value.user?.fullName ?? "";
  if (!name) return "?";
  const parts = name.trim().split(/\s+/);
  if (parts.length >= 2) {
    return (parts[0][0] + parts[1][0]).toUpperCase();
  }
  return name.slice(0, 1).toUpperCase();
});

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
    completionCourseIds.value = new Set(courses.value.map((c) => c.clazzCourseId));
  }
};

const spotlightRef = ref<HTMLElement | null>(null);
const spotlightPos = ref({ x: 50, y: 50 });

const onSpotlightMove = (e: MouseEvent) => {
  const el = spotlightRef.value;
  if (!el) return;
  const rect = el.getBoundingClientRect();
  const x = ((e.clientX - rect.left) / rect.width) * 100;
  const y = ((e.clientY - rect.top) / rect.height) * 100;
  spotlightPos.value = { x, y };
};

const triggerCardReveal = () => {
  cardsRevealed.value = false;
  nextTick(() => {
    requestAnimationFrame(() => {
      cardsRevealed.value = true;
    });
  });
};

const bootstrapSession = async () => {
  isBootstrapping.value = true;
  bootstrapError.value = "";

  try {
    session.value = await invoke<SessionState>("bootstrap_session");
  } catch (error) {
    bootstrapError.value =
      typeof error === "string" ? error : "初始化失败，请稍后再试";
    session.value = createGuestSession(session.value.rememberedUsername);
  } finally {
    isBootstrapping.value = false;
  }
};

const handleLoginSuccess = (nextSession: SessionState) => {
  bootstrapError.value = "";
  dashboardError.value = "";
  session.value = nextSession;
  triggerCardReveal();
};

const refreshDashboard = async () => {
  if (isRefreshing.value) return;
  dashboardError.value = "";
  isRefreshing.value = true;
  try {
    const dashboard = await invoke<DashboardState>("refresh_dashboard");
    session.value = { ...session.value, authenticated: true, dashboard };
    triggerCardReveal();
  } catch (error) {
    dashboardError.value =
      typeof error === "string" ? error : "刷新失败，请稍后再试";
  } finally {
    isRefreshing.value = false;
  }
};

const logout = async () => {
  const rememberedUsername = session.value.rememberedUsername;
  try {
    await invoke("logout_command");
  } catch (error) {
    dashboardError.value =
      typeof error === "string" ? error : "退出登录失败，请稍后再试";
    return;
  }
  dashboardError.value = "";
  session.value = createGuestSession(rememberedUsername);
};

const checkinLabel = (course: CourseSummary) => {
  switch (course.checkinState) {
    case "open": return "进行中";
    case "closed": return "未开启";
    default: return "获取失败";
  }
};

const checkinClass = (course: CourseSummary) => ({
  "status-chip": true,
  "status-chip--success": course.checkinState === "open",
  "status-chip--warning": course.checkinState === "closed",
  "status-chip--muted": course.checkinState === "error",
});

watch(courses, () => {
  if (courses.value.length > 0) triggerCardReveal();
});

onMounted(() => {
  void bootstrapSession();
});
</script>

<template>
  <main class="app-shell">
    <section v-if="isBootstrapping" class="state-card">
      <p class="eyebrow brand-gradient">YbkAuto</p>
      <h1>正在恢复登录状态</h1>
      <p class="state-copy">稍等一下，我们在帮你检查本地会话。</p>
    </section>

    <section v-else-if="!session.authenticated" class="auth-layout">
      <div class="aurora-bg" aria-hidden="true"></div>
      <div class="auth-copy">
        <p class="eyebrow brand-gradient">YbkAuto</p>
        <h1>登录云班课账号</h1>
        <p class="auth-desc">首版会记住账号和登录令牌。下次打开时会先尝试自动恢复会话。</p>
      </div>
      <div class="auth-card">
        <p v-if="bootstrapError" class="banner banner--error">{{ bootstrapError }}</p>
        <LoginForm :remembered-username="session.rememberedUsername" @login-success="handleLoginSuccess" />
      </div>
    </section>

    <section v-else class="dashboard-layout">
      <nav class="view-tabs">
        <button :class="['view-tab', { active: currentView === 'dashboard' }]" @click="currentView = 'dashboard'">课程概览</button>
        <button :class="['view-tab', { active: currentView === 'completion' }]" @click="currentView = 'completion'">资源完成</button>
        <button :class="['view-tab', { active: currentView === 'profile' }]" @click="currentView = 'profile'">个人中心</button>
      </nav>

      <template v-if="currentView === 'dashboard'">
        <header class="dashboard-header"><p class="eyebrow">课程概览</p></header>
        <p v-if="dashboardError" class="banner banner--error">{{ dashboardError }}</p>

        <div class="summary-strip">
          <div class="summary-item summary-item--has-action">
            <div class="summary-item-row"><span class="summary-label">课程数量</span></div>
            <span class="summary-value">{{ courses.length }}</span>
            <button class="btn-icon magnetic-btn summary-action-btn" :disabled="isRefreshing" :title="isRefreshing ? '刷新中...' : '刷新数据'" @click="refreshDashboard">
              <svg :class="['refresh-icon', { spinning: isRefreshing }]" width="18" height="18" viewBox="0 0 18 18" fill="none">
                <path d="M14.5 9a5.5 5.5 0 01-10.4 2.5M3.5 9a5.5 5.5 0 0110.4-2.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                <path d="M14.5 3.5v3h-3M3.5 14.5v-3h3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
          <div class="summary-item">
            <div class="summary-item-row"><span class="summary-label">开放签到</span></div>
            <span class="summary-value">{{ openCheckinCount }}</span>
          </div>
          <div class="summary-item">
            <div class="summary-item-row"><span class="summary-label">资源状态</span></div>
            <span class="summary-value">已完成 {{ totalCompletedResources }} / 未完成 {{ totalIncompleteResources }}</span>
          </div>
        </div>

        <div v-if="courses.length" class="course-grid">
          <article
            v-for="(course, index) in courses"
            :key="course.clazzCourseId"
            :class="['course-card', 'spotlight-card', { 'in-view': cardsRevealed }]"
            :style="{
              transitionDelay: cardsRevealed ? `${Math.min(index * 0.06, 0.42)}s` : '0s',
              '--mx': spotlightPos.x + '%',
              '--my': spotlightPos.y + '%',
            }"
            @mousemove="onSpotlightMove"
          >
            <div class="course-card__top">
              <div>
                <p class="course-class">{{ course.className }}</p>
                <h2>{{ course.courseName }}</h2>
                <p class="course-teacher">{{ course.teacherName }}</p>
              </div>
              <span class="course-state">{{ course.courseStatus }}</span>
            </div>
            <div class="course-card__body">
              <div class="info-row">
                <span class="info-label">签到状态</span>
                <span :class="checkinClass(course)">{{ checkinLabel(course) }}</span>
              </div>
              <div class="info-row">
                <span class="info-label">签到情况</span>
                <span v-if="course.openCheckin" class="checkin-note">{{ course.openCheckin.title }}<span v-if="course.openCheckin.type"> · {{ course.openCheckin.type }}</span></span>
                <span v-else-if="course.checkinState === 'closed'" class="checkin-note muted">未开启</span>
                <span v-else class="checkin-note muted">获取失败</span>
              </div>
              <div class="info-row">
                <span class="info-label">资源状态</span>
                <span class="resource-pill">已完成 {{ course.resourceState.completed }} / 未完成 {{ course.resourceState.incomplete }}</span>
              </div>
            </div>
          </article>
        </div>
        <section v-else class="state-card state-card--compact">
          <h2>还没有课程数据</h2>
          <p class="state-copy">当前账号没有返回课程列表，或者数据暂时还没同步出来。</p>
        </section>
      </template>

      <template v-else-if="currentView === 'completion'">
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
                  <input type="checkbox" :checked="allCoursesSelected" :indeterminate="completionCourseIds.size > 0 && !allCoursesSelected" @change="toggleAllCourses" />
                  <span>全选</span>
                </label>
                <span class="course-count-badge">{{ completionCourseIds.size }} / {{ courses.length }}</span>
              </div>
              <div class="course-select-body">
                <AnimatedList
                  v-if="sortedCourses.length"
                  :items="sortedCourses.map(c => ({ id: c.clazzCourseId, label: c.courseName, subLabel: c.className + ' · ' + c.teacherName }))"
                  :selected-ids="completionCourseIds"
                  :show-gradients="true"
                  :enable-arrow-navigation="true"
                  @item-selected="onAnimatedItemSelected"
                />
                <p v-else class="completion-empty">暂无可选课程</p>
              </div>
            </div>
            <div class="completion-action-panel">
              <button class="btn btn-primary btn-complete" :disabled="completionCourseIds.size === 0 || completing" @click="completing = true">
                <svg :class="{ spinning: completing }" width="18" height="18" viewBox="0 0 18 18" fill="none" style="display:block">
                  <path d="M14.5 9a5.5 5.5 0 01-10.4 2.5M3.5 9a5.5 5.5 0 0110.4-2.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                  <path d="M14.5 3.5v3h-3M3.5 14.5v-3h3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                {{ completing ? "处理中..." : "一键完成资源" }}
              </button>
            </div>
          </div>
        </div>
      </template>

      <template v-else>
        <div class="profile-layout">
          <aside class="profile-sidebar">
            <div class="profile-card">
              <div class="profile-avatar">{{ userInitials }}</div>
              <h2 class="profile-name">{{ session.user?.fullName ?? "已登录用户" }}</h2>
              <p class="profile-meta">{{ session.user?.schoolName ?? "未知学校" }}<span v-if="session.user?.departmentName"> · {{ session.user.departmentName }}</span></p>
              <p v-if="session.user?.studentNo" class="profile-id">学号 {{ session.user.studentNo }}</p>
              <div class="profile-stats">
                <div class="profile-stat"><span class="profile-stat-value">{{ courses.length }}</span><span class="profile-stat-label">总课程</span></div>
                <div class="profile-stat"><span class="profile-stat-value">{{ totalCompletedResources }}</span><span class="profile-stat-label">已完成</span></div>
                <div class="profile-stat"><span class="profile-stat-value">{{ totalIncompleteResources }}</span><span class="profile-stat-label">未完成</span></div>
              </div>
            </div>
            <div class="settings-panel">
              <h3 class="settings-heading">设置</h3>
              <div class="settings-list">
                <button :class="['settings-item', { active: selectedSetting === 'account' }]" @click="selectedSetting = 'account'">
                  <svg class="settings-icon" width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M8 8a3 3 0 100-6 3 3 0 000 6zM2 14c0-2.5 2.7-4.5 6-4.5s6 2 6 4.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
                  账号安全
                </button>
                <button :class="['settings-item', { active: selectedSetting === 'about' }]" @click="selectedSetting = 'about'">
                  <svg class="settings-icon" width="16" height="16" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.5"/><path d="M8 7.5v4M8 5v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
                  关于 YbkAuto
                </button>
                <button class="settings-item settings-item--danger" @click="logout">
                  <svg class="settings-icon" width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M6 2H3a1 1 0 00-1 1v10a1 1 0 001 1h3M10 11.5L13.5 8 10 4.5M13.5 8H6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  退出登录
                </button>
              </div>
            </div>
          </aside>
          <main class="profile-content">
            <template v-if="selectedSetting === 'about'">
              <div class="settings-content">
                <h3 class="settings-content-title">关于 YbkAuto</h3>
                <div class="about-card">
                  <div class="about-logo">Y</div>
                  <div class="about-info"><p class="about-name">YbkAuto</p><p class="about-version">版本 0.1.0</p><p class="about-desc">云班课桌面助手 — 课程管理、签到提醒、资源追踪</p></div>
                </div>
                <p class="about-tech">技术栈: Tauri v2 · Vue 3 · TypeScript · Rust</p>
              </div>
            </template>
            <template v-else-if="selectedSetting === 'account'">
              <div class="settings-content">
                <h3 class="settings-content-title">账号安全</h3>
                <div class="account-info-card">
                  <div class="account-row"><span class="account-row-label">登录账号</span><span class="account-row-value">{{ session.rememberedUsername || "未知" }}</span></div>
                  <div class="account-row"><span class="account-row-label">姓名</span><span class="account-row-value">{{ session.user?.fullName ?? "未知" }}</span></div>
                  <div class="account-row"><span class="account-row-label">学校</span><span class="account-row-value">{{ session.user?.schoolName ?? "未知" }}</span></div>
                  <div class="account-row"><span class="account-row-label">学号</span><span class="account-row-value">{{ session.user?.studentNo ?? "未知" }}</span></div>
                </div>
              </div>
            </template>
          </main>
        </div>
      </template>
    </section>
  </main>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=DM+Sans:opsz,wght@9..40,400..700&family=Noto+Sans+SC:wght@400..700&family=JetBrains+Mono:wght@400;500&display=swap');

:root {
  --bg: #f5f5f7;
  --bg-rgb: 245, 245, 247;
  --bg-2: #eeedf2;
  --surface: #ffffff;
  --surface-rgb: 255, 255, 255;
  --surface-hover: #f0f0f4;
  --surface-2: #f8f8fc;
  --border: #e0dee8;
  --border-rgb: 224, 222, 232;
  --border-strong: rgba(0,0,0,0.12);
  --text: #1a1a20;
  --text-rgb: 26, 26, 32;
  --text-2: #4f4e5c;
  --text-3: #8684a0;
  --text-4: #b0aec4;
  --accent-cool: #3b82f6;
  --accent-cool-rgb: 59, 130, 246;
  --accent-cool-soft: rgba(59, 130, 246, 0.10);
  --accent-cool-dim: #2563eb;
  --accent-cool-hover-dim: #3b82f6;
  --accent-warm: #f97316;
  --accent-warm-rgb: 249, 115, 22;
  --gradient-key: linear-gradient(135deg, #3b82f6 0%, #f97316 100%);
  --success: #22c55e;
  --success-rgb: 34, 197, 94;
  --warning: #f97316;
  --warning-rgb: 249, 115, 22;
  --error: #ef4444;
  --error-rgb: 239, 68, 68;
  --shadow-color: 0, 0, 0;
  --shadow-strength: 6%;
  --ease: cubic-bezier(.2, 0, 0, 1);
  --ease-cinema: cubic-bezier(.16, 1, .3, 1);
  --font-ui: 'DM Sans', 'Noto Sans SC', system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', ui-monospace, 'SF Mono', Menlo, monospace;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #0a0b0e;
    --bg-rgb: 10, 11, 14;
    --bg-2: #07080b;
    --surface: #111217;
    --surface-rgb: 17, 18, 23;
    --surface-hover: #1a1c25;
    --surface-2: #171921;
    --border: rgba(67, 70, 81, 0.5);
    --border-rgb: 67, 70, 81;
    --border-strong: rgba(67, 70, 81, 0.9);
    --text: #ebecef;
    --text-rgb: 235, 236, 239;
    --text-2: #c6c9d2;
    --text-3: #8d909c;
    --text-4: #60636f;
    --accent-cool: #5EEAD4;
    --accent-cool-rgb: 94, 234, 212;
    --accent-cool-soft: rgba(94, 234, 212, 0.12);
    --accent-cool-dim: #3b9e8e;
    --accent-cool-hover-dim: #47b8a5;
    --accent-warm: #FB923C;
    --accent-warm-rgb: 251, 146, 60;
    --gradient-key: linear-gradient(135deg, #5EEAD4 0%, #FB923C 100%);
    --success: #30d158;
    --warning: #ffd60a;
    --warning-rgb: 255, 214, 10;
    --error: #ff453a;
    --shadow-strength: 24%;
  }
}

* { box-sizing: border-box; }

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
  font-family: var(--font-ui);
  font-size: 0.875rem;
  font-weight: 400;
  line-height: 1.5;
  color: var(--text);
  background: var(--bg);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
  text-size-adjust: 100%;
  overflow-x: hidden;
}

button, input { font: inherit; }
#app { min-height: 100vh; }
h1, h2, h3, p { margin: 0; }
</style>

<style scoped>
.app-shell { min-height: 100vh; padding: 32px; }
.auth-layout, .dashboard-layout { width: min(1100px, 100%); margin: 0 auto; }

/* Aurora Background */
.aurora-bg {
  position: fixed; inset: 0; z-index: -1; overflow: hidden; pointer-events: none;
}
.aurora-bg::before, .aurora-bg::after {
  content: ''; position: absolute; width: 600px; height: 600px; border-radius: 50%;
  filter: blur(100px); opacity: 0.25;
  animation: auroraDrift 18s ease-in-out infinite alternate;
}
.aurora-bg::before { top: -15%; left: -8%; background: var(--accent-cool); }
.aurora-bg::after { bottom: -15%; right: -8%; background: var(--accent-warm); animation-delay: -6s; }
@keyframes auroraDrift {
  0% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(40px, -30px) scale(1.05); }
  66% { transform: translate(-20px, 20px) scale(0.95); }
  100% { transform: translate(30px, -10px) scale(1.02); }
}

/* Brand Gradient Text */
.brand-gradient {
  background: var(--gradient-key);
  background-size: 200% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: gradientShift 4s ease-in-out infinite alternate;
}
@keyframes gradientShift {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}

/* Eyebrow */
.eyebrow {
  display: inline-block;
  margin: 0 0 8px;
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.14em;
}

/* Auth Layout */
.auth-layout {
  display: grid;
  grid-template-columns: minmax(280px, 460px) minmax(320px, 392px);
  gap: 24px;
  align-items: center;
  min-height: calc(100vh - 64px);
  position: relative;
}
.auth-copy { padding-right: 12px; }
.auth-desc, .state-copy { color: var(--text-2); font-size: 0.9375rem; line-height: 1.6; }

.auth-card {
  border: 1px solid var(--border);
  border-radius: 24px;
  padding: 22px 24px;
  background: rgba(var(--surface-rgb), 0.65);
  backdrop-filter: blur(18px);
  -webkit-backdrop-filter: blur(18px);
  box-shadow: 0 24px 64px rgba(var(--shadow-color), calc(var(--shadow-strength) * 3)), 0 1px 0 rgba(var(--text-rgb), 0.04) inset;
}

/* Dashboard Header */
.dashboard-header { padding: 0; }

/* View Tabs */
.view-tabs {
  display: flex; gap: 2px;
  background: var(--border); border-radius: 12px; padding: 2px; width: fit-content;
}
.view-tab {
  padding: 8px 20px; border: none; border-radius: 10px; background: transparent;
  color: var(--text-2); font-family: inherit; font-size: 0.875rem; font-weight: 500;
  cursor: pointer; transition: all 0.2s ease; line-height: 1.4;
}
.view-tab:hover { color: var(--text); }
.view-tab.active {
  background: var(--surface); color: var(--text);
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}

/* Summary Strip */
.summary-strip {
  display: grid; grid-template-columns: repeat(3, 1fr);
  gap: 2px; background: var(--border); border-radius: 16px; overflow: hidden;
  border: 1px solid var(--border);
  opacity: 0; transform: translateY(-8px);
  animation: summaryIn 0.6s 0.15s var(--ease-cinema) forwards;
}
@keyframes summaryIn { to { opacity: 1; transform: translateY(0); } }

.summary-item {
  background: var(--surface); padding: 18px 22px;
  display: flex; flex-direction: column; gap: 4px;
}
.summary-item--has-action { position: relative; }
.summary-action-btn {
  position: absolute; top: 18px; right: 22px;
}

.summary-item-row { display: flex; align-items: center; justify-content: space-between; gap: 8px; }

.summary-label {
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4); text-transform: uppercase; letter-spacing: 0.04em;
}
.summary-value { font-size: 1.125rem; font-weight: 700; color: var(--text); letter-spacing: -0.02em; line-height: 1.2; }

/* Buttons */
.btn {
  display: inline-flex; align-items: center; justify-content: center;
  gap: 6px; min-height: 36px; padding: 0 16px; border: none; border-radius: 9999px;
  font-family: var(--font-ui); font-size: 0.875rem; font-weight: 500;
  line-height: 1; cursor: pointer; user-select: none; white-space: nowrap;
  transition: background 0.18s var(--ease), border-color 0.12s var(--ease), transform 0.22s var(--ease), box-shadow 0.22s var(--ease);
}
.btn:focus-visible { outline: 2px solid var(--accent-cool); outline-offset: 2px; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; transform: none !important; }

.btn-primary { background: var(--accent-cool-dim); color: #ffffff; }
.btn-primary:hover:not(:disabled) { background: var(--accent-cool-hover-dim); transform: translateY(-1px); box-shadow: 0 2px 6px rgba(var(--accent-cool-rgb), 0.12); }

.btn-ghost { background: transparent; color: var(--text); border: 1px solid var(--border); }
.btn-ghost:hover:not(:disabled) { background: var(--surface-hover); border-color: var(--border-strong); transform: translateY(-1px); }
.btn-ghost-danger { color: var(--error); }
.btn-ghost-danger:hover:not(:disabled) { background: rgba(var(--error-rgb), 0.08); border-color: var(--error); }

.btn-icon {
  display: flex; align-items: center; justify-content: center;
  width: 32px; height: 32px; border: none; border-radius: 8px;
  background: transparent; color: var(--text-3); cursor: pointer;
  transition: all 0.2s var(--ease); flex-shrink: 0;
}
.btn-icon:hover:not(:disabled) { background: var(--surface-hover); color: var(--accent-cool); }
.magnetic-btn { transition: background 0.2s var(--ease), color 0.2s var(--ease), transform 0.3s var(--ease-cinema); }
.magnetic-btn:hover:not(:disabled) { transform: scale(1.15); }
.refresh-icon { display: block; }
.refresh-icon.spinning { animation: spin 0.8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }

/* Error Banner */
.banner { padding: 10px 14px; border-radius: 12px; font-size: 0.875rem; }
.banner--error { background: rgba(var(--error-rgb), 0.1); border: 1px solid rgba(var(--error-rgb), 0.25); color: var(--error); }

/* Course Grid */
.course-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }

/* Course Card (SpotlightCard) */
.course-card {
  position: relative; background: var(--surface); border: 1px solid var(--border);
  border-radius: 16px; padding: 20px;
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
  opacity: 0; transform: translateY(16px);
  transition: opacity 0.6s var(--ease-cinema), transform 0.6s var(--ease-cinema), box-shadow 0.25s var(--ease), border-color 0.2s var(--ease);
  overflow: hidden;
}
.course-card::before {
  content: ''; position: absolute; inset: 0; border-radius: inherit; opacity: 0;
  background: radial-gradient(600px circle at var(--mx, 50%) var(--my, 50%), rgba(var(--accent-cool-rgb), 0.08) 0%, transparent 60%);
  transition: opacity 0.3s var(--ease); pointer-events: none; z-index: 0;
}
.course-card:hover::before { opacity: 1; }
.course-card > * { position: relative; z-index: 1; }
.course-card.in-view { opacity: 1; transform: translateY(0); }
.course-card:hover {
  transform: translateY(-3px); border-color: var(--border-strong);
  box-shadow: 0 8px 24px rgba(var(--shadow-color), calc(var(--shadow-strength) * 2)), 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}

.course-card__top { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; margin-bottom: 18px; }
.course-class { font-size: 0.8125rem; font-weight: 500; color: var(--text-3); text-transform: uppercase; letter-spacing: 0.04em; margin-bottom: 4px; }
.course-card__top h2 { font-size: 1.125rem; font-weight: 600; color: var(--text); line-height: 1.3; letter-spacing: -0.01em; }
.course-teacher { margin-top: 6px; font-size: 0.8125rem; color: var(--text-2); }
.course-state {
  display: inline-flex; align-items: center; justify-content: center;
  min-height: 26px; padding: 0 10px; border-radius: 999px;
  font-size: 0.75rem; font-weight: 600; line-height: 1;
  background: var(--accent-cool-soft); color: var(--accent-cool); flex-shrink: 0;
}

.course-card__body { display: flex; flex-direction: column; gap: 12px; }
.info-row { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
.info-label { font-size: 0.8125rem; font-weight: 500; color: var(--text-2); flex-shrink: 0; }
.checkin-note { font-size: 0.8125rem; color: var(--text-2); line-height: 1.5; }
.muted { color: var(--text-3) !important; }

/* Status Chips */
.status-chip {
  display: inline-flex; align-items: center; justify-content: center;
  min-height: 24px; padding: 0 10px; border-radius: 999px;
  font-size: 0.75rem; font-weight: 600; line-height: 1; letter-spacing: 0.01em; flex-shrink: 0;
}
.status-chip--success { background: rgba(var(--success-rgb), 0.12); color: var(--success); }
.status-chip--warning { background: var(--accent-cool-soft); color: var(--accent-cool); }
.status-chip--muted { background: rgba(var(--text-rgb), 0.07); color: var(--text-2); }

.resource-pill {
  display: inline-flex; align-items: center; justify-content: center;
  min-height: 28px; padding: 0 12px; border-radius: 999px;
  font-size: 0.8125rem; font-weight: 500;
  background: rgba(var(--text-rgb), 0.06); color: var(--text-2);
}

/* State Card */
.state-card {
  width: min(460px, 100%); margin: 80px auto 0;
  background: var(--surface); border: 1px solid var(--border); border-radius: 16px;
  padding: 28px; box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}
.state-card--compact { width: 100%; margin: 0; }
.state-card h1 { font-size: 1.75rem; font-weight: 700; color: var(--text); letter-spacing: -0.02em; }
.state-card h2 { font-size: 1.25rem; font-weight: 600; color: var(--text); }
.state-copy { margin-top: 8px; }

/* Dashboard Layout */
.dashboard-layout { display: flex; flex-direction: column; gap: 20px; }

/* Completion Layout */
.completion-layout { display: flex; flex-direction: column; gap: 20px; }
.completion-header { padding: 0; }
.completion-desc { font-size: 0.875rem; color: var(--text-2); margin-top: 4px; line-height: 1.5; }
.completion-body { display: grid; grid-template-columns: 1fr 280px; gap: 24px; align-items: start; }

.course-select-panel { background: var(--surface); border: 1px solid var(--border); border-radius: 16px; overflow: hidden; }
.course-select-header { display: flex; align-items: center; gap: 12px; padding: 16px 20px; border-bottom: 1px solid var(--border); }
.course-select-title { font-size: 0.9375rem; font-weight: 600; color: var(--text); flex: 1; }
.course-select-all { display: flex; align-items: center; gap: 6px; font-size: 0.8125rem; font-weight: 500; color: var(--text-2); cursor: pointer; user-select: none; }
.course-select-all input[type="checkbox"] { accent-color: var(--accent-cool); width: 16px; height: 16px; cursor: pointer; }
.course-count-badge { font-size: 0.75rem; font-weight: 600; color: var(--text-3); background: rgba(var(--text-rgb), 0.06); padding: 2px 10px; border-radius: 999px; }
.course-select-body { position: relative; min-height: 200px; }
.completion-empty { padding: 40px 20px; text-align: center; font-size: 0.875rem; color: var(--text-3); }

.completion-action-panel {
  background: var(--surface); border: 1px solid var(--border); border-radius: 16px;
  padding: 24px; display: flex; flex-direction: column; gap: 16px;
  position: sticky; top: 0;
}
.btn-complete { width: 100%; min-height: 44px; gap: 8px; font-size: 0.9375rem; font-weight: 600; }

/* Profile Layout */
.profile-layout { display: grid; grid-template-columns: 280px 1fr; gap: 24px; align-items: start; }

.profile-sidebar {
  display: flex; flex-direction: column; gap: 2px;
  background: var(--border); border-radius: 16px; overflow: hidden; border: 1px solid var(--border);
}
.profile-card { background: var(--surface); padding: 28px 24px; display: flex; flex-direction: column; align-items: center; text-align: center; }
.profile-avatar {
  width: 64px; height: 64px; border-radius: 50%;
  background: var(--accent-cool-dim); color: #ffffff;
  display: flex; align-items: center; justify-content: center;
  font-size: 1.5rem; font-weight: 700; letter-spacing: -0.02em; margin-bottom: 14px; flex-shrink: 0;
}
.profile-name { font-size: 1.125rem; font-weight: 600; color: var(--text); line-height: 1.3; margin-bottom: 4px; }
.profile-meta { font-size: 0.8125rem; color: var(--text-2); line-height: 1.5; }
.profile-id { font-size: 0.75rem; color: var(--text-3); margin-top: 2px; }
.profile-stats { display: grid; grid-template-columns: repeat(3, 1fr); width: 100%; margin-top: 20px; padding-top: 18px; border-top: 1px solid var(--border); }
.profile-stat { display: flex; flex-direction: column; align-items: center; gap: 2px; }
.profile-stat + .profile-stat { border-left: 1px solid var(--border); }
.profile-stat-value { font-size: 1.25rem; font-weight: 700; color: var(--text); letter-spacing: -0.02em; line-height: 1.2; }
.profile-stat-label { font-size: 0.75rem; font-weight: 500; color: var(--text-3); }

.settings-panel { background: var(--surface); padding: 20px 16px; }
.settings-heading { font-size: 0.75rem; font-weight: 600; color: var(--text-3); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 12px; padding: 0 8px; }
.settings-list { display: flex; flex-direction: column; gap: 2px; }
.settings-item {
  display: flex; align-items: center; gap: 10px; width: 100%; padding: 10px 12px;
  border: none; border-radius: 10px; background: transparent; color: var(--text);
  font-family: inherit; font-size: 0.875rem; font-weight: 500; cursor: pointer;
  text-align: left; transition: all 0.15s ease; line-height: 1.4;
}
.settings-item:hover { background: var(--surface-hover); }
.settings-item.active { background: var(--accent-cool-soft); color: var(--accent-cool); }
.settings-item--danger { color: var(--error); }
.settings-item--danger:hover { background: rgba(var(--error-rgb), 0.08); }
.settings-icon { flex-shrink: 0; opacity: 0.6; }
.settings-item.active .settings-icon { opacity: 1; }

.profile-content { background: var(--surface); border: 1px solid var(--border); border-radius: 16px; padding: 28px; min-height: 400px; }
.settings-content-title { font-size: 1.25rem; font-weight: 600; color: var(--text); margin-bottom: 20px; letter-spacing: -0.01em; }

.about-card { display: flex; align-items: center; gap: 16px; padding: 20px; border-radius: 14px; background: rgba(var(--accent-cool-rgb), 0.06); margin-bottom: 16px; }
.about-logo {
  width: 48px; height: 48px; border-radius: 14px;
  background: var(--accent-cool-dim); color: #ffffff;
  display: flex; align-items: center; justify-content: center;
  font-size: 1.5rem; font-weight: 700; flex-shrink: 0;
}
.about-name { font-size: 1rem; font-weight: 600; color: var(--text); }
.about-version { font-size: 0.8125rem; color: var(--text-3); margin-top: 2px; }
.about-desc { font-size: 0.8125rem; color: var(--text-2); margin-top: 4px; line-height: 1.5; }
.about-tech { font-size: 0.8125rem; color: var(--text-3); padding: 0 4px; }

.account-info-card { display: flex; flex-direction: column; border-radius: 14px; border: 1px solid var(--border); overflow: hidden; }
.account-row { display: flex; align-items: center; justify-content: space-between; padding: 14px 18px; gap: 12px; }
.account-row + .account-row { border-top: 1px solid var(--border); }
.account-row-label { font-size: 0.875rem; font-weight: 500; color: var(--text-2); }
.account-row-value { font-size: 0.875rem; color: var(--text); font-weight: 500; }

/* Reduced Motion */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
  .course-card { opacity: 1; transform: none; transition-delay: 0s !important; }
  .course-card::before { display: none; }
  .magnetic-btn:hover { transform: none !important; }
  .summary-strip { opacity: 1; transform: none; animation: none !important; }
  .brand-gradient { animation: none !important; }
  .aurora-bg { display: none; }
}

/* Responsive */
@media (max-width: 900px) {
  .app-shell { padding: 18px; }
  .auth-layout { grid-template-columns: 1fr; min-height: auto; padding: 24px 0; }
  .summary-strip { grid-template-columns: 1fr; }
  .completion-body { grid-template-columns: 1fr; }
  .completion-action-panel { position: static; }
  .profile-layout { grid-template-columns: 1fr; }
  .view-tabs { width: 100%; }
  .view-tab { flex: 1; text-align: center; }
  .course-card__top, .info-row { flex-direction: column; align-items: flex-start; }
}
</style>
