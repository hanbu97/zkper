use zkper_base::math::factorization::primitive_root;

use super::*;
use crate::curves::bls12_381::BLS12_381_SCALAR;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bls12_381ScalarField(pub Integer);

lazy_static::lazy_static! {
    /// 2^s root of unity computed by GENERATOR^t
    pub static ref TWO_ADIC_ROOT_OF_UNITY: Integer = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
}

#[test]
fn test_two_adic_root_of_unity() {
    // let modulus = Bls12_381ScalarField::modulus();
    let modulus = Integer::from_str(
        "52435875175126190479447",
        // "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    println!("modulus: {}", modulus.to_string_radix(10)); // 52435875175126190479447740508185965837690552500527637822603658699938581184513

    // // Calculate two-adicity
    // // find_one() Returns the location of the first one, starting at `start`. If the bit
    // // at location `start` is one, returns `start`.
    // let two_adicity = (modulus.clone() - Integer::ONE).find_one(0).unwrap();
    // println!("two_adicity: {}", two_adicity); // 32

    // // Calculate trace
    // let trace = (modulus.clone() - 1) >> two_adicity;

    // Find generator using the primitive_root function
    // very slow, but works
    let (generator, factors) = primitive_root(&modulus, None).unwrap();
    println!("Generator: {}", generator);
}

impl Bls12_381ScalarField {
    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    pub const TWO_ADICITY: u32 = 32;

    pub fn two_adic_root_of_unity() -> Integer {
        TWO_ADIC_ROOT_OF_UNITY.clone()
    }
}

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
    fn random_mont<R: RngCore>(rng: &mut R) -> Integer {
        BLS12_381_SCALAR.sample_mont(rng)
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
    fn cubic(input: Integer) -> Integer {
        BLS12_381_SCALAR.cubic(input)
    }
    fn to_mont(&self) -> Integer {
        BLS12_381_SCALAR.to_montgomery(&self.0)
    }
    fn from_mont(input: &Integer) -> Integer {
        BLS12_381_SCALAR.from_montgomery(input)
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
