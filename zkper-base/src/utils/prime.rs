/// A trait for checking primality of integers
pub trait PrimeChecking {
    /// Returns whether the number is prime
    fn is_prime(&self) -> bool;
}

impl PrimeChecking for rug::Integer {
    fn is_prime(&self) -> bool {
        self.is_probably_prime(25) != rug::integer::IsPrime::No
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rug::Integer;

    #[test]
    fn test_is_prime() {
        assert!(Integer::from(0xffffffffffffffc5u64).is_prime());
        assert!(Integer::from(18446744073709551629u128).is_prime());
        assert!(!Integer::from(0xffffffffffffffffu64).is_prime());
    }
}
