// @ts-ignore - wasm_bindgen is a global variable set by encrypt.js script
declare const wasm_bindgen: {
  (): Promise<void>;
  initSync: () => void;
  encrypt_password: (phone: string, password: string) => {
    ciphertext: () => string;
    free: () => void;
  };
  Password: {
    new: (ciphertext: string) => {
      ciphertext: () => string;
      free: () => void;
    };
  };
};

let initialized = false;

export async function initCrypto(): Promise<void> {
  if (initialized) return;

  try {
    console.log("[Crypto] Calling wasm_bindgen()...");
    await wasm_bindgen();
    console.log("[Crypto] WASM initialized successfully");
    initialized = true;
  } catch (e) {
    console.error("[Crypto] Failed to initialize WASM:", e);
    throw e;
  }
}

export async function encryptPassword(
  phone: string,
  password: string
): Promise<string> {
  if (!initialized) {
    await initCrypto();
  }
  try {
    const encrypted = wasm_bindgen.encrypt_password(phone, password);
    const ciphertext = encrypted.ciphertext();
    console.log("[Crypto] Encrypted password, ciphertext length:", ciphertext.length);
    encrypted.free();
    return ciphertext;
  } catch (e) {
    console.error("[Crypto] Encryption failed:", e);
    throw e;
  }
}
