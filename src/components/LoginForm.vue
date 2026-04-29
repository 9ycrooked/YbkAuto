<script lang="ts" setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

import InputBox from "./FloatingLabelInput.vue";
import LoginButton from "./LoginButton.vue";
import ToastMessage from "./ToastMessage.vue";
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
const showToast = ref(false);
const toastMessage = ref("");

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
    toastMessage.value = "账号或密码错误";
    showToast.value = true;
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

  <ToastMessage
    :visible="showToast"
    :message="toastMessage"
    type="error"
    @close="showToast = false"
  />
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
