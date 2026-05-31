export type UpdatePolicy = {
  check_updates_on_startup: boolean;
  force_update_on_startup: boolean;
  message: string | null;
};

export const DEFAULT_UPDATE_POLICY: UpdatePolicy = {
  check_updates_on_startup: true,
  force_update_on_startup: false,
  message: null,
};

export function normalizeUpdatePolicy(
  value: Partial<UpdatePolicy> | null | undefined,
  fallback: UpdatePolicy = DEFAULT_UPDATE_POLICY,
): UpdatePolicy {
  return {
    check_updates_on_startup:
      typeof value?.check_updates_on_startup === "boolean"
        ? value.check_updates_on_startup
        : fallback.check_updates_on_startup,
    force_update_on_startup:
      typeof value?.force_update_on_startup === "boolean"
        ? value.force_update_on_startup
        : fallback.force_update_on_startup,
    message: typeof value?.message === "string" ? value.message : fallback.message,
  };
}

