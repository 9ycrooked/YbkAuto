<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

const courses = computed(() => session.value.dashboard?.courses ?? []);

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
};

const refreshDashboard = async () => {
  if (isRefreshing.value) {
    return;
  }

  dashboardError.value = "";
  isRefreshing.value = true;

  try {
    const dashboard = await invoke<DashboardState>("refresh_dashboard");
    session.value = {
      ...session.value,
      authenticated: true,
      dashboard,
    };
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
    case "open":
      return "进行中";
    case "closed":
      return "未开启";
    default:
      return "获取失败";
  }
};

const checkinClass = (course: CourseSummary) => ({
  "status-chip": true,
  "status-chip--open": course.checkinState === "open",
  "status-chip--closed": course.checkinState === "closed",
  "status-chip--error": course.checkinState === "error",
});

onMounted(() => {
  void bootstrapSession();
});
</script>

<template>
  <main class="app-shell">
    <section v-if="isBootstrapping" class="state-card">
      <p class="eyebrow">YbkAuto</p>
      <h1>正在恢复登录状态</h1>
      <p class="state-copy">稍等一下，我们在帮你检查本地会话。</p>
    </section>

    <section v-else-if="!session.authenticated" class="auth-layout">
      <div class="auth-copy">
        <p class="eyebrow">YbkAuto</p>
        <h1>登录云班课账号</h1>
        <p class="auth-desc">
          首版会记住账号和登录令牌。下次打开时会先尝试自动恢复会话。
        </p>
      </div>

      <div class="auth-card">
        <p v-if="bootstrapError" class="banner banner--error">{{ bootstrapError }}</p>
        <LoginForm
          :remembered-username="session.rememberedUsername"
          @login-success="handleLoginSuccess"
        />
      </div>
    </section>

    <section v-else class="dashboard-layout">
      <header class="dashboard-header">
        <div>
          <p class="eyebrow">课程概览</p>
          <h1>{{ session.user?.fullName ?? "已登录用户" }}</h1>
          <p class="meta-line">
            {{ session.user?.schoolName ?? "未知学校" }}
            <span v-if="session.user?.departmentName">
              · {{ session.user.departmentName }}
            </span>
            <span v-if="session.user?.studentNo">
              · 学号 {{ session.user.studentNo }}
            </span>
          </p>
        </div>

        <div class="header-actions">
          <button class="secondary-btn" :disabled="isRefreshing" @click="refreshDashboard">
            {{ isRefreshing ? "刷新中..." : "刷新" }}
          </button>
          <button class="secondary-btn secondary-btn--warn" @click="logout">
            退出登录
          </button>
        </div>
      </header>

      <p v-if="dashboardError" class="banner banner--error">{{ dashboardError }}</p>

      <div class="summary-strip">
        <div class="summary-item">
          <span class="summary-label">课程数量</span>
          <strong>{{ courses.length }}</strong>
        </div>
        <div class="summary-item">
          <span class="summary-label">开放签到</span>
          <strong>{{ courses.filter((course) => course.checkinState === "open").length }}</strong>
        </div>
        <div class="summary-item">
          <span class="summary-label">资源状态</span>
          <strong>占位中</strong>
        </div>
      </div>

      <div v-if="courses.length" class="course-grid">
        <article
          v-for="course in courses"
          :key="course.clazzCourseId"
          class="course-card"
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

            <p v-if="course.openCheckin" class="checkin-note">
              {{ course.openCheckin.title }}
              <span v-if="course.openCheckin.type">
                · {{ course.openCheckin.type }}
              </span>
            </p>
            <p v-else-if="course.checkinState === 'closed'" class="checkin-note muted">
              当前没有开放签到
            </p>
            <p v-else class="checkin-note muted">
              暂时拿不到这门课的签到状态
            </p>

            <div class="info-row">
              <span class="info-label">未完成资源</span>
              <span class="resource-pill">{{ course.resourceState.label }}</span>
            </div>
          </div>
        </article>
      </div>

      <section v-else class="state-card state-card--compact">
        <h2>还没有课程数据</h2>
        <p class="state-copy">当前账号没有返回课程列表，或者数据暂时还没同步出来。</p>
      </section>
    </section>
  </main>
</template>

