use crate::backends::montgomery::MontgomeryBackend;

lazy_static::lazy_static! {
    pub static ref BLS12_381_SCALAR: MontgomeryBackend = MontgomeryBackend::from_str_radix(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513", 10,4
    );
    pub static ref BLS12_381_BASE: MontgomeryBackend = MontgomeryBackend::from_str_radix(
        "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787", 10,6
    );
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
}
