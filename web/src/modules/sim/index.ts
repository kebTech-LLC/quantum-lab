import { reactive } from "vue";
import init, { version } from "./pkg/qsv_wasm";

/**
 * The simulator module: sole owner of the qsv-wasm boundary.
 *
 * Everything the frontend knows about quantum state comes through this
 * singleton. Components never import from ./pkg directly — they bind to
 * reactive properties here and call methods here.
 *
 * Zero-copy rule (matters from Phase 1 on): amplitude data crosses the wasm
 * boundary as Float64Array views into wasm linear memory. Views are re-acquired
 * on every read and never stored — any wasm allocation can grow memory and
 * silently detach an old view.
 */
class Sim {
  ready = false;
  wasmVersion = "";
  error = "";

  async init(): Promise<void> {
    try {
      await init();
      this.wasmVersion = version();
      this.ready = true;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    }
  }
}

export const sim = reactive(new Sim());
