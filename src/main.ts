import { createApp } from "vue";
import { createPinia } from "pinia";
import router from "./router";
import App from "./App.vue";
import "./styles/variables.css";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);
app.use(router);

app.mount("#app");

// Bootstrap session after mount to ensure store is ready
import { useSessionStore } from "./stores/session";
const sessionStore = useSessionStore();
void sessionStore.bootstrapSession();
