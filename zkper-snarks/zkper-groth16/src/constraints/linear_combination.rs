use rug::Integer;
use zkper_curves::curves::bls12_381::BLS12_381_SCALAR;

use super::Variable;

/// This represents a linear combination of some variables, with coefficients
/// in the scalar field of a pairing-friendly elliptic curve group.
#[derive(Clone, Debug)]
pub struct LinearCombination(pub Vec<(Variable, Integer)>);

impl LinearCombination {
    /// Create a new empty linear combination.
    pub fn zero() -> Self {
        LinearCombination(Vec::new())
    }

    pub fn add(mut self, coeff_var: (Variable, Integer)) -> Self {
        self.0.push(coeff_var);
        self
    }

    pub fn sub(mut self, coeff_var: (Variable, Integer)) -> Self {
        self.0
            .push((coeff_var.0, BLS12_381_SCALAR.neg(coeff_var.1)));
        self
    }

    pub fn add_one(mut self, var: Variable) -> Self {
        self.0.push((var, Integer::from(1)));
        self
    }

    pub fn sub_one(self, var: Variable) -> Self {
        self.sub((var, Integer::from(1)))
    }

    pub fn add_linear_combination(mut self, other: LinearCombination) -> Self {
        for (var, coeff) in other.0 {
            self = self.add((var, coeff));
        }
        self
    }

    pub fn sub_linear_combination(mut self, other: LinearCombination) -> Self {
        for (var, coeff) in other.0 {
            self = self.sub((var, coeff));
        }
        self
    }

    // Implementation for adding a scaled LinearCombination to another LinearCombination
    // This allows operations of the form:
    // (LinearCombination) + (Scalar * LinearCombination)
    pub fn add_scaled(mut self, scalar: Integer, other: LinearCombination) -> Self {
        for (var, coeff) in other.0 {
            self = self.add((var, BLS12_381_SCALAR.mul(coeff, &scalar)));
        }
        self
    }

    pub fn sub_scaled(mut self, scalar: Integer, other: LinearCombination) -> Self {
        for (var, coeff) in other.0 {
            self = self.sub((var, BLS12_381_SCALAR.mul(coeff, &scalar)));
        }
        self
    }
}

#[test]
fn test_neg() {
    let t = Integer::from(1);
    let t_neg = BLS12_381_SCALAR.neg(t.clone());

    println!("t_neg: {}", t_neg.to_string_radix(16));
}
