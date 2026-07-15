# quantum-lab

A long-term learning laboratory: quantum computing implemented from first
principles in Rust. A dense state-vector simulator, circuit layer, and
experiment harness, built concept by concept alongside Nielsen & Chuang —
no SDKs until the fundamentals exist here first.

This is a personal study project, not a product. The code favors the dumbest
correct implementation, heavy doc comments, and tests that reproduce
hand-calculated or textbook values. See [ROADMAP.md](ROADMAP.md) for where
things stand.

## Crates

- `qsv` — state-vector simulator core (amplitudes, gates, measurement)
- `qcircuit` — circuit representation, gate definitions, builder API
- `qlab` — binary: experiments, exercises, worked examples

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE),
at your option.
