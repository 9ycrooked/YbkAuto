import { computed, markRaw, reactive, ref, shallowRef } from "vue";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";
import {
  DEFAULT_UPDATE_POLICY,
  normalizeUpdatePolicy,
  type UpdatePolicy,
} from "../utils/updatePolicy";

const UPDATE_POLICY_URL =
  "https://github.com/9ycrooked/YbkAuto/releases/latest/download/update-policy.json";

type UpdateInfo = Update & {
  body?: string;
  notes?: string;
  version?: string;
  currentVersion?: string;
};

type UpdaterState = ReturnType<typeof createUpdater>;

let updaterState: UpdaterState | null = null;

function createUpdater() {
  const updatePolicy = reactive<UpdatePolicy>({ ...DEFAULT_UPDATE_POLICY });
  const updateDialogOpen = ref(false);
  const updateChecking = ref(false);
  const updateDownloading = ref(false);
  const updateError = ref("");
  const updateMessage = ref("");
  const lastUpdateCheckedAt = ref<string | null>(null);
  const pendingUpdate = shallowRef<Update | null>(null);
  const updateDownloadedBytes = ref(0);
  const updateTotalBytes = ref(0);

  const pendingUpdateInfo = computed(() => pendingUpdate.value as UpdateInfo | null);
  const pendingUpdateNotes = computed(
    () =>
      pendingUpdateInfo.value?.body ||
      pendingUpdateInfo.value?.notes ||
      "这个版本没有填写更新说明。",
  );
  const updateProgressPercent = computed(() => {
    if (!updateTotalBytes.value) return 0;
    return Math.min(
      100,
      Math.round((updateDownloadedBytes.value / updateTotalBytes.value) * 100),
    );
  });
  const updateIsForced = computed(() =>
    Boolean(updatePolicy.force_update_on_startup && pendingUpdate.value),
  );

  async function loadUpdatePolicy(): Promise<UpdatePolicy> {
    try {
      const response = await fetch(UPDATE_POLICY_URL, { cache: "no-store" });
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      const remote = (await response.json()) as Partial<UpdatePolicy>;
      const nextPolicy = normalizeUpdatePolicy(remote);
      Object.assign(updatePolicy, nextPolicy);
      return nextPolicy;
    } catch {
      Object.assign(updatePolicy, DEFAULT_UPDATE_POLICY);
      return { ...DEFAULT_UPDATE_POLICY };
    }
  }

  async function runUpdateCheck(options: { manual?: boolean } = {}) {
    const manual = Boolean(options.manual);
    updateMessage.value = "";

    const policy = await loadUpdatePolicy();
    if (!manual && !policy.check_updates_on_startup) return;

    updateChecking.value = true;
    updateError.value = "";

    try {
      const update = await check();
      if (!update) {
        if (manual) updateMessage.value = "当前已是最新版本";
        return;
      }

      pendingUpdate.value = markRaw(update);
      updateDialogOpen.value = true;
    } catch (error) {
      const message = `更新检查失败：${String(error)}`;
      updateError.value = message;
      if (manual) updateMessage.value = message;
    } finally {
      updateChecking.value = false;
      lastUpdateCheckedAt.value = new Date().toISOString();
    }
  }

  async function installPendingUpdate() {
    if (!pendingUpdate.value) return;

    updateDownloading.value = true;
    updateError.value = "";
    updateDownloadedBytes.value = 0;
    updateTotalBytes.value = 0;

    try {
      await pendingUpdate.value.downloadAndInstall((event) => {
        if (event.event === "Started") {
          updateTotalBytes.value = event.data.contentLength ?? 0;
        }
        if (event.event === "Progress") {
          updateDownloadedBytes.value += event.data.chunkLength;
        }
      });

      await relaunch();
    } catch (error) {
      updateError.value = `更新安装失败：${String(error)}`;
    } finally {
      updateDownloading.value = false;
    }
  }

  function dismissUpdateDialog() {
    if (updateIsForced.value) return;
    updateDialogOpen.value = false;
  }

  return {
    updatePolicy,
    updateDialogOpen,
    updateChecking,
    updateDownloading,
    updateError,
    updateMessage,
    lastUpdateCheckedAt,
    pendingUpdate,
    pendingUpdateInfo,
    pendingUpdateNotes,
    updateDownloadedBytes,
    updateTotalBytes,
    updateProgressPercent,
    updateIsForced,
    runUpdateCheck,
    checkForUpdatesManually: () => runUpdateCheck({ manual: true }),
    installPendingUpdate,
    dismissUpdateDialog,
  };
}

export function useUpdater() {
  if (!updaterState) updaterState = createUpdater();
  return updaterState;
}
