import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SessionState, DashboardState } from "../types/login";

export const useSessionStore = defineStore("session", () => {
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
  const sortedCourses = computed(() =>
    [...courses.value].sort((a, b) => b.createTime.localeCompare(a.createTime)),
  );
  const openCheckinCount = computed(
    () => courses.value.filter((course) => course.checkinState === "open").length,
  );
  const totalCompletedResources = computed(() =>
    courses.value.reduce((count, course) => count + course.resourceState.completed, 0),
  );
  const totalIncompleteResources = computed(() =>
    courses.value.reduce((count, course) => count + course.resourceState.incomplete, 0),
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

  const loginSuccess = (nextSession: SessionState) => {
    bootstrapError.value = "";
    dashboardError.value = "";
    session.value = nextSession;
  };

  const refreshDashboard = async () => {
    if (isRefreshing.value) return;
    dashboardError.value = "";
    isRefreshing.value = true;
    try {
      const dashboard = await invoke<DashboardState>("refresh_dashboard");
      session.value = { ...session.value, authenticated: true, dashboard };
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

  return {
    session,
    isBootstrapping,
    isRefreshing,
    bootstrapError,
    dashboardError,
    courses,
    sortedCourses,
    openCheckinCount,
    totalCompletedResources,
    totalIncompleteResources,
    userInitials,
    bootstrapSession,
    loginSuccess,
    refreshDashboard,
    logout,
  };
});
