import { reactive } from "vue";

/**
 * UI layout state: which panels are open, selections, view modes. Models and
 * components read and write this; it holds no application data of its own.
 */
class Views {
  activePanel: "lab" | "notes" = "lab";
}

export const views = reactive(new Views());
