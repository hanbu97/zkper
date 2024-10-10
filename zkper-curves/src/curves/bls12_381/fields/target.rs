use std::fmt::{self, Display};

use rug::Integer;

use super::fp12::Fp12;

/// Represents an element of the target group of the pairing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetField(pub Fp12);

impl Display for TargetField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TargetField {
    /// Returns the multiplicative identity element of TargetField.
    pub fn one() -> Self {
        TargetField(Fp12::one())
    }
    pub fn identity() -> Self {
        TargetField(Fp12::one())
    }

    pub fn is_identity(&self) -> bool {
        self.0.is_one()
    }

    /// Returns the additive identity element of TargetField.
    pub fn zero() -> Self {
        TargetField(Fp12::zero())
    }

    /// Checks if the element is zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Raises this element to p.
    pub fn frobenius_map(&self) -> Self {
        TargetField(self.0.frobenius_map())
    }

    /// Computes the square of this element.
    pub fn square(&self) -> Self {
        TargetField(self.0.square())
    }

    /// Doubles this group element.
    pub fn double(&self) -> Self {
        TargetField(self.0.square())
    }

    /// Neg
    pub fn neg(&self) -> Self {
        TargetField(self.0.conjugate())
    }

    /// Add
    pub fn add(&self, other: &Self) -> Self {
        TargetField(self.0.mul(&other.0))
    }

    /// Sub
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// Mul
    /// Scalar multiplication of this element.
    pub fn mul_scalar(&self, scalar: &Integer) -> Self {
        if scalar.is_zero() {
            return Self::identity();
        }

        let mut result = Self::identity();
        let mut temp = self.clone();
        let mut scalar_bits = scalar.clone();

        while !scalar_bits.is_zero() {
            if scalar_bits.is_odd() {
                result = result.add(&temp);
            }
            temp = temp.double();
            scalar_bits >>= 1;
        }

        result
    }
}
