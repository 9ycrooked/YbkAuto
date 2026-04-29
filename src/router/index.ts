import { createRouter, createWebHistory } from "vue-router";
import { useSessionStore } from "../stores/session";

const router = createRouter({
  history: createWebHistory(),
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

router.beforeEach((to, _from, next) => {
  const sessionStore = useSessionStore();

  if (to.meta.requiresAuth && !sessionStore.session.authenticated) {
    next({ name: "login" });
  } else if (to.name === "login" && sessionStore.session.authenticated) {
    next({ name: "dashboard" });
  } else {
    next();
  }
});

export default router;
