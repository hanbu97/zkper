use rug::Integer;
use zkper_curves::curves::bls12_381::BLS12_381_SCALAR;
use zkper_groth16::{
    circuit::Circuit,
    constraints::{ConstraintSystem, Variable},
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

        println!("0: {:#?}", cs);

        // Allocate the first component of the preimage.
        let xl = cs.new_private()?;
        let mut xl_value = self.xl.clone();

        println!("1: {:#?}", cs);

        // Allocate the second component of the preimage.
        let xr = cs.new_private()?;
        let mut xr_value = self.xr.clone();

        println!("2: {:#?}", cs);

        for i in 0..2 {
            // tmp = (xL + Ci)^2
            let tmp_value = xl_value.clone().map(|mut e| {
                e = BLS12_381_SCALAR.add(e, &self.constants[i]);
                e = BLS12_381_SCALAR.square(e);
            });

            let tmp = if let Some(tmp) = tmp_value {
                Variable::Private(999)
            } else {
                cs.new_private()?
            };

            println!("{} --- 1: {:#?}", i, cs);
        }

        Ok(())
    }
}
