use rand_core::RngCore;
use rug::Integer;

/// Trait to add behaviour to finite field.
pub trait FieldTrait {
    /// Returns an element chosen uniformly at random using a user-provided RNG.
    fn random<R: RngCore>(rng: &mut R) -> Integer;

    /// Returns the modulus of the field.
    fn modulus<'a>() -> &'a Integer;

    /// Returns r ref.
    fn r<'a>() -> &'a Integer;

    /// Returns r2 ref.
    fn r2<'a>() -> &'a Integer;

    /// Returns the number of limbs.
    fn limbs() -> usize;

    /// to montgomery form
    fn to_mont(&self) -> Integer;

    /// from montgomery form
    fn from_mont(input: &Integer) -> Integer;

    /// montgomery_multiply
    fn mont_mul(a: &Integer, b: &Integer) -> Integer;

    /// zero
    fn zero() -> Integer {
        Integer::from(0)
    }

    /// one
    fn one() -> Integer {
        Integer::from(1)
    }
}
