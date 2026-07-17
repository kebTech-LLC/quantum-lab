import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { sim } from "@/modules/sim";

sim.init();

createApp(App).mount("#app");
