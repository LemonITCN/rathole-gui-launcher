import { createApp } from "vue";
import { createPinia } from "pinia";
import Antd from "ant-design-vue";
import App from "./App.vue";
import { router } from "./router";
import { i18n } from "./i18n";
import "./styles/index.less";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);
app.use(Antd);
app.mount("#app");
