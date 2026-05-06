<script setup lang="ts">
import { ref } from "vue";
import { useSessionStore } from "../stores/session";
import { useRouter } from "vue-router";
import { checkUpdate } from "../utils/update";

const sessionStore = useSessionStore();
const router = useRouter();
const selectedSetting = ref<string>("about");
const isCheckingUpdate = ref(false);

const handleLogout = async () => {
  await sessionStore.logout();
  router.push("/login");
};

const handleCheckUpdate = async () => {
  isCheckingUpdate.value = true;
  try {
    await checkUpdate();
  } finally {
    isCheckingUpdate.value = false;
  }
};
</script>

<template>
  <div class="profile-layout">
    <aside class="profile-sidebar">
      <div class="profile-card">
        <div class="profile-avatar">{{ sessionStore.userInitials }}</div>
        <h2 class="profile-name">
          {{ sessionStore.session.user?.fullName ?? "已登录用户" }}
        </h2>
        <p class="profile-meta">
          {{ sessionStore.session.user?.schoolName ?? "未知学校" }}<span
            v-if="sessionStore.session.user?.departmentName"
          >
            · {{ sessionStore.session.user.departmentName }}</span
          >
        </p>
        <p v-if="sessionStore.session.user?.studentNo" class="profile-id">
          学号 {{ sessionStore.session.user.studentNo }}
        </p>
        <div class="profile-stats">
          <div class="profile-stat">
            <span class="profile-stat-value">{{ sessionStore.courses.length }}</span
            ><span class="profile-stat-label">总课程</span>
          </div>
          <div class="profile-stat">
            <span class="profile-stat-value">{{
              sessionStore.totalCompletedResources
            }}</span
            ><span class="profile-stat-label">已完成</span>
          </div>
          <div class="profile-stat">
            <span class="profile-stat-value">{{
              sessionStore.totalIncompleteResources
            }}</span
            ><span class="profile-stat-label">未完成</span>
          </div>
        </div>
      </div>
      <div class="settings-panel">
        <h3 class="settings-heading">设置</h3>
        <div class="settings-list">
          <button
            :class="['settings-item', { active: selectedSetting === 'account' }]"
            @click="selectedSetting = 'account'"
          >
            <svg
              class="settings-icon"
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="none"
              ><path
                d="M8 8a3 3 0 100-6 3 3 0 000 6zM2 14c0-2.5 2.7-4.5 6-4.5s6 2 6 4.5"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
              /></svg>
            账号安全
          </button>
          <button
            :class="['settings-item', { active: selectedSetting === 'about' }]"
            @click="selectedSetting = 'about'"
          >
            <svg
              class="settings-icon"
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="none"
              ><circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.5" /><path
                d="M8 7.5v4M8 5v.5"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
              /></svg>
            关于 YbkAuto
          </button>
          <button class="settings-item settings-item--danger" @click="handleLogout">
            <svg
              class="settings-icon"
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="none"
              ><path
                d="M6 2H3a1 1 0 00-1 1v10a1 1 0 001 1h3M10 11.5L13.5 8 10 4.5M13.5 8H6"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              /></svg>
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
            <div class="about-info">
              <p class="about-name">YbkAuto</p>
              <p class="about-version">版本 0.1.0</p>
              <p class="about-desc">云班课桌面助手 — 课程管理、资源追踪、资源完成</p>
            </div>
          </div>
          <p class="about-tech">技术栈: Tauri v2 · Vue 3 · TypeScript · Rust</p>

          <div class="update-section">
            <button
              class="update-btn"
              :disabled="isCheckingUpdate"
              @click="handleCheckUpdate"
            >
              <svg
                :class="['update-icon', { spinning: isCheckingUpdate }]"
                width="16"
                height="16"
                viewBox="0 0 16 16"
                fill="none"
              >
                <path
                  d="M13.5 8a5.5 5.5 0 01-10.4 2.5M2.5 8a5.5 5.5 0 0110.4-2.5"
                  stroke="currentColor"
                  stroke-width="1.5"
                  stroke-linecap="round"
                />
                <path
                  d="M13.5 2.5v3h-3M2.5 13.5v-3h3"
                  stroke="currentColor"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
              {{ isCheckingUpdate ? '检查中...' : '检查更新' }}
            </button>
          </div>

          <div class="contact-card">
            <div class="contact-icon-wrapper">
              <svg class="contact-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <rect width="20" height="16" x="2" y="4" rx="2"/>
                <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
              </svg>
            </div>
            <div class="contact-content">
              <h4 class="contact-title">意见反馈</h4>
              <p class="contact-desc">如有任何问题请向该邮箱反馈</p>
              <a :href="'mailto:qianmang1@gmail.com'" class="contact-email">qianmang1@gmail.com</a>
            </div>
          </div>
        </div>
      </template>
      <template v-else-if="selectedSetting === 'account'">
        <div class="settings-content">
          <h3 class="settings-content-title">账号安全</h3>
          <div class="account-info-card">
            <div class="account-row">
              <span class="account-row-label">登录账号</span><span
                class="account-row-value">{{
                  sessionStore.session.rememberedUsername || "未知"
                }}</span>
            </div>
            <div class="account-row">
              <span class="account-row-label">姓名</span><span
                class="account-row-value">{{
                  sessionStore.session.user?.fullName ?? "未知"
                }}</span>
            </div>
            <div class="account-row">
              <span class="account-row-label">学校</span><span
                class="account-row-value">{{
                  sessionStore.session.user?.schoolName ?? "未知"
                }}</span>
            </div>
            <div class="account-row">
              <span class="account-row-label">学号</span><span
                class="account-row-value">{{
                  sessionStore.session.user?.studentNo ?? "未知"
                }}</span>
            </div>
          </div>
        </div>
      </template>
    </main>
  </div>
