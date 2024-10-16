// ref: https://github.com/skyf0l/ecm-rs/blob/main/src/ecm.rs

use rand::{rngs::OsRng, Rng};
use zkper_integer::{traits::ZkperIntegerTrait, ZkperInteger};
use zkper_rand::ZkperRng;

use self::ecm::ecm_one_factor;

pub mod ecm;
pub mod errors;
pub mod point;

/// Elliptic Curve Method (ECM) for factorization.
/// This function attempts to find a single factor of the input number.
pub fn get_factor_ecm<T: ZkperIntegerTrait>(
    n: &ZkperInteger<T>,
) -> anyhow::Result<ZkperInteger<T>> {
    let digits = n.to_string().len();
    let (b1, b2, max_curve) = ecm::optimal_params(digits);

    // rgen.seed(&Integer::from(1234)); // You can change this seed if needed
    let mut csprng = OsRng;
    let mut rgen = ZkperRng::from_seed(csprng.gen::<u64>());

    match ecm_one_factor(n, b1, b2, max_curve, &mut rgen) {
        Ok(factor) => Ok(factor),
        Err(errors::ECMErrors::NumberIsPrime) => Ok(n.clone()),
        Err(e) => Err(e.into()),
    }
}
