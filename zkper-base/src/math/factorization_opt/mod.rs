use primal::Sieve;
use rug::Integer;
use std::ops::Neg;

use num_traits::identities::One;

use crate::utils::prime::PrimeChecking;

const SIEVE_LIMIT: usize = 1000000;

pub fn find_generator(modulus: &Integer) -> Integer {
    let phi = modulus.clone() - 1;
    let factors = factor(&phi);

    let sieve = Sieve::new(SIEVE_LIMIT);
    for prime in sieve.primes_from(2) {
        let candidate = Integer::from(prime);
        if is_primitive_root(&candidate, modulus, &factors) {
            return candidate;
        }
        let neg_candidate = candidate.neg();
        if is_primitive_root(&neg_candidate, modulus, &factors) {
            return neg_candidate;
        }
        if prime > 1000 {
            break;
        }
    }

    panic!("Generator not found");
}

pub fn is_primitive_root(a: &Integer, modulus: &Integer, factors: &[(Integer, u32)]) -> bool {
    let phi: Integer = modulus.clone() - 1;
    for (p, _) in factors {
        let exp = phi.clone() / p;
        if a.clone().pow_mod(&exp, modulus).unwrap() == 1 {
            return false;
        }
    }
    true
}

pub fn factor(n: &Integer) -> Vec<(Integer, u32)> {
    let mut factors = Vec::new();
    let mut m = n.clone();

    let sieve = Sieve::new(SIEVE_LIMIT);
    for prime in sieve.primes_from(2) {
        let p = Integer::from(prime);
        let mut exp = 0;
        while m.is_divisible(&p) {
            m /= &p;
            exp += 1;
        }
        if exp > 0 {
            factors.push((p, exp));
        }
        if m.is_one() {
            return factors;
        }
        if m < Integer::from(SIEVE_LIMIT * SIEVE_LIMIT) {
            break;
        }
    }

    if !m.is_one() {
        if m.is_prime() {
            factors.push((m, 1));
        } else {
            pollard_rho_factor(&mut m, &mut factors);
        }
    }

    factors.sort_by(|a, b| a.0.cmp(&b.0));
    factors
}

pub fn pollard_rho_factor(n: &mut Integer, factors: &mut Vec<(Integer, u32)>) {
    while n > &mut Integer::from(1) {
        if n.is_prime() {
            factors.push((n.clone(), 1));
            break;
        }
        let d = pollard_rho(n);
        let mut exp = 1;
        *n /= &d;
        while n.is_divisible(&d) {
            *n /= &d;
            exp += 1;
        }
        factors.push((d, exp));
    }
}

pub fn pollard_rho(n: &Integer) -> Integer {
    let mut x = Integer::from(2);
    let mut y = Integer::from(2);
    let mut d = Integer::from(1);

    while d.is_one() {
        x = (x.clone() * &x + 1) % n;
        y = (y.clone() * &y + 1) % n;
        y = (y.clone() * &y + 1) % n;
        d = Integer::from((x.clone() - &y).abs()).gcd(n);
    }

    d
}

#[test]
pub fn test_gen_two_adic_primitive_root_of_unity() {
    let modulus = Integer::from_str_radix(
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