</template>

<style scoped>
.profile-layout {
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: 24px;
  align-items: start;
}

.profile-sidebar {
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: var(--border);
  border-radius: 16px;
  overflow: hidden;
  border: 1px solid var(--border);
}

.profile-card {
  background: var(--surface);
  padding: 28px 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.profile-avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--accent-dim);
  color: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 14px;
  flex-shrink: 0;
}

.profile-name {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text);
  line-height: 1.3;
  margin-bottom: 4px;
}

.profile-meta {
  font-size: 0.8125rem;
  color: var(--text-2);
  line-height: 1.5;
}

.profile-id {
  font-size: 0.75rem;
  color: var(--text-3);
  margin-top: 2px;
}

.profile-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  width: 100%;
  margin-top: 20px;
  padding-top: 18px;
  border-top: 1px solid var(--border);
}

.profile-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.profile-stat + .profile-stat {
  border-left: 1px solid var(--border);
}

.profile-stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
  line-height: 1.2;
}

.profile-stat-label {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-3);
}

.settings-panel {
  background: var(--surface);
  padding: 20px 16px;
}

.settings-heading {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-3);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  margin-bottom: 12px;
  padding: 0 8px;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.settings-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 12px;
  border: none;
  border-radius: 10px;
  background: transparent;
  color: var(--text);
  font-family: inherit;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  text-align: left;
  transition: all 0.15s ease;
  line-height: 1.4;
}

.settings-item:hover {
  background: var(--surface-hover);
}

.settings-item.active {
  background: var(--accent-soft);
  color: var(--accent);
}

.settings-item--danger {
  color: var(--error);
}

.settings-item--danger:hover {
  background: rgba(var(--error-rgb), 0.08);
}

.settings-icon {
  flex-shrink: 0;
  opacity: 0.6;
}

.settings-item.active .settings-icon {
  opacity: 1;
}

.profile-content {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 28px;
  min-height: 400px;
}

.settings-content-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 20px;
  letter-spacing: -0.01em;
}

.about-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  border-radius: 14px;
  background: rgba(var(--accent-rgb), 0.06);
  margin-bottom: 16px;
}

.about-logo {
  width: 48px;
  height: 48px;
  border-radius: 14px;
  background: var(--accent-dim);
  color: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: 700;
  flex-shrink: 0;
}

.about-name {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text);
}

.about-version {
  font-size: 0.8125rem;
  color: var(--text-3);
  margin-top: 2px;
}

.about-desc {
  font-size: 0.8125rem;
  color: var(--text-2);
  margin-top: 4px;
  line-height: 1.5;
}

.about-tech {
  font-size: 0.8125rem;
  color: var(--text-3);
  padding: 0 4px;
  margin-bottom: 24px;
}

.update-section {
  display: flex;
  justify-content: flex-start;
  margin-bottom: 24px;
}

.contact-card {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  border-radius: 14px;
  background: var(--surface-raised);
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all 0.2s ease;
}

.contact-card:hover {
  border-color: var(--accent);
  transform: translateY(-1px);
}

.contact-icon-wrapper {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: var(--accent-soft);
  color: var(--accent);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.contact-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.contact-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text);
}

.contact-desc {
  font-size: 0.8125rem;
  color: var(--text-2);
  line-height: 1.5;
}

.contact-email {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--accent);
  text-decoration: none;
  margin-top: 4px;
  transition: opacity 0.15s ease;
}

.contact-email:hover {
  opacity: 0.8;
}

.account-info-card {
  display: flex;
  flex-direction: column;
  border-radius: 14px;
  border: 1px solid var(--border);
  overflow: hidden;
}

.account-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  gap: 12px;
}

.account-row + .account-row {
  border-top: 1px solid var(--border);
}

.account-row-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-2);
}

.account-row-value {
  font-size: 0.875rem;
  color: var(--text);
  font-weight: 500;
}

.update-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 40px;
  padding: 0 20px;
  border: none;
  border-radius: 9999px;
  background: var(--accent-soft);
  color: var(--accent);
  font-family: inherit;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s var(--ease);
}

.update-btn:hover:not(:disabled) {
  background: var(--accent-dim);
  color: #ffffff;
}

.update-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.update-icon {
  flex-shrink: 0;
}

.update-icon.spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 900px) {
  .profile-layout {
    grid-template-columns: 1fr;
  }
}
</style>
