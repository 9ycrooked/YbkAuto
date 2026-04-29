<script setup lang="ts">
import { useSessionStore } from "../stores/session";
import LoginForm from "../components/LoginForm.vue";

const sessionStore = useSessionStore();
</script>

<template>
  <section class="auth-layout">
    <div class="aurora-bg" aria-hidden="true"></div>
    <div class="auth-copy">
      <p class="eyebrow brand-gradient">YbkAuto</p>
      <h1>登录云班课账号</h1>
      <p class="auth-desc">首版会记住账号和登录令牌。下次打开时会先尝试自动恢复会话。</p>
    </div>
    <div class="auth-card">
      <p v-if="sessionStore.bootstrapError" class="banner banner--error">
        {{ sessionStore.bootstrapError }}
      </p>
      <LoginForm
        :remembered-username="sessionStore.session.rememberedUsername"
        @login-success="sessionStore.loginSuccess"
      />
    </div>
  </section>
</template>

<style scoped>
.auth-layout {
  display: grid;
  grid-template-columns: minmax(280px, 460px) minmax(320px, 392px);
  gap: 24px;
  align-items: center;
  min-height: calc(100vh - 64px);
  position: relative;
}

.auth-copy {
  padding-right: 12px;
}

.auth-desc {
  color: var(--text-2);
  font-size: 0.9375rem;
  line-height: 1.6;
}

.auth-card {
  border: 1px solid var(--border);
  border-radius: 24px;
  padding: 22px 24px;
  background: rgba(var(--surface-rgb), 0.65);
  backdrop-filter: blur(18px);
  -webkit-backdrop-filter: blur(18px);
  box-shadow:
    0 24px 64px rgba(var(--shadow-color), calc(var(--shadow-strength) * 3)),
    0 1px 0 rgba(var(--text-rgb), 0.04) inset;
}

.eyebrow {
  display: inline-block;
  margin: 0 0 8px;
  font: 500 0.75rem/1 var(--font-mono);
  color: var(--text-4);
  text-transform: uppercase;
  letter-spacing: 0.14em;
}

.brand-gradient {
  background: var(--gradient-key);
  background-size: 200% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: gradientShift 4s ease-in-out infinite alternate;
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 100% 50%;
  }
}

.aurora-bg {
  position: fixed;
  inset: 0;
  z-index: -1;
  overflow: hidden;
  pointer-events: none;
}

.aurora-bg::before,
.aurora-bg::after {
  content: "";
  position: absolute;
  width: 600px;
  height: 600px;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 0.25;
  animation: auroraDrift 18s ease-in-out infinite alternate;
}

.aurora-bg::before {
  top: -15%;
  left: -8%;
  background: var(--accent);
}

.aurora-bg::after {
  bottom: -15%;
  right: -8%;
  background: var(--accent-warm);
  animation-delay: -6s;
}

@keyframes auroraDrift {
  0% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(40px, -30px) scale(1.05);
  }
  66% {
    transform: translate(-20px, 20px) scale(0.95);
  }
  100% {
    transform: translate(30px, -10px) scale(1.02);
  }
}

.banner {
  padding: 10px 14px;
  border-radius: 12px;
  font-size: 0.875rem;
  margin-bottom: 16px;
}

.banner--error {
  background: rgba(var(--error-rgb), 0.1);
  border: 1px solid rgba(var(--error-rgb), 0.25);
  color: var(--error);
}

@media (max-width: 900px) {
  .auth-layout {
    grid-template-columns: 1fr;
    min-height: auto;
    padding: 24px 0;
  }
}
</style>
