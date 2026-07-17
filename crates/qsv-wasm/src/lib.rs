//! # qsv-wasm — WebAssembly bindings for qsv
//!
//! A deliberately thin shim: `qsv` stays pure Rust with zero knowledge of
//! wasm-bindgen or the browser; this crate is the only place the two worlds
//! meet. The web frontend consumes this crate through the module at
//! `web/src/modules/sim/`.
//!
//! ## The zero-copy contract (established now, honored from Phase 1 on)
//!
//! State vectors get large (2^20 amplitudes at 20 qubits). Amplitude data is
//! therefore exposed to JavaScript as `Float64Array` *views* into wasm linear
//! memory — never serialized, never copied per frame. The JS side must treat
//! these views as ephemeral: any wasm allocation can grow memory and detach
//! them, so views are re-acquired each read and never stored.

use wasm_bindgen::prelude::*;

/// One-time setup. Routes Rust panics to the browser console so a crashed
/// simulator says why instead of "unreachable executed".
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Crate version, exposed so the frontend can prove the wasm module it loaded
/// is the one built from this workspace.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
