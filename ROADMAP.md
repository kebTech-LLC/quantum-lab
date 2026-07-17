# Roadmap

Phase tracker for quantum-lab. Updated as work progresses. Full phase
descriptions and checkpoints live in CLAUDE.md; this file records status,
dates, and what turned out to be surprising or hard.

Reading: Nielsen & Chuang, "Quantum Computation and Quantum Information"
(primary); Aaronson, "Quantum Computing Since Democritus" (companion).

## Status

| Phase | Topic | Status |
|-------|-------|--------|
| 0 | Scaffold: workspace, three crates, CI, GitHub | done (2026-07-15) |
| 1 | Single qubit: amplitudes, gates, measurement | in progress |
| 2 | Multi-qubit and entanglement | not started |
| 3 | Circuits proper (qcircuit crate) | not started |
| 4 | Canonical algorithms: DJ, BV, Grover, QFT, phase estimation | not started |
| 5 | Noise and open systems | not started |
| 6 | Error correction | not started |
| 7 | Real hardware (new repo: quantum-hw) | not started |

## Log

### 2026-07-15 — Phase 0
- Workspace scaffolded: qsv, qcircuit, qlab crates.
- Decision: repo lives under the kebTech-LLC GitHub org.
- Decision: CI from day one (fmt, clippy, test on every push).
- Pushed to https://github.com/kebTech-LLC/quantum-lab; first CI run green.
- Phase 0 complete. Next: Phase 1, single-qubit state and gates in qsv.

### 2026-07-17 — Phase 0 addendum: web-first architecture
- Decision: web-first lab, modeled on cnctd.world's proven frontend patterns.
  qsv compiles to WASM (qsv-wasm shim crate), Vue 3 + Vite + TS frontend in
  web/, no backend server until a phase genuinely needs one.
- Scaffolded qsv-wasm (wasm-bindgen 0.2.126) and web/ (Vue 3.5, Vite 8,
  vue-tsc 3, wasm-pack 0.15 pinned via npm). TS held at 5.9: vue-tsc does
  not yet support the TS 7 native compiler (no lib/tsc) — bump when it does.
- Gotcha for posterity: getrandom 0.3 on wasm32 needs the wasm_js feature
  (qsv-wasm Cargo.toml) AND --cfg getrandom_backend="wasm_js" rustflag
  (.cargo/config.toml) or rand fails to compile for the browser.
- Ported cnctd.world frontend rules + validation hooks into .claude/
  (template-before-script, no business logic in components — enforced).
- CI grew a web job: wasm-pack build + vue-tsc + vite build.
- First real UI (Bloch sphere, amplitude bars) is its own milestone after
  Phase 1 checkpoints pass. Until then qlab CLI is the primary loop.
