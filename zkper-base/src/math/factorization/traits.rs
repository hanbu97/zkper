use super::*;

pub trait CompositeSplitter<T: ZkperIntegerTrait> {
    /// Undefined behavior if `n` is prime.
    fn divisor(&self, n: &ZkperInteger<T>) -> ZkperInteger<T>;

    fn split(&self, n: &ZkperInteger<T>) -> (ZkperInteger<T>, ZkperInteger<T>) {
        let d1 = self.divisor(n);
        let d2 = (n / &d1).into();
        if d1 < d2 {
            (d1, d2)
        } else {
            (d2, d1)
        }
    }
}
