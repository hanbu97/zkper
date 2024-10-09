use std::str::FromStr;

use crate::curves::bls12_381::BLS12_381_BASE;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bls12_381BaseField(pub Integer);

impl Bls12_381BaseField {
    pub fn from_u64_hex_str_vec(hex_str_vec: &[&str]) -> Integer {
        let u64_vec = hex_str_vec
            .iter()
            .map(|x| u64::from_str_radix(x.strip_prefix("0x").unwrap_or(x), 16).unwrap())
            .collect::<Vec<_>>();

        let value = Integer::from_digits::<u64>(&u64_vec, rug::integer::Order::Lsf);
        value
    }

    pub fn from_u64_vec(u64_vec: &[u64]) -> Integer {
        let value = Integer::from_digits::<u64>(u64_vec, rug::integer::Order::Lsf);
        value
    }
}

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

impl Bls12_381BaseField {
    pub fn square(input: Integer) -> Integer {
        BLS12_381_BASE.square(input)
    }

    pub fn cubic(input: Integer) -> Integer {
        BLS12_381_BASE.cubic(input)
    }

    pub fn sqrt(input: Integer) -> Option<Integer> {
        BLS12_381_BASE.sqrt(input)
    }

    pub fn neg(input: Integer) -> Integer {
        BLS12_381_BASE.neg(input)
    }

    pub fn invert(input: Integer) -> Option<Integer> {
        BLS12_381_BASE.invert(input)
    }

    pub fn mul(input: Integer, other: &Integer) -> Integer {
        BLS12_381_BASE.mul(input, other)
    }

    pub fn add(input: Integer, other: &Integer) -> Integer {
        BLS12_381_BASE.add(input, other)
    }

    pub fn sub(input: Integer, other: &Integer) -> Integer {
        BLS12_381_BASE.sub(input, other)
    }
}

impl FieldTrait for Bls12_381BaseField {
    fn random<R: RngCore>(rng: &mut R) -> Integer {
        BLS12_381_BASE.sample_raw(rng)
    }
    fn random_mont<R: RngCore>(rng: &mut R) -> Integer {
        BLS12_381_BASE.sample_mont(rng)
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
    fn mont_mul(a: &Integer, b: &Integer) -> Integer {
        BLS12_381_BASE.mont_mul(a, b)
    }
    fn cubic(input: Integer) -> Integer {
        BLS12_381_BASE.cubic(input)
    }
    fn to_mont(&self) -> Integer {
        BLS12_381_BASE.to_montgomery(&self.0)
    }
    fn from_mont(input: &Integer) -> Integer {
        BLS12_381_BASE.from_montgomery(input)
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

        let out =
            Bls12_381BaseField::mont_mul(&d0, r_ref) + Bls12_381BaseField::mont_mul(&d1, r2_ref);

        (out % modulus).into()
    }
}

#[cfg(test)]
mod tests {

    use rug::Integer;

    use crate::{
        curves::bls12_381::{Bls12_381BaseField, BLS12_381_BASE},
        traits::field::FieldTrait,
    };

    #[test]
    fn test_sqrt_constant() {
        let d = vec![
            "ee7fbfffffffeaab",
            "07aaffffac54ffff",
            "d9cc34a83dac3d89",
            "d91dd2e13ce144af",
            "92c6e9ed90d2eb35",
            "0680447a8e5ff9a6",
        ];
        let d = d
            .iter()
            .map(|x| u64::from_str_radix(x, 16).unwrap())
            .collect::<Vec<_>>();

        let d = Integer::from_digits::<u64>(&d, rug::integer::Order::Lsf);
        println!("d: {}", d.to_string_radix(16));

        let modulus = BLS12_381_BASE.modulus();
        let t: Integer = modulus + 1;
        let t: Integer = t / 4;

        println!("t: {}", t.to_string_radix(16));
    }

    #[test]
    fn test_sqrt() {
        let a = Integer::from(4);
        let a_mont = BLS12_381_BASE.to_montgomery(&a);

        let a_sqrt = BLS12_381_BASE.mont_sqrt(&a_mont);

        if let Some(a) = a_sqrt {
            let a = BLS12_381_BASE.from_montgomery(&a);
            println!("a_sqrt_raw: {}", a.to_string_radix(16));

            let a_neg = BLS12_381_BASE.neg(a.clone());
            println!("a_neg: {}", a_neg.to_string_radix(16));

            let a_square = BLS12_381_BASE.square(a.clone());
            println!("a_square: {}", a_square.to_string_radix(16));
        } else {
            println!("No square root found");
        }
    }
    #[test]
    fn test_square() {
        let a_mont: [u64; 6] = [
            0xd215_d276_8e83_191b,
            0x5085_d80f_8fb2_8261,
            0xce9a_032d_df39_3a56,
            0x3e9c_4fff_2ca0_c4bb,
            0x6436_b6f7_f4d9_5dfb,
            0x1060_6628_ad4a_4d90,
        ];
        let b_mont: [u64; 6] = [
            0x33d9_c42a_3cb3_e235,
            0xdad1_1a09_4c4c_d455,
            0xa2f1_44bd_729a_aeba,
            0xd415_0932_be9f_feac,
            0xe27b_c7c4_7d44_ee50,
            0x14b6_a78d_3ec7_a560,
        ];

        let a_raw =
            Bls12_381BaseField::from_mont(&Integer::from_digits(&a_mont, rug::integer::Order::Lsf));
        let b_raw =
            Bls12_381BaseField::from_mont(&Integer::from_digits(&b_mont, rug::integer::Order::Lsf));

        let a_square = Bls12_381BaseField::square(a_raw.clone());

        println!("a_square: {}", a_square.to_string_radix(16));
        println!("b_raw: {}", b_raw.to_string_radix(16));
    }

