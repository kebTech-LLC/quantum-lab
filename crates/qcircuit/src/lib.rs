//! # qcircuit — circuit representation and gate definitions
//!
//! A quantum circuit as data: an ordered sequence of gate applications on
//! named qubits, with a builder API and a text diagram pretty-printer.
//! Circuits execute against the `qsv` state-vector simulator.
//!
//! This crate stays empty until Phase 3. Phases 1 and 2 apply gates directly
//! through `qsv` so the mechanics are understood before being abstracted.

#[cfg(test)]
mod tests {
    /// Phase 0 checkpoint: the workspace compiles and the test harness runs.
    #[test]
    fn scaffold_compiles() {}
}
