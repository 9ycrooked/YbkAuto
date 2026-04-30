<script setup lang="ts">
import { computed, watch } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useSessionStore } from "./stores/session";

const router = useRouter();
const route = useRoute();
const sessionStore = useSessionStore();

const currentView = computed(() => {
  if (route.name === "dashboard") return "dashboard";
  if (route.name === "completion") return "completion";
  if (route.name === "profile") return "profile";
  return "dashboard";
});

const navigateTo = (view: string) => {
  router.push({ name: view });
};

watch(
  () => sessionStore.session.authenticated,
  (isAuth) => {
    if (isAuth && route.name === "login") {
      router.push({ name: "dashboard" });
    }
  },
);
</script>

<template>
  <main class="app-shell">
    <section v-if="sessionStore.isBootstrapping" class="state-card">
      <p class="eyebrow brand-gradient">YbkAuto</p>
      <h1>正在恢复登录状态</h1>
      <p class="state-copy">稍等一下，我们在帮你检查本地会话。</p>
    </section>

    <template v-else-if="!sessionStore.session.authenticated">
      <router-view v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" />
        </Transition>
      </router-view>
    </template>

    <section v-else class="dashboard-layout">
      <nav class="view-tabs">
        <button
          :class="['view-tab', { active: currentView === 'dashboard' }]"
          @click="navigateTo('dashboard')"
        >
          课程概览
        </button>
        <button
          :class="['view-tab', { active: currentView === 'completion' }]"
          @click="navigateTo('completion')"
        >
          资源完成
        </button>
        <button
          :class="['view-tab', { active: currentView === 'profile' }]"
          @click="navigateTo('profile')"
        >
          个人中心
        </button>
      </nav>

      <router-view v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" />
        </Transition>
      </router-view>
    </section>
  </main>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
  padding: 32px;
}

.dashboard-layout {
  width: min(1100px, 100%);
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
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

.state-card h1 {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text);
  letter-spacing: -0.02em;
}

.state-copy {
  color: var(--text-2);
  font-size: 0.9375rem;
  line-height: 1.6;
  margin-top: 8px;
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

.view-tabs {
  display: flex;
  gap: 2px;
  background: var(--border);
  border-radius: 12px;
  padding: 2px;
  width: fit-content;
}

.view-tab {
  padding: 8px 20px;
  border: none;
  border-radius: 10px;
  background: transparent;
  color: var(--text-2);
  font-family: inherit;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  line-height: 1.4;
}

.view-tab:hover {
  color: var(--text);
}

.view-tab.active {
  background: var(--surface);
  color: var(--text);
  box-shadow: 0 1px 3px rgba(var(--shadow-color), calc(var(--shadow-strength) * 1));
}

.page-enter-active,
.page-leave-active {
  transition:
    opacity 0.2s var(--ease),
    transform 0.2s var(--ease);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

@media (max-width: 900px) {
  .app-shell {
    padding: 18px;
  }

  .view-tabs {
    width: 100%;
  }

  .view-tab {
    flex: 1;
    text-align: center;
  }
}
</style>
