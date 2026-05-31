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
    await wasm_bindgen();
    initialized = true;
  } catch (e) {
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
    encrypted.free();
    return ciphertext;
  } catch (e) {
    throw e;
  }
}
