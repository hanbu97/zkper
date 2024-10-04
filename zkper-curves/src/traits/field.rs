use rand_core::RngCore;
use rug::Integer;

/// Trait to add behaviour to finite field.
pub trait FieldTrait {
    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random<R: RngCore>(rng: &mut R) -> Integer;
}
