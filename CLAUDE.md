# Quantum Lab

Long-term personal project: build deep, working understanding of quantum computing
by implementing it from first principles in Rust. This is a learning laboratory,
not a product. The goal is expertise and critical thinking, not employability or
commercial output. Timeline is measured in years and that is fine.

Owner: Kyle Ebner. Standalone project — deliberately outside the cnctd and kebTech
monorepo structures. All crates public (GitHub), dual-licensed MIT/Apache-2.0.

## Guiding principles

1. Build before you use. Implement every concept from scratch in the simulator
   before touching any SDK (Qiskit, Cirq, etc.). SDKs come later, as a bridge to
   real hardware, never as the learning vehicle.
2. Formalism over intuition. When intuition and the math disagree, the math wins.
   Every phase ends with a verifiable checkpoint — correct amplitudes, correct
   probabilities — not a vibes-based "I think I get it."
3. Pair code with theory. Primary text: Nielsen & Chuang, "Quantum Computation
   and Quantum Information." Companion: Aaronson, "Quantum Computing Since
   Democritus." Each roadmap phase maps to reading.
4. No premature abstraction. Start with the dumbest correct implementation
   (dense state vector, naive gate application). Optimize only when a phase
   demands it and only after the naive version is verified.

## Repository layout

    ~/Development/quantum/
        quantum-lab/            # main cargo workspace (this repo)
            Cargo.toml          # workspace root
            CLAUDE.md           # this file
            ROADMAP.md          # phase tracker, updated as work progresses
            crates/
                qsv/            # state-vector simulator core (the heart)
                qcircuit/       # circuit representation, gate definitions, DSL
                qlab/           # binary: experiments, exercises, worked examples
                qsv-wasm/       # wasm-bindgen shim over qsv (no physics here)
            web/                # Vue 3 + Vite + TS frontend (see below)
            notes/              # markdown notes per topic, written as I learn

    Future sibling repos (do not create yet):
        quantum-hw/             # IBM hardware bridge (Qiskit C API or QRMI, later)
        anything standalone that outgrows the workspace

## Conventions

- Rust, 4-space indentation, no emojis in code or comments.
- Commit and push often. Small commits per concept/checkpoint.
- Each crate gets real doc comments — writing the explanation is part of learning.
- Tests are the checkpoints. A phase is not done until its tests pass and the
  results match hand-calculated or textbook values.
- Keep ROADMAP.md current: mark phases done, note what was surprising or hard.

## Technical starting decisions

- Complex numbers: use the `num-complex` crate (Complex64). Do not hand-roll.
- State representation: `Vec<Complex64>` of length 2^n, index i encodes basis
  state |i> in binary (qubit 0 = least significant bit; document this convention
  prominently, it is the source of endless off-by-one confusion).
- Gate application: start naive — iterate over the state vector applying the
  2x2 (or 4x4) unitary to the affected amplitude pairs. No matrix exponentials,
  no Kronecker-product-the-whole-system (that is O(4^n) memory; the pair-wise
  method is the standard trick and understanding WHY it works is itself a
  checkpoint).
- Measurement: sample from |amplitude|^2, then collapse and renormalize.
- RNG: `rand` crate, seedable for reproducible experiments.
- Practical qubit ceiling for a dense simulator on the Mac Studio (128 GB):
  ~30 qubits (2^30 Complex64 = 16 GB). Design for n as a runtime parameter.

## Web app and visualization architecture

Decided 2026-07-17. The lab is web-first: the Rust core compiles to WebAssembly
and runs in-browser; there is no backend server unless a future phase genuinely
needs one (25+ qubit runs, long sweeps — add axum then, not before).

- Dependency arrow points ONE way: web -> qsv-wasm -> qsv. The physics crates
  never know wasm-bindgen or the browser exist. If the frontend needs a
  quantum computation, it goes in qsv and is exposed through the shim.
- qsv-wasm is a thin cdylib shim, the only crate touching wasm-bindgen.
- Zero-copy contract: amplitude data is exposed to JS as Float64Array views
  into wasm linear memory — re-acquired each read, never stored (memory growth
  detaches views), never serialized per frame.
- Frontend: Vue 3 + Vite + TypeScript, no UI framework, latest stable
  versions of all packages (upgrade freely; this repo tracks latest).
