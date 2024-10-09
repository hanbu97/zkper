use rug::Integer;
use zkper_base::math::factorization::find_generator;

fn main() {
    let modulus: Integer = Integer::from_str_radix(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
        10,
    )
    .unwrap();

    let generator = find_generator(&modulus);
    println!("Generator: {}", generator);

    let two_adicity = (modulus.clone() - Integer::ONE).find_one(0).unwrap();
    let trace = (modulus.clone() - 1) >> two_adicity;
    let two_adic_root_of_unity = generator.pow_mod(&trace, &modulus).unwrap();
    println!("2-adic Root of Unity: {}", two_adic_root_of_unity);
}
