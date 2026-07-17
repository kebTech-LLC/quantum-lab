import { reactive } from "vue";

/**
 * Application data state. Reactive singletons, exported for direct import by
 * components — no props drilling, no Pinia. Grows as phases land (e.g. the
 * current register state, experiment results).
 */
class AppState {
  title = "quantum-lab";
}

export const app = reactive(new AppState());
