use super::*;

// Implements Pollard's Rho algorithm for factorization using rug::Integer.
/// This function attempts to find a single factor of the input number.
pub fn get_factor_pollard_rho<T: ZkperIntegerTrait>(m: &ZkperInteger<T>) -> ZkperInteger<T> {
    if m.is_prime() {
        return m.clone();
    }

    // Try different values of c in the polynomial x^2 + c
    for i in 1..10usize {
        let mut x = ZkperInteger::two();
        let mut y = ZkperInteger::two();
        let mut d = ZkperInteger::one();

        let c = ZkperInteger::from(i);

        while d.is_zero() || &d == m {
            // "Tortoise and hare" step
            x = polynomial_pollards_rho(&x, &c, m);
            y = polynomial_pollards_rho(&polynomial_pollards_rho(&y, &c, m), &c, m);

            d = &x - &y;
            d = d.abs().gcd(m);

            if d.is_not_one() {
                return d;
            }
        }
    }

    ZkperInteger::one() // Return 1 if no factor is found
}

/// Polynomial function used in Pollard's Rho algorithm: f(x) = x^2 + c mod n
fn polynomial_pollards_rho<T: ZkperIntegerTrait>(
    x: &ZkperInteger<T>,
    c: &ZkperInteger<T>,
    n: &ZkperInteger<T>,
) -> ZkperInteger<T> {
    (x.square() + c) % n
}

#[cfg(test)]
mod test {
    use crate::integer::backends::rug_backend::RugBackend;

    use super::*;

    #[test]
    fn test_get_factor_pollard_rho() {
        let prime = 0x1fffffffffe00001u64;
        let m = ZkperInteger::<RugBackend>::from(prime - 1);
        assert_eq!(m.clone() % get_factor_pollard_rho(&m), ZkperInteger::zero());
    }
}
