import {createApp} from "vue";
import App from "./App.vue";
import { createPinia } from 'pinia'
import ClickOutside from '@/utils/ClickOutside'
import router from "@/router/index.js";
// import './utils/tray.js'

const pinia = createPinia()
const app = createApp(App)
app.use(ClickOutside)
app.use(router)
app.use(pinia)
app.mount("#app");