<style>
:root {
  color: #e8ecf5;
  background:
    radial-gradient(circle at top, rgba(96, 111, 255, 0.22), transparent 30%),
    linear-gradient(180deg, #0f1525 0%, #0b1020 100%);
  font-family:
    Inter, "PingFang SC", "Microsoft YaHei", -apple-system, BlinkMacSystemFont,
    "Segoe UI", sans-serif;
  line-height: 1.5;
  font-weight: 400;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-size-adjust: 100%;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}

button,
input {
  font: inherit;
}

#app {
  min-height: 100vh;
}
</style>

<style scoped>
.app-shell {
  min-height: 100vh;
  padding: 32px;
}

.auth-layout,
.dashboard-layout {
  width: min(1100px, 100%);
  margin: 0 auto;
}

.auth-layout {
  display: grid;
  grid-template-columns: minmax(280px, 460px) minmax(320px, 420px);
  gap: 24px;
  align-items: center;
  min-height: calc(100vh - 64px);
}

.auth-copy {
  padding-right: 12px;
}

.auth-card,
.state-card,
.course-card,
.summary-strip,
.dashboard-header {
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(8, 13, 25, 0.72);
  backdrop-filter: blur(18px);
  box-shadow: 0 28px 60px rgba(0, 0, 0, 0.28);
}

.auth-card,
.state-card {
  border-radius: 20px;
  padding: 28px;
}

.dashboard-layout {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.dashboard-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 24px 28px;
  border-radius: 20px;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.summary-strip {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
  padding: 18px 22px;
  border-radius: 18px;
}

.summary-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.summary-label,
.course-class,
.info-label,
.eyebrow {
  color: #93a0bf;
  font-size: 0.88rem;
}

.eyebrow {
  margin: 0 0 8px;
  letter-spacing: 0;
  text-transform: uppercase;
}

.auth-desc,
.state-copy,
.meta-line,
.course-teacher,
.checkin-note {
  color: #c1cae0;
}

h1,
h2,
p {
  margin: 0;
}

h1 {
  font-size: clamp(2rem, 4vw, 3.2rem);
  line-height: 1.05;
}

h2 {
  font-size: 1.2rem;
  line-height: 1.3;
}

.meta-line {
  margin-top: 8px;
}

.banner {
  padding: 12px 14px;
  border-radius: 12px;
  font-size: 0.92rem;
}

.banner--error {
  background: rgba(255, 102, 124, 0.14);
  border: 1px solid rgba(255, 102, 124, 0.25);
  color: #ffb2bc;
}

.course-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 16px;
}

.course-card {
  border-radius: 18px;
  padding: 20px;
}

.course-card__top,
.info-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.course-card__top {
  margin-bottom: 18px;
}

.course-card__body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.course-teacher {
  margin-top: 8px;
}

.course-state,
.resource-pill,
.status-chip,
.secondary-btn {
  border-radius: 999px;
}

.course-state,
.resource-pill,
.status-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 30px;
  padding: 0 12px;
  font-size: 0.82rem;
  font-weight: 600;
}

.course-state {
  background: rgba(114, 138, 255, 0.14);
  color: #bcc8ff;
}

.resource-pill {
  background: rgba(255, 255, 255, 0.08);
  color: #d9e1f5;
}

.status-chip--open {
  background: rgba(38, 201, 116, 0.16);
  color: #8df0bb;
}

.status-chip--closed {
  background: rgba(255, 201, 87, 0.14);
  color: #ffd88b;
}

.status-chip--error {
  background: rgba(255, 102, 124, 0.14);
  color: #ffb2bc;
}

.muted {
  color: #93a0bf;
}

.secondary-btn {
  min-height: 40px;
  padding: 0 16px;
  border: 1px solid rgba(255, 255, 255, 0.12);
  background: rgba(255, 255, 255, 0.04);
  color: #eef2ff;
  cursor: pointer;
  transition:
    background 0.18s ease,
    border-color 0.18s ease,
    opacity 0.18s ease;
}

.secondary-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.2);
}

.secondary-btn:disabled {
  cursor: wait;
  opacity: 0.72;
}

.secondary-btn--warn {
  color: #ffb2bc;
}

.state-card {
  width: min(460px, 100%);
  margin: 80px auto 0;
}

.state-card--compact {
  width: 100%;
  margin: 0;
}

@media (max-width: 900px) {
  .app-shell {
    padding: 18px;
  }

  .auth-layout {
    grid-template-columns: 1fr;
    min-height: auto;
    padding: 24px 0;
  }

  .dashboard-header,
  .course-card__top,
  .info-row {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    flex-wrap: wrap;
  }

  .summary-strip {
    grid-template-columns: 1fr;
  }
}
</style>
