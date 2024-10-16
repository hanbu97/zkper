use primal::Sieve;

use crate::integer::{traits::ZkperIntegerTrait, ZkperInteger};

const SIEVE_LIMIT: usize = 1000000;

pub fn find_generator<T: ZkperIntegerTrait>(modulus: &ZkperInteger<T>) -> ZkperInteger<T> {
    let phi = modulus - 1u64;
    let factors = factor(&phi);

    let sieve = Sieve::new(SIEVE_LIMIT);
    for prime in sieve.primes_from(2) {
        let candidate = ZkperInteger::from(prime);
        if is_primitive_root(&candidate, modulus, &factors) {
            return candidate;
        }
        let neg_candidate = -candidate;
        if is_primitive_root(&neg_candidate, modulus, &factors) {
            return neg_candidate;
        }
        if prime > 1000 {
            break;
        }
    }

    panic!("Generator not found");
}

pub fn is_primitive_root<T: ZkperIntegerTrait>(
    a: &ZkperInteger<T>,
    modulus: &ZkperInteger<T>,
    factors: &[(ZkperInteger<T>, u32)],
) -> bool {
    let phi: ZkperInteger<T> = modulus.clone() - 1;
    for (p, _) in factors {
        let exp = phi.clone() / p;
        if a.clone().pow_mod(&exp, modulus).is_one() {
            return false;
        }
    }
    true
}

pub fn factor<T: ZkperIntegerTrait>(n: &ZkperInteger<T>) -> Vec<(ZkperInteger<T>, u32)> {
    let mut factors = Vec::new();
    let mut m = n.clone();

    let sieve = Sieve::new(SIEVE_LIMIT);
    for prime in sieve.primes_from(2) {
        let p = ZkperInteger::from(prime);
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
        if m < ZkperInteger::from(SIEVE_LIMIT * SIEVE_LIMIT) {
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

pub fn pollard_rho_factor<T: ZkperIntegerTrait>(
    n: &mut ZkperInteger<T>,
    factors: &mut Vec<(ZkperInteger<T>, u32)>,
) {
    while n > &mut ZkperInteger::one() {
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

pub fn pollard_rho<T: ZkperIntegerTrait>(n: &ZkperInteger<T>) -> ZkperInteger<T> {
    let mut x = ZkperInteger::two();
    let mut y = ZkperInteger::two();
    let mut d = ZkperInteger::one();

    while d.is_one() {
        x = (x.clone() * &x + 1) % n;
        y = (y.clone() * &y + 1) % n;
        y = (y.clone() * &y + 1) % n;
        d = (x.clone() - &y).abs().gcd(n);
    }

    d
}

#[cfg(test)]
mod tests {
    use crate::{
        integer::{backends::rug_backend::RugBackend, ZkperInteger},
        math::factorization::find_generator,
    };

    #[test]
    pub fn test_gen_two_adic_primitive_root_of_unity() {
        let modulus = ZkperInteger::<RugBackend>::from_hex_str(
            "52435875175126190479447740508185965837690552500527637822603658699938581184513",
        );

        let generator = find_generator(&modulus);
        println!("Generator: {}", generator);

        let two_adicity = (&modulus - ZkperInteger::one()).find_first_one(0).unwrap();
        let trace = (modulus.clone() - 1) >> two_adicity;
        let two_adic_root_of_unity = generator.pow_mod(&trace, &modulus);
        println!("2-adic Root of Unity: {}", two_adic_root_of_unity);
    }
}
