use super::*;
use crate::curves::bls12_381::BLS12_381_SCALAR;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bls12_381ScalarField(pub Integer);

impl From<Integer> for Bls12_381ScalarField {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl Display for Bls12_381ScalarField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_radix(16))
    }
}

impl FieldTrait for Bls12_381ScalarField {
    fn random<R: RngCore>(rng: &mut R) -> Integer {
        BLS12_381_SCALAR.sample_raw(rng)
    }
    fn modulus<'a>() -> &'a Integer {
        &&BLS12_381_SCALAR.modulus_ref()
    }
    fn r<'a>() -> &'a Integer {
        &BLS12_381_SCALAR.r_ref()
    }
    fn r2<'a>() -> &'a Integer {
        &BLS12_381_SCALAR.r2_ref()
    }
    fn limbs() -> usize {
        BLS12_381_SCALAR.limbs()
    }
    fn mont_mul(a: &Integer, b: &Integer) -> Integer {
        BLS12_381_SCALAR.mont_mul(a, b)
    }
    fn to_mont(&self) -> Integer {
        BLS12_381_SCALAR.to_montgomery(&self.0)
    }
    fn from_mont(&self) -> Integer {
        BLS12_381_SCALAR.from_montgomery(&self.0)
    }
}

impl Distribution<Bls12_381ScalarField> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Bls12_381ScalarField {
        let r_ref = Bls12_381ScalarField::r();
        let r2_ref = Bls12_381ScalarField::r2();
        let modulus = Bls12_381ScalarField::modulus();

        let mut bytes = vec![0u8; 64];
        rng.fill_bytes(&mut bytes);

        let d0 = Integer::from_digits(&bytes[..32], rug::integer::Order::Lsf);
        let d1 = Integer::from_digits(&bytes[32..], rug::integer::Order::Lsf);

        let out = Bls12_381ScalarField::mont_mul(&d0, r_ref)
            + Bls12_381ScalarField::mont_mul(&d1, r2_ref);

        (out % modulus).into()
    }
}