- Frontend patterns are Kyle's proven cnctd.world architecture — four layers
  (components/views/models/state+modules), reactive class singletons, Vue
  files as thin reactive remotes with ALL business logic in TS files.
  Full rules: .claude/rules/frontend.md, enforced by .claude/hooks/.
- Build: web/ npm scripts drive wasm-pack (pinned as npm devDependency);
  generated pkg/ is gitignored. `npm run build` = wasm + typecheck + bundle.
- The qlab CLI remains the fastest loop for checkpoint tests; the web app is
  the visualization surface (Bloch sphere, amplitude bars, histograms). First
  real UI lands after Phase 1 checkpoints pass, as its own milestone.

## Roadmap

### Phase 0 — Scaffold
Workspace + three crates compile, CI-less but git-initialized, pushed to GitHub
under a new personal org or the personal account (decide at scaffold time).

### Phase 1 — Single qubit (N&C ch. 1-2 selections)
- Complex amplitudes, normalization, Dirac notation in doc comments.
- Gates: X, Y, Z, H, S, T, and general single-qubit rotation.
- Measurement in the computational basis.
- Checkpoint: H|0> measured 10k times gives ~50/50; HZH = X verified
  numerically; global phase demonstrably unobservable.

### Phase 2 — Multi-qubit and entanglement
- Tensor-product state space, the indexing convention, applying single-qubit
  gates to qubit k of n.
- Two-qubit gates: CNOT, CZ, SWAP, controlled-U.
- Checkpoint: Bell state construction, GHZ state {000: ~50%, 111: ~50%},
  partial measurement collapses the partner qubit correctly.
- Concept checkpoints (write up in notes/): no-cloning (show the math refuses),
  why entangled states cannot be factored.

### Phase 3 — Circuits proper (qcircuit crate)
- Circuit as data: sequence of gate applications, builder API.
- Circuit execution against qsv; circuit diagram pretty-printer (text).
- Checkpoint: quantum teleportation implemented and verified end to end.
  Superdense coding as a second exercise.

### Phase 4 — Canonical algorithms
- Deutsch-Jozsa, Bernstein-Vazirani, Grover (with the amplitude picture
  documented — WHY quadratic, not exponential), QFT, phase estimation.
- Stretch: Shor on toy numbers (factor 15) via period finding.
- Checkpoint: each algorithm's success probability matches theory.

### Phase 5 — Noise and open systems
- Density-matrix representation OR trajectory/Kraus-operator sampling on the
  state vector (decide when there; trajectories scale better).
- Bit flip, phase flip, depolarizing, amplitude damping channels; T1/T2 style
  decay.
- Checkpoint: watch a Bell state's entanglement degrade under noise; reproduce
  the qualitative behavior of real-hardware error rates.

### Phase 6 — Error correction
- 3-qubit bit-flip and phase-flip codes, Shor 9-qubit code, stabilizer
  formalism intro. Stretch goal: toy surface-code intuition.
- Checkpoint: inject errors, detect and correct them, show logical error rate
  beats physical error rate below threshold.

### Phase 7 — Real hardware (new repo: quantum-hw)
- IBM Quantum free/paid tier. Run the SAME circuits from Phases 2-4 on real
  QPUs. Compare against the Phase 5 noise models.
- Rust path: Qiskit C API FFI or QRMI, with Python-Qiskit as the pragmatic
  fallback for anything the Rust path makes painful.
- Checkpoint: side-by-side simulator-vs-hardware results for Bell, GHZ, and
  Grover; a written analysis of the gap.

### Beyond (unordered)
Quantum information theory, complexity (BQP and friends), interpretations,
variational algorithms (VQE/QAOA), tensor-network simulation as a performance
project, quantum machine learning skepticism review.

## Working with Claude Code on this repo

- At session start: read this file and ROADMAP.md, then `git pull`.
- Keep a running progress note in ROADMAP.md during large multi-step changes
  (sessions can crash; the doc is the recovery point).
- Commit and push at every passing checkpoint.
- When implementing a concept, prefer explaining it in doc comments and
  notes/ over just writing code — the writing is the point.
- Do not add dependencies beyond num-complex, rand, and (when needed for qlab
  output) something minimal for tables/plots, without discussing first.
- Never skip a checkpoint test to move faster. The tests are the curriculum.
