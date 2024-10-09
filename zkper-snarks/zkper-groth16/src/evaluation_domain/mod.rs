use rug::Integer;
use zkper_curves::curves::bls12_381::{Bls12_381ScalarField, BLS12_381_SCALAR};

#[derive(Debug)]
pub struct EvaluationDomain {
    pub coeffs: Vec<Integer>,
    pub exp: u32,
    pub omega: Integer,
    pub omegainv: Integer,
    pub geninv: Integer,
    pub minv: Integer,
}

impl EvaluationDomain {
    pub fn new(mut coeffs: Vec<Integer>) -> anyhow::Result<Self> {
        // Compute the size of our evaluation domain
        let needed_size = coeffs.len().next_power_of_two();
        let exp = needed_size.trailing_zeros();

        // The pairing-friendly curve may not be able to support
        // large enough (radix2) evaluation domains.
        if exp >= Bls12_381ScalarField::TWO_ADICITY {
            return Err(anyhow::anyhow!("radix2 evaluation domain too large"));
        }

        // Compute omega, the 2^exp primitive root of unity
        let mut omega = Bls12_381ScalarField::two_adic_root_of_unity();
        for _ in exp..Bls12_381ScalarField::TWO_ADICITY {
            omega = BLS12_381_SCALAR.square(omega);
        }

        // Extend the coeffs vector with zeroes if necessary
        coeffs.resize(needed_size, Integer::from(0));

        Ok(EvaluationDomain {
            coeffs,
            exp,
            omega: omega.clone(),
            omegainv: BLS12_381_SCALAR.invert(omega.clone()).unwrap(),
            geninv: BLS12_381_SCALAR
                .invert(Bls12_381ScalarField::MULTIPLICATIVE_GENERATOR.clone())
                .unwrap(),
            minv: BLS12_381_SCALAR.invert(Integer::from(needed_size)).unwrap(),
        })
    }
}
