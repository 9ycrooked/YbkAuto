import { createRouter, createWebHashHistory } from "vue-router";
import { useSessionStore } from "../stores/session";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/dashboard",
    },
    {
      path: "/login",
      name: "login",
      component: () => import("../views/AuthView.vue"),
      meta: { requiresAuth: false },
    },
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("../views/DashboardView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/completion",
      name: "completion",
      component: () => import("../views/CompletionView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/profile",
      name: "profile",
      component: () => import("../views/ProfileView.vue"),
      meta: { requiresAuth: true },
    },
  ],
});

router.beforeEach((to, _from) => {
  const sessionStore = useSessionStore();

  if (to.meta.requiresAuth && !sessionStore.session.authenticated) {
    return { name: "login" };
  }
  if (to.name === "login" && sessionStore.session.authenticated) {
    return { name: "dashboard" };
  }
  return true;
});

export default router;
