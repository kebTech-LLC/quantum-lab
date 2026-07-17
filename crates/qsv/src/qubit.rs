//! The single-qubit state: two complex amplitudes.
//!
//! This module is Phase 1's foundation. Everything a single qubit *is* lives
//! here; everything you can *do* to one (gates, measurement) lives in sibling
//! modules and operates on this type.

use num_complex::Complex64;

/// Tolerance for treating a floating-point norm as exactly 1. Amplitudes are
/// f64 pairs, so norms drift by ~1e-16 per arithmetic op; 1e-10 catches real
/// mistakes while ignoring accumulated rounding.
pub const NORM_TOLERANCE: f64 = 1e-10;

/// A pure single-qubit state.
///
/// # The math
///
/// A qubit state is a unit vector in a 2-dimensional complex vector space.
/// In Dirac notation:
///
/// ```text
/// |psi> = a0|0> + a1|1>        a0, a1 in C,   |a0|^2 + |a1|^2 = 1
/// ```
///
/// `|0>` and `|1>` are the computational basis states — the reference
/// directions. The amplitudes `a0`, `a1` are the coordinates of one definite
/// arrow in that space. "Superposition" means nothing more exotic than: both
/// coordinates are nonzero.
///
/// # The Born rule
///
/// Measuring in the computational basis yields outcome 0 with probability
/// `|a0|^2` and outcome 1 with probability `|a1|^2`. Probabilities are the
/// *squared magnitudes* of amplitudes — this is the only place probability
/// enters quantum mechanics, and it is why the normalization constraint
/// exists: the outcomes must exhaust probability 1.
///
/// # What the probabilities do NOT capture
///
/// Amplitudes are complex, so a state carries more information than its two
/// probabilities. `(|0> + |1>)/sqrt(2)` and `(|0> - |1>)/sqrt(2)` both give
/// 50/50 measurement odds, yet they are different states — a Hadamard gate
/// maps the first to `|0>` and the second to `|1>`, perfectly
/// distinguishable. The relative phase between amplitudes is physical.
/// (The *global* phase is not — a Phase 1 checkpoint proves this.)
#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    amps: [Complex64; 2],
}

impl Qubit {
    /// Construct a state from raw amplitudes.
    ///
    /// # Panics
    ///
    /// Panics if `|a0|^2 + |a1|^2` deviates from 1 by more than
    /// [`NORM_TOLERANCE`]. A non-normalized state is not a valid quantum
    /// state, and in a learning lab the right response to constructing one
    /// is a loud failure, not a silent renormalization that hides the bug.
    pub fn new(a0: Complex64, a1: Complex64) -> Self {
        let norm_sqr = a0.norm_sqr() + a1.norm_sqr();
        assert!(
            (norm_sqr - 1.0).abs() <= NORM_TOLERANCE,
            "amplitudes not normalized: |a0|^2 + |a1|^2 = {norm_sqr}"
        );
        Self { amps: [a0, a1] }
    }

    /// The basis state `|0>`: amplitudes (1, 0).
    pub fn zero() -> Self {
        Self::new(Complex64::ONE, Complex64::ZERO)
    }

    /// The basis state `|1>`: amplitudes (0, 1).
    pub fn one() -> Self {
        Self::new(Complex64::ZERO, Complex64::ONE)
    }

    /// The amplitude of `|0>`.
    pub fn amp0(&self) -> Complex64 {
        self.amps[0]
    }

    /// The amplitude of `|1>`.
    pub fn amp1(&self) -> Complex64 {
        self.amps[1]
    }

    /// Born rule: probability that measurement yields 0.
    pub fn prob_zero(&self) -> f64 {
        self.amps[0].norm_sqr()
    }

    /// Born rule: probability that measurement yields 1.
    pub fn prob_one(&self) -> f64 {
        self.amps[1].norm_sqr()
    }

    /// Amplitude-wise approximate equality, for numerical checkpoint tests.
    ///
    /// Note this is *stricter* than physical equality: two states differing
    /// only by global phase are physically identical but compare unequal
    /// here. That distinction is exactly what the global-phase checkpoint
    /// will exercise.
    pub fn approx_eq(&self, other: &Self, tol: f64) -> bool {
        (self.amps[0] - other.amps[0]).norm() <= tol && (self.amps[1] - other.amps[1]).norm() <= tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn basis_states_have_certain_outcomes() {
        let zero = Qubit::zero();
        assert_eq!(zero.prob_zero(), 1.0);
        assert_eq!(zero.prob_one(), 0.0);

        let one = Qubit::one();
        assert_eq!(one.prob_zero(), 0.0);
        assert_eq!(one.prob_one(), 1.0);
    }

    #[test]
    fn equal_superposition_is_fifty_fifty() {
        // (|0> + |1>)/sqrt(2) — the state H|0> will produce in the next step.
        let plus = Qubit::new(
            Complex64::new(FRAC_1_SQRT_2, 0.0),
            Complex64::new(FRAC_1_SQRT_2, 0.0),
        );
        assert!((plus.prob_zero() - 0.5).abs() < 1e-12);
        assert!((plus.prob_one() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn complex_amplitudes_hand_check() {
        // Hand-verified: a0 = 0.6, a1 = 0.8i.
        // |a0|^2 = 0.36, |a1|^2 = |0.8i|^2 = 0.64, sum = 1. Valid state.
        // The i contributes nothing to the probability — magnitude only.
        let q = Qubit::new(Complex64::new(0.6, 0.0), Complex64::new(0.0, 0.8));
        assert!((q.prob_zero() - 0.36).abs() < 1e-12);
        assert!((q.prob_one() - 0.64).abs() < 1e-12);
    }

    #[test]
    #[should_panic(expected = "not normalized")]
    fn unnormalized_state_is_rejected() {
        Qubit::new(Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0));
    }

    #[test]
    fn phase_differs_but_probabilities_agree() {
        // |+> = (|0> + |1>)/sqrt(2) and |-> = (|0> - |1>)/sqrt(2):
        // identical probabilities, different states. The probabilities
        // cannot tell them apart; the amplitudes can.
        let plus = Qubit::new(
            Complex64::new(FRAC_1_SQRT_2, 0.0),
            Complex64::new(FRAC_1_SQRT_2, 0.0),
        );
        let minus = Qubit::new(
            Complex64::new(FRAC_1_SQRT_2, 0.0),
            Complex64::new(-FRAC_1_SQRT_2, 0.0),
        );
        assert_eq!(plus.prob_zero(), minus.prob_zero());
        assert_eq!(plus.prob_one(), minus.prob_one());
        assert!(!plus.approx_eq(&minus, 1e-12));
    }
}
