<script setup lang="ts">
import { useSessionStore } from "../stores/session";
import LoginForm from "../components/LoginForm.vue";

const sessionStore = useSessionStore();
</script>

<template>
  <section class="auth-page">
    <div class="auth-aurora" aria-hidden="true"></div>
    <div class="auth-grid-overlay" aria-hidden="true"></div>

    <div class="auth-container">
      <header class="auth-header">
        <div class="auth-brand">
          <svg width="36" height="36" viewBox="0 0 40 40" fill="none" aria-hidden="true">
            <rect width="40" height="40" rx="12" fill="url(#logo-grad)"/>
            <path d="M14 26V14l6 4.5 6-4.5v12" stroke="#fff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <defs>
              <linearGradient id="logo-grad" x1="0" y1="0" x2="40" y2="40">
                <stop offset="0%" stop-color="var(--accent)"/>
                <stop offset="100%" stop-color="var(--accent-warm)"/>
              </linearGradient>
            </defs>
          </svg>
          <span class="auth-brand-name">YbkAuto</span>
        </div>
        <h1 class="auth-title">登录云班课</h1>
        <p class="auth-subtitle">默认自动登录</p>
      </header>

      <div class="auth-card spotlight-card">
        <p v-if="sessionStore.bootstrapError" class="banner banner--error">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none" aria-hidden="true">
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
    </div>
  </section>
</template>

<style scoped>
.auth-page {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  overflow: hidden;
  background: var(--bg);
}

.auth-aurora {
  position: absolute;
  inset: -40%;
  background:
    radial-gradient(ellipse 80% 60% at 30% 10%, var(--accent-cool) 0%, transparent 45%),
    radial-gradient(ellipse 60% 60% at 70% 90%, var(--accent) 0%, transparent 40%),
    radial-gradient(ellipse 50% 50% at 50% 50%, var(--accent-warm) 0%, transparent 50%);
  opacity: 0.12;
  animation: auroraFloat 12s ease-in-out infinite alternate;
  pointer-events: none;
}

@keyframes auroraFloat {
  0% {
    opacity: 0.08;
    transform: translate(0, 0) scale(1);
  }
  50% {
    opacity: 0.14;
    transform: translate(-2%, 1%) scale(1.02);
  }
  100% {
    opacity: 0.10;
    transform: translate(1%, -1%) scale(0.98);
  }
}

.auth-grid-overlay {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(var(--border) 1px, transparent 1px),
    linear-gradient(90deg, var(--border) 1px, transparent 1px);
  background-size: 60px 60px;
  opacity: 0.3;
  mask-image: radial-gradient(ellipse 80% 60% at 50% 50%, black 20%, transparent 70%);
  pointer-events: none;
}

.auth-container {
  width: 100%;
  max-width: 340px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
  position: relative;
  z-index: 1;
  animation: containerIn 0.6s var(--ease-cinema) both;
}

@keyframes containerIn {
  from {
    opacity: 0;
    transform: translateY(12px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.auth-header {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.auth-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.auth-brand-name {
  font-family: var(--font-mono);
  font-size: 1rem;
  font-weight: 500;
  letter-spacing: 0.06em;
  background: var(--gradient-key);
  background-size: 200% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: brandShift 4s ease-in-out infinite alternate;
}

@keyframes brandShift {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}

.auth-title {
  font-size: 1.375rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
  line-height: 1.3;
}

.auth-subtitle {
  font-family: var(--font-mono);
  font-size: 0.6875rem;
  font-weight: 500;
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.1em;
}

.auth-card {
  width: 100%;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 24px 20px;
  box-shadow:
    0 2px 8px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1)),
    0 0 0 1px rgba(var(--border-rgb), 0.4);
  transition:
    border-color 0.2s var(--ease),
    box-shadow 0.25s var(--ease);
}

.auth-card:focus-within {
  border-color: var(--accent);
  box-shadow:
    0 2px 12px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1.5)),
    0 0 0 1px var(--accent);
}

.spotlight-card {
  --mx: 50%;
  --my: 50%;
  position: relative;
  overflow: hidden;
}

.spotlight-card::before {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  opacity: 0;
  background: radial-gradient(
    500px circle at var(--mx) var(--my),
    rgba(var(--accent-rgb), 0.06) 0%,
    transparent 60%
  );
  transition: opacity 0.3s var(--ease);
  pointer-events: none;
  z-index: 0;
}

.spotlight-card:hover::before {
  opacity: 1;
}

.spotlight-card > * {
  position: relative;
  z-index: 1;
}

.banner {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 10px;
  font-size: 0.8125rem;
  margin-bottom: 16px;
  background: rgba(var(--error-rgb), 0.08);
  border: 1px solid rgba(var(--error-rgb), 0.2);
  color: var(--error);
  line-height: 1.4;
}

.banner svg {
  flex-shrink: 0;
  opacity: 0.8;
}

@media (prefers-color-scheme: dark) {
  .auth-aurora {
    opacity: 0.18;
  }

  .auth-card {
    box-shadow:
      0 2px 12px rgba(0, 0, 0, 0.4),
      0 0 0 1px rgba(var(--border-rgb), 0.6);
  }
}

@media (prefers-reduced-motion: reduce) {
  .auth-aurora {
    animation: none;
    opacity: 0.10;
  }

  .brand-gradient {
    animation: none;
  }

  .auth-container {
    animation: none;
    opacity: 1;
    transform: none;
  }
}
</style>