/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module "/wasm/encrypt.js" {
  export interface Password {
    ciphertext(): string;
    free(): void;
  }

  export function encrypt_password(phone: string, password: string): Password;
  export default function init(): Promise<void>;
}
