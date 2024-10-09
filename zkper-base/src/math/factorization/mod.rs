pub mod ecm;
pub mod pollards_rho;
pub mod traits;
use num_traits::One;
use rug::{Complete, Integer};
use std::ops::Neg;

use crate::utils::prime::PrimeChecking;

use self::{ecm::get_factor_ecm, pollards_rho::get_factor_pollard_rho};

/// Checks that the given list of factors contains all the unique primes of m.
pub fn check_factors(m: &Integer, factors: &[Integer]) -> anyhow::Result<()> {
    let mut remaining = m.clone();
    for factor in factors {
        if !factor.is_prime() {
            return Err(anyhow::anyhow!("Composite factor found"));
        }
        while (&remaining % factor).complete().is_zero() {
            remaining /= factor;
        }
    }
    if remaining.is_one() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Incomplete factor list"))
    }
}

pub fn find_generator(modulus: &Integer) -> Integer {
    let phi = modulus.clone() - 1;
    let factors = get_factors(&phi).unwrap();

    for a in 2..=20 {
        let candidate = Integer::from(a);
        if is_primitive_root(&candidate, modulus, &phi, &factors) {
            return candidate;
        }
        let neg_candidate = candidate.neg();
        if is_primitive_root(&neg_candidate, modulus, &phi, &factors) {
            return neg_candidate;
        }
    }
    panic!("Generator not found");
}

pub fn is_primitive_root(
    a: &Integer,
    modulus: &Integer,
    phi: &Integer,
    factors: &[Integer],
) -> bool {
    for p in factors {
        let exp = (phi / p).complete();
        if a.clone().pow_mod(&exp, modulus).unwrap().is_one() {
            return false;
        }
    }
    true
}

/// Computes the smallest primitive root of the given prime q.
/// The unique factors of q-1 can be given to speed up the search for the root.
pub fn primitive_root(
    q: &Integer,
    factors: Option<Vec<Integer>>,
) -> anyhow::Result<(Integer, Vec<Integer>)> {
    let factors = match factors {
        Some(f) => {
            check_factors(&(q.clone() - 1), &f)?;
            f
        }
        None => get_factors(&(q.clone() - 1))?,
    };

    let mut g = Integer::from(2);
    let q_minus_one: Integer = q.clone() - 1;
    loop {
        let mut is_primitive_root = true;
        for factor in &factors {
            let exp = q_minus_one.clone() / factor;
            if g.clone().pow_mod(&exp, q).unwrap() == 1 {
                is_primitive_root = false;
                break;
            }
        }
        if is_primitive_root {
            return Ok((g, factors));
        }
        g += 1;
    }
}

/// Finds all prime factors of a given BigUint.
/// Returns a sorted vector of prime factors.
pub fn get_factors(m: &Integer) -> anyhow::Result<Vec<Integer>> {
    let mut m_cpy = m.clone();
    if m_cpy.is_prime() {
        return Ok(vec![m_cpy]);
    }

    let mut f = std::collections::HashSet::new();

    for prime in primal::Primes::all().take(10000) {
        let small_prime = Integer::from(prime);
        let mut add_factor = false;

        while m_cpy.is_divisible_u(prime as u32) {
            m_cpy /= &small_prime;
            add_factor = true;
        }
        if add_factor {
            f.insert(small_prime);
        }
    }

    // Second, find the remaining large prime factors
    while !m_cpy.is_one() {
        if m_cpy.is_prime() {
            f.insert(m_cpy.clone());
            break;
        }

        // Try Pollard's Rho algorithm first
        let mut factor: Integer = get_factor_pollard_rho(&m_cpy);
        if factor.is_one() || factor == m_cpy {
            // If Pollard's Rho fails, try ECM factorization
            factor = get_factor_ecm(&m_cpy)?;
        }

        // Remove all instances of this factor from m_cpy
        let temp: Integer = (&m_cpy % &factor).into();
        while temp.is_zero() {
            m_cpy /= &factor;
        }

        f.insert(factor);
    }

    // Convert the set of factors to a sorted vector
    let mut factors: Vec<Integer> = f.into_iter().collect();
    factors.sort();
    Ok(factors)
}

/// Checks if the given factors completely factorize the input number.
///
/// # Arguments
///
/// * `p` - The number to be factorized.
/// * `factors` - A slice of factors to check against.
///
/// # Returns
///
/// `true` if the factors completely factorize `p`, `false` otherwise.
pub fn check_factorization(p: &Integer, factors: &[Integer]) -> bool {
    let mut remaining = p.clone();

    for factor in factors {
        while (&remaining % factor).complete().is_zero() {
            remaining /= factor;
        }
    }

    remaining.is_one()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_primes() {
        let max = primal::Primes::all().take(10000).last().unwrap();
        dbg!(max);
    }

    #[test]
    fn test_get_factors() {
        let m = Integer::from(0x1fffffffffe00001u64 - 1);
        let factors = get_factors(&m).unwrap();

        println!("m: {} {:?}", m, factors);
        assert!(check_factorization(&m, &factors));
    }
}
