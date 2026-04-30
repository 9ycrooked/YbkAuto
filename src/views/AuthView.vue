<script setup lang="ts">
import { useSessionStore } from "../stores/session";
import LoginForm from "../components/LoginForm.vue";

const sessionStore = useSessionStore();
</script>

<template>
  <section class="auth-page">
    <div class="auth-glow" aria-hidden="true"></div>

    <div class="auth-container">
      <header class="auth-header">
        <div class="auth-logo">
          <svg width="36" height="36" viewBox="0 0 36 36" fill="none" aria-hidden="true">
            <rect width="36" height="36" rx="10" fill="var(--accent-dim)"/>
            <path d="M12 24V12l6 4.5L24 12v12" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <h1 class="auth-title">登录云班课账号</h1>
        <p class="auth-subtitle">会记住账号和登录令牌</p>
      </header>

      <div class="auth-card">
        <p v-if="sessionStore.bootstrapError" class="banner banner--error">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
            <circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.5"/>
            <path d="M8 5v4M8 11v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
          {{ sessionStore.bootstrapError }}
        </p>
        <LoginForm
          :remembered-username="sessionStore.session.rememberedUsername"
          @login-success="sessionStore.loginSuccess"
        />
      </div>

      <footer class="auth-footer">
        <p>下次打开时会自动恢复会话</p>
      </footer>
    </div>
  </section>
</template>

<style scoped>
.auth-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 60px 20px 20px;
  position: relative;
  overflow: hidden;
}

.auth-glow {
  position: fixed;
  top: -150px;
  left: 50%;
  transform: translateX(-50%);
  width: 400px;
  height: 400px;
  background: radial-gradient(circle, rgba(var(--accent-rgb), 0.15) 0%, transparent 70%);
  pointer-events: none;
  animation: glowPulse 6s ease-in-out infinite;
}

@keyframes glowPulse {
  0%, 100% { opacity: 0.6; transform: translateX(-50%) scale(1); }
  50% { opacity: 1; transform: translateX(-50%) scale(1.1); }
}

.auth-container {
  width: 100%;
  max-width: 360px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
  position: relative;
  z-index: 1;
}

.auth-header {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.auth-logo {
  margin-bottom: 4px;
}

.auth-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
}

.auth-subtitle {
  font-size: 0.9375rem;
  color: var(--text-2);
  line-height: 1.5;
}

.auth-card {
  width: 100%;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 20px;
  padding: 28px 24px;
  box-shadow:
    0 4px 24px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1)),
    0 1px 2px rgba(var(--shadow-color), calc(var(--shadow-strength) * 0.5));
}

.banner {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 14px;
  border-radius: 12px;
  font-size: 0.875rem;
  margin-bottom: 20px;
  background: rgba(var(--error-rgb), 0.08);
  border: 1px solid rgba(var(--error-rgb), 0.2);
  color: var(--error);
}

.banner svg {
  flex-shrink: 0;
  opacity: 0.8;
}

.auth-footer {
  text-align: center;
}

.auth-footer p {
  font-size: 0.8125rem;
  color: var(--text-3);
}

@media (max-width: 380px) {
  .auth-page {
    padding: 16px;
    align-items: flex-start;
    padding-top: 40px;
  }

  .auth-container {
    gap: 20px;
  }

  .auth-card {
    padding: 24px 20px;
    border-radius: 16px;
  }
}

@media (prefers-reduced-motion: reduce) {
  .auth-glow {
    animation: none;
    opacity: 0.6;
  }
}
</style>