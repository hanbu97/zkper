use std::ops::{Add, Sub};
use zkper_integer::{traits::ZkperIntegerTrait, ZkperInteger};

// Define a trait for ZkperPrime
pub trait ZkperPrimeTrait<T: ZkperIntegerTrait>: Sized + Add + Sub {
    fn value() -> ZkperInteger<T>;

    fn reduce(a: &ZkperInteger<T>) -> ZkperInteger<T> {
        a % &Self::value()
    }

    // basic arithmetic operations
    fn additive(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T> {
        // to optimize clone
        Self::reduce(&(a + b))
    }
    // fn sub(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T>;
    // fn mul(a: &ZkperInteger<T>, b: &ZkperInteger<T>) -> ZkperInteger<T>;
    // fn neg(a: &ZkperInteger<T>) -> ZkperInteger<T>;
}
