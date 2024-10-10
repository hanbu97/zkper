use zkper_curves::curves::bls12_381::Bls12_381ScalarField;

use crate::MIMC_ROUNDS;

/// Implementation of LongsightF322p3 MiMC for BLS12-381
/// ref: http://eprint.iacr.org/2016/492
///
/// ```
/// function LongsightF322p3(xL ⦂ Fp, xR ⦂ Fp) {
///     for i from 0 up to 321 {
///         xL, xR := xR + (xL + Ci)^3, xL
///     }
///     return xL
/// }
/// ```
pub fn mimc_implemention(
    mut xl: Bls12_381ScalarField,
    mut xr: Bls12_381ScalarField,
    constants: &[Bls12_381ScalarField],
) -> Bls12_381ScalarField {
    assert_eq!(constants.len(), MIMC_ROUNDS);

    for c in constants {
        let mut tmp1 = xl.clone();
        tmp1.add_assign(c);
        let mut tmp2 = tmp1.square();
        tmp2.mul_assign(&tmp1);
        tmp2.add_assign(&xr);
        xr = xl.clone();
        xl = tmp2;
    }

    xl
}
