use std::{
    fmt::Debug,
    ops::{Add, Sub},
};
use zkper_integer::{traits::ZkperIntegerTrait, ZkperInteger};

// Define a trait for ZkperPrime
pub trait ZkperPrimeTrait<T: ZkperIntegerTrait>: Sized + Add + Sub + Clone + Debug {
    fn value() -> ZkperInteger<T>;

    fn reduce(a: &ZkperInteger<T>) -> ZkperInteger<T> {
        a % &Self::value()
    }

    // basic arithmetic operations  // to optimize clone
    fn additive(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T> {
        Self::reduce(&(a + b))
    }
    fn subtract(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T> {
        Self::reduce(&(a.clone() - b))
    }
    fn multiply(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T> {
        Self::reduce(&(a.clone() * b))
    }
    fn negative(a: &ZkperInteger<T>) -> ZkperInteger<T> {
        Self::reduce(&-a.clone())
    }

    // advanced arithmetic operations
    fn square(a: &ZkperInteger<T>) -> ZkperInteger<T> {
        Self::multiply(a, a)
    }
    fn power(base: &ZkperInteger<T>, exp: &ZkperInteger<T>) -> ZkperInteger<T> {
        let mut result = ZkperInteger::one();
        let mut base = base.clone();
        let mut exp = exp.clone();
        while !exp.is_zero() {
            if exp.is_odd() {
                result = Self::multiply(&result, &base);
            }
            base = Self::square(&base);
            exp >>= 1;
        }
        result
    }
}
