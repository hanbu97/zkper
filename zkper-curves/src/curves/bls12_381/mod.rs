use crate::backends::montgomery::MontgomeryBackend;

pub mod curves;
pub mod fields;

pub use fields::base::Bls12_381BaseField;
pub use fields::scalar::Bls12_381ScalarField;

lazy_static::lazy_static! {
    pub static ref BLS12_381_SCALAR: MontgomeryBackend = MontgomeryBackend::from_str_radix(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513", 10, 4
    );
    pub static ref BLS12_381_BASE: MontgomeryBackend = MontgomeryBackend::from_str_radix(
        "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787", 10, 6
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rug::Integer;
    use tests::fields::scalar::Bls12_381ScalarField;
    use zkper_base::rand::ZkperRng;

    #[test]
    fn test_sqrt_neg_square() {
        let four = Integer::from(4);

        let a_sqrt = BLS12_381_BASE.sqrt(four).unwrap();
        println!("a_sqrt: {:?}", a_sqrt.to_string_radix(16));

        let a_sqrt_square = BLS12_381_BASE.square(a_sqrt.clone());
        println!("a_sqrt_square: {:?}", a_sqrt_square.to_string_radix(16));

        let a_sqrt_neg = BLS12_381_BASE.neg(a_sqrt);
        println!("a_sqrt_neg: {:?}", a_sqrt_neg.to_string_radix(16));

        let a_sqrt_neg_square = BLS12_381_BASE.square(a_sqrt_neg);
        println!(
            "a_sqrt_neg_square: {:?}",
            a_sqrt_neg_square.to_string_radix(16)
        );
    }

    #[test]
    fn test_rng_gen() {
        let mut rng = ZkperRng::new_test();
        let scalar: Bls12_381ScalarField = rng.gen();
        println!("scalar: {}", scalar.0.to_string_radix(16));

        let mut rng = ZkperRng::new_test();
        let base: Bls12_381BaseField = rng.gen();
        println!("base: {}", base.0.to_string_radix(16));
    }

    #[test]
    fn test_sample() {
        let mut rng = ZkperRng::new_test();

        let scalar = super::BLS12_381_SCALAR.sample_raw(&mut rng);
        println!("scalar: {}", scalar.to_string_radix(16));

        let mut rng = ZkperRng::new_test();

        let base = super::BLS12_381_BASE.sample_raw(&mut rng);
        println!("base: {}", base.to_string_radix(16));
    }

    #[test]
    fn test_field_params() {
        let modulus = BLS12_381_SCALAR.modulus();
        let r = BLS12_381_SCALAR.r();
        let r_inv = BLS12_381_SCALAR.r_inv();
        let r2 = BLS12_381_SCALAR.r2();
        let inv = BLS12_381_SCALAR.inv();
        let limbs = BLS12_381_SCALAR.limbs();

        println!("modulus: {}", modulus.to_string_radix(16));
        println!("r: {}", r.to_string_radix(16));
        println!("r_inv : {}", r_inv.to_string_radix(16));
        println!("r2: {}", r2.to_string_radix(16));
        println!("inv: {}", inv);
        println!("limbs: {}", limbs);

        let mut rng = ZkperRng::new_test();
        let scalar = BLS12_381_SCALAR.sample(&mut rng);
        println!("scalar: {}", scalar.to_string_radix(16));
        let scalar_from_mont = BLS12_381_SCALAR.from_montgomery(&scalar);
        println!("scalar_from_mont: {}", scalar_from_mont.to_string_radix(16));

        println!("------------------------------------");

        let modulus_base = BLS12_381_BASE.modulus();
        let r_base = BLS12_381_BASE.r();
        let r_inv_base = BLS12_381_BASE.r_inv();
        let r2_base = BLS12_381_BASE.r2();
        let inv_base = BLS12_381_BASE.inv();
        let limbs_base = BLS12_381_BASE.limbs();

        println!("modulus_base: {}", modulus_base.to_string_radix(16));
        println!("r_base: {}", r_base.to_string_radix(16));
        println!("r_inv_base : {}", r_inv_base.to_string_radix(16));
        println!("r2_base: {}", r2_base.to_string_radix(16));
        println!("inv_base: {}", inv_base);
        println!("limbs_base: {}", limbs_base);

        let mut rng = ZkperRng::new_test();
        let base = BLS12_381_BASE.sample(&mut rng);
        println!("base: {}", base.to_string_radix(16));
        let base_from_mont = BLS12_381_BASE.from_montgomery(&base);
        println!("base_from_mont: {}", base_from_mont.to_string_radix(16));
    }
}
