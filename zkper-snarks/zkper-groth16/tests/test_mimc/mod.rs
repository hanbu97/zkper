use std::str::FromStr;

use rug::Integer;
use zkper_curves::curves::bls12_381::BLS12_381_SCALAR;
use zkper_groth16::{
    circuit::Circuit,
    constraints::{linear_combination::LinearCombination, ConstraintSystem},
};

use crate::MIMC_ROUNDS;

pub struct MiMCDemo<'a> {
    pub xl: Option<Integer>,
    pub xr: Option<Integer>,
    pub constants: &'a [Integer],
}

impl Circuit for MiMCDemo<'_> {
    /// Generate the constraints for the MiMC circuit
    fn synthesize(&self, cs: &mut ConstraintSystem) -> anyhow::Result<()> {
        assert_eq!(self.constants.len(), MIMC_ROUNDS);

        // Allocate the first component of the preimage.
        let mut xl = cs.new_private()?;
        let mut xl_value = self.xl.clone();

        // Allocate the second component of the preimage.
        let mut xr = cs.new_private()?;
        let mut xr_value = self.xr.clone();

        for i in 0..MIMC_ROUNDS {
            // tmp = (xL + Ci)^2
            let tmp_value = xl_value.clone().map(|mut e| {
                e = BLS12_381_SCALAR.add(e, &self.constants[i]);
                let t = BLS12_381_SCALAR.square(e);

                println!("t: {}", t.to_string_radix(16));

                t
            });

            let tmp = cs.new_private()?;

            let a = LinearCombination::zero()
                .add((xl, Integer::from(1)))
                .add((ConstraintSystem::one(), self.constants[i].clone()));
            let b = LinearCombination::zero()
                .add((xl, Integer::from(1)))
                .add((ConstraintSystem::one(), self.constants[i].clone()));
            let c = LinearCombination::zero().add((tmp, Integer::from(1)));
            cs.enforce_constraint(a, b.clone(), c.clone());

            // new_xL = xR + (xL + Ci)^3
            // new_xL = xR + tmp * (xL + Ci)
            // new_xL - xR = tmp * (xL + Ci)
            let new_xl_value = xl_value.clone().map(|mut e| {
                e = BLS12_381_SCALAR.add(e, &self.constants[i]);
                e = BLS12_381_SCALAR.mul(e, &tmp_value.clone().unwrap());
                e = BLS12_381_SCALAR.add(e, &xr_value.clone().unwrap());
                e
            });

            let new_xl = if i == (MIMC_ROUNDS - 1) {
                // This is the last round, xL is our image and so
                // we allocate a public input.
                cs.new_public()?
            } else {
                cs.new_private()?
            };

            let a = c;
            let b = b;
            let c = LinearCombination::zero()
                .add_variable(new_xl)
                .sub_variable(xr);
            cs.enforce_constraint(a, b, c);

            // xR = xL
            xr = xl;
            xr_value = xl_value;

            // xL = new_xL
            xl = new_xl;
            xl_value = new_xl_value;
        }

        Ok(())
    }
}
