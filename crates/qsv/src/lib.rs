//! # qsv — dense state-vector quantum simulator
//!
//! The heart of the quantum-lab workspace. Everything here is implemented from
//! first principles: no SDK, no black boxes. The simulator represents an
//! n-qubit pure state as a dense vector of 2^n complex amplitudes.
//!
//! ## The indexing convention (read this first)
//!
//! The state is a `Vec<Complex64>` of length 2^n. Index `i`, written in binary,
//! encodes the computational basis state |i>, and **qubit 0 is the least
//! significant bit**.
//!
//! For a 3-qubit system, index 5 = 0b101 is the basis state where qubit 0 = 1,
//! qubit 1 = 0, qubit 2 = 1. In ket notation with the highest qubit written
//! leftmost, that is |101> = |q2 q1 q0>.
//!
//! This is the little-endian convention (the one Qiskit also uses). Most
//! textbooks, including Nielsen & Chuang, write states big-endian, so a
//! textbook |01> may be this simulator's index 2, not index 1. Every indexing
//! bug in this project will trace back to this paragraph.
//!
//! Phase 1 (single qubit) begins here: amplitudes, normalization, the standard
//! gates, and measurement in the computational basis.

#[cfg(test)]
mod tests {
    /// Phase 0 checkpoint: the workspace compiles and the test harness runs.
    #[test]
    fn scaffold_compiles() {}
}
