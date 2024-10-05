use crate::curves::bls12_381::BLS12_381_BASE;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bls12_381BaseField(pub Integer);

impl From<Integer> for Bls12_381BaseField {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl Display for Bls12_381BaseField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_radix(16))
    }
}

impl FieldTrait for Bls12_381BaseField {
    fn random<R: RngCore>(rng: &mut R) -> Integer {
        BLS12_381_BASE.sample_raw(rng)
    }
    fn modulus<'a>() -> &'a Integer {
        &&BLS12_381_BASE.modulus_ref()
    }
    fn r<'a>() -> &'a Integer {
        &BLS12_381_BASE.r_ref()
    }
    fn r2<'a>() -> &'a Integer {
        &BLS12_381_BASE.r2_ref()
    }
    fn limbs() -> usize {
        BLS12_381_BASE.limbs()
    }
    fn montgomery_multiply(a: &Integer, b: &Integer) -> Integer {
        BLS12_381_BASE.montgomery_multiply(a, b)
    }
}

impl Distribution<Bls12_381BaseField> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Bls12_381BaseField {
        let r_ref = Bls12_381BaseField::r();
        let r2_ref = Bls12_381BaseField::r2();
        let modulus = Bls12_381BaseField::modulus();

        let mut bytes = vec![0u8; 96];
        rng.fill_bytes(&mut bytes);

        let d0 = Integer::from_digits(&bytes[..48], rug::integer::Order::Msf);
        let d1 = Integer::from_digits(&bytes[48..], rug::integer::Order::Msf);

        let out = Bls12_381BaseField::montgomery_multiply(&d0, r_ref)
            + Bls12_381BaseField::montgomery_multiply(&d1, r2_ref);

        (out % modulus).into()
    }
}
