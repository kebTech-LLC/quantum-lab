# Roadmap

Phase tracker for quantum-lab. Updated as work progresses. Full phase
descriptions and checkpoints live in CLAUDE.md; this file records status,
dates, and what turned out to be surprising or hard.

Reading: Nielsen & Chuang, "Quantum Computation and Quantum Information"
(primary); Aaronson, "Quantum Computing Since Democritus" (companion).

## Status

| Phase | Topic | Status |
|-------|-------|--------|
| 0 | Scaffold: workspace, three crates, CI, GitHub | in progress |
| 1 | Single qubit: amplitudes, gates, measurement | not started |
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
