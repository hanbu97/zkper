use crate::{constraints::ConstraintSystem, prover::ProvingSystem};

/// Computations are expressed in terms of arithmetic circuits, in particular
/// rank-1 quadratic constraint systems. The `Circuit` trait represents a
/// circuit that can be synthesized. The `synthesize` method is called during
/// CRS generation and during proving.
pub trait Circuit {
    /// Synthesize the circuit into a rank-1 quadratic constraint system
    fn synthesize(&self, cs: &mut ConstraintSystem) -> anyhow::Result<()>;

    fn synthesize_proof(&self, cs: &mut ProvingSystem) -> anyhow::Result<()>;
}
