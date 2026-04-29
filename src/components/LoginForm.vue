<script lang="ts" setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

import InputBox from "./FloatingLabelInput.vue";
import LoginButton from "./LoginButton.vue";
import type { SessionState } from "../types/login";

const props = defineProps<{
  rememberedUsername?: string;
}>();

const emit = defineEmits<{
  loginSuccess: [session: SessionState];
}>();

const username = ref("");
const password = ref("");
const loginError = ref("");
const isLoading = ref(false);

watch(
  () => props.rememberedUsername,
  (value) => {
    username.value = value ?? "";
  },
  { immediate: true },
);

const handleLogin = async () => {
  if (isLoading.value) {
    return;
  }

  loginError.value = "";
  isLoading.value = true;

  try {
    const session = await invoke<SessionState>("login_command", {
      username: username.value,
      password: password.value,
    });

    password.value = "";
    emit("loginSuccess", session);
  } catch (error) {
    loginError.value =
      typeof error === "string" ? error : "登录失败，请稍后再试";
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <form class="login-form" @submit.prevent="handleLogin">
    <InputBox v-model="username" label="账号" type="text" />
    <InputBox v-model="password" label="密码" type="password" />

    <p v-if="loginError" class="login-error">{{ loginError }}</p>

    <LoginButton
      :disabled="!username || !password"
      :loading="isLoading"
      label="登录"
    />
  </form>
</template>

<style scoped>
.login-form {
  display: flex;
  flex-direction: column;
  gap: 1.1rem;
  width: 100%;
}

.login-error {
  margin: -0.25rem 0 0;
  color: var(--error);
  font-size: 0.92rem;
  text-align: left;
}
</style>