    #[test]
    fn test_mul() {
        let a: [u64; 6] = [
            0x0397_a383_2017_0cd4,
            0x734c_1b2c_9e76_1d30,
            0x5ed2_55ad_9a48_beb5,
            0x095a_3c6b_22a7_fcfc,
            0x2294_ce75_d4e2_6a27,
            0x1333_8bd8_7001_1ebb,
        ];
        let b: [u64; 6] = [
            0xb9c3_c7c5_b119_6af7,
            0x2580_e208_6ce3_35c1,
            0xf49a_ed3d_8a57_ef42,
            0x41f2_81e4_9846_e878,
            0xe076_2346_c384_52ce,
            0x0652_e893_26e5_7dc0,
        ];
        let c: [u64; 6] = [
            0xf96e_f3d7_11ab_5355,
            0xe8d4_59ea_00f1_48dd,
            0x53f7_354a_5f00_fa78,
            0x9e34_a4f3_125c_5f83,
            0x3fbe_0c47_ca74_c19e,
            0x01b0_6a8b_bd4a_dfe4,
        ];

        let a_mont = Integer::from_digits(&a, rug::integer::Order::Lsf);
        let b_mont = Integer::from_digits(&b, rug::integer::Order::Lsf);
        let c_mont = Integer::from_digits(&c, rug::integer::Order::Lsf);

        let a_raw = Bls12_381BaseField::from_mont(&a_mont);
        let b_raw = Bls12_381BaseField::from_mont(&b_mont);
        let c_raw = Bls12_381BaseField::from_mont(&c_mont);

        let ab = Bls12_381BaseField::mul(a_raw, &b_raw);

        println!("ab: {}", ab.to_string_radix(16));
        println!("c_raw: {}", c_raw.to_string_radix(16));

        let ab_mont = Bls12_381BaseField::mont_mul(&a_mont, &b_mont);
        let ab_mont_raw = Bls12_381BaseField::from_mont(&ab_mont);
        println!("ab_mont_raw: {}", ab_mont_raw.to_string_radix(16));
    }

    #[test]
    fn test_inversion() {
        let a = [
            0x43b4_3a50_78ac_2076u64,
            0x1ce0_7630_46f8_962b,
            0x724a_5276_486d_735c,
            0x6f05_c2a6_282d_48fd,
            0x2095_bd5b_b4ca_9331,
            0x03b3_5b38_94b0_f7da,
        ];
        let b = [
            0x69ec_d704_0952_148fu64,
            0x985c_cc20_2219_0f55,
            0xe19b_ba36_a9ad_2f41,
            0x19bb_16c9_5219_dbd8,
            0x14dc_acfd_fb47_8693,
            0x115f_f58a_fff9_a8e1,
        ];

        let a_mont = Integer::from_digits(&a, rug::integer::Order::Lsf);
        let b_mont = Integer::from_digits(&b, rug::integer::Order::Lsf);

        let a_raw = Bls12_381BaseField::from_mont(&a_mont);
        let b_raw = Bls12_381BaseField::from_mont(&b_mont);

        println!("a: {}", a_mont.to_string_radix(16));
        println!("b: {}", b_mont.to_string_radix(16));

        println!("a_raw: {}", a_raw.to_string_radix(16));
        println!("b_raw: {}", b_raw.to_string_radix(16));

        let a_inv = Bls12_381BaseField::invert(a_raw).unwrap();
        println!("a_inv: {}", a_inv.to_string_radix(16));

        let a_mont_inv = Bls12_381BaseField::invert(a_mont).unwrap();
        let a_mont_inv_raw = Bls12_381BaseField::from_mont(&a_mont_inv);

        println!("a_mont_inv_raw: {}", a_mont_inv_raw.to_string_radix(16));

        let modulus_plus_one_div_four = BLS12_381_BASE.modulus_plus_one_div_four.clone().unwrap();
        println!(
            "modulus_plus_one_div_four: {}",
            modulus_plus_one_div_four.to_string_radix(16)
        );
    }
}
