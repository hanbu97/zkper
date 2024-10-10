use rug::Integer;

use super::{
    curves::{g1_affine::G1Affine, g2::G2Projective, g2_affine::G2Affine},
    fields::{fp12::Fp12, fp2::Fp2, fp6::Fp6, target::TargetField},
    MILLER_LOOP_CONSTANT, MILLER_LOOP_CONSTANT_IS_NEG,
};

pub struct BLS12_381Pairing;

impl BLS12_381Pairing {
    /// Compute the optimal ate pairing e(P, Q) where P ∈ G1 and Q ∈ G2
    ///
    /// Mathematical formula: e(P, Q) = f_{u,Q}(P)^((p^12 - 1)/r)
    /// where u is the BLS parameter, p is the field characteristic, and r is the group order
    pub fn pairing(g1_point: &G1Affine, g2_point: &G2Affine) -> TargetField {
        let (g1_point, g2_point) = if g1_point.is_identity() || g2_point.is_identity() {
            (G1Affine::generator(), G2Affine::generator())
        } else {
            (g1_point.clone(), g2_point.clone())
        };

        let miller_loop_result = Self::miller_loop(&g1_point, &g2_point);

        // println!("miller_loop_result: {:#}", miller_loop_result);

        Self::final_exponentiation(&miller_loop_result)
    }

    /// Performs the final exponentiation to convert the result of a Miller loop
    /// into an element of the target group Gt.
    ///
    /// Mathematical formula: f^((p^12 - 1) / r)
    /// where p is the field characteristic and r is the group order.
    ///
    /// This is split into two parts:
    /// 1. Easy part: f^(p^6 - 1) * (p^2 + 1)
    /// 2. Hard part: f^((p^4 - p^2 + 1) / r)
    pub fn final_exponentiation(miller_loop_result: &Fp12) -> TargetField {
        // Easy part of final exponentiation
        // f^(p^6 - 1) * (p^2 + 1)
        let mut f1 = miller_loop_result
            .clone()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map();

        if let Some(f_inv) = miller_loop_result.invert() {
            f1 = f1.mul(&f_inv); // f^(p^6 - 1)
            let mut f2 = f1.clone();
            f2 = f2.frobenius_map().frobenius_map(); // f^(p^2)
            f2 = f2.mul(&f1); // f^(p^6 - 1) * (p^2 + 1)

            // Hard part of final exponentiation
            // (p^4 - p^2 + 1) / r
            let y0 = f2.clone();
            let y1 = Self::cyclotomic_exp(&y0);
            let y2 = Self::cyclotomic_square(&y1);
            let y3 = y2.mul(&y1);
            let y4 = Self::cyclotomic_exp(&y3);
            let y5 = Self::cyclotomic_exp(&y4);
            let y6 = y5.mul(&y2);
            let y7 = Self::cyclotomic_exp(&y6);
            let y8 = y7.mul(&y3.conjugate());
            let y9 = y8.mul(&y1);
            let y10 = y9.mul(&y6);
            let y11 = y10.frobenius_map();
            let y12 = y11.mul(&y9);

            TargetField(y12)
        } else {
            // This should never happen for valid input
            TargetField(Fp12::one())
        }
    }

    /// Computes the square of an Fp4 element: (a + bi)^2 = (a^2 - b^2) + (2ab)i
    fn fp4_square(a: &Fp2, b: &Fp2) -> (Fp2, Fp2) {
        let a_squared = a.square();
        let b_squared = b.square();
        let ab_mul_2 = a.add(&b).square().sub(&a_squared).sub(&b_squared);

        let c0 = a_squared.add(&b_squared.mul_by_nonresidue());
        let c1 = ab_mul_2;

        (c0, c1)
    }

    /// Performs efficient squaring in the cyclotomic subgroup
    /// Based on "Faster Squaring in the Cyclotomic Subgroup of Sixth Degree Extensions"
    /// by F. Beuchat et al. (https://eprint.iacr.org/2009/565.pdf)
    fn cyclotomic_square(f: &Fp12) -> Fp12 {
        let mut z0 = f.c0.c0.clone();
        let mut z1 = f.c1.c1.clone();
        let mut z2 = f.c1.c0.clone();
        let mut z3 = f.c0.c2.clone();
        let mut z4 = f.c0.c1.clone();
        let mut z5 = f.c1.c2.clone();

        let (t0, t1) = Self::fp4_square(&z0, &z1);

        // For A
        z0 = t0.sub(&z0);
        z0 = z0.add(&z0).add(&t0);

        z1 = t1.add(&z1);
        z1 = z1.add(&z1).add(&t1);

        let (mut t0, t1) = Self::fp4_square(&z2, &z3);
        let (t2, t3) = Self::fp4_square(&z4, &z5);

        // For C
        z4 = t0.sub(&z4);
        z4 = z4.add(&z4).add(&t0);

        z5 = t1.add(&z5);
        z5 = z5.add(&z5).add(&t1);

        // For B
        t0 = t3.mul_by_nonresidue();
        z2 = t0.add(&z2);
        z2 = z2.add(&z2).add(&t0);

        z3 = t2.sub(&z3);
        z3 = z3.add(&z3).add(&t2);

        Fp12 {
            c0: Fp6 {
                c0: z0,
                c1: z4,
                c2: z3,
            },
            c1: Fp6 {
                c0: z2,
                c1: z1,
                c2: z5,
            },
        }
    }

    /// Performs exponentiation by x in the cyclotomic subgroup
    /// where x is the BLS parameter
    fn cyclotomic_exp(base: &Fp12) -> Fp12 {
        let mut result = Fp12::one();
        let mut found_one = false;

        for i in (0..64).rev() {
            if found_one {
                result = Self::cyclotomic_square(&result);
            }

            if ((MILLER_LOOP_CONSTANT >> i) & 1) == 1 {
                found_one = true;
                result = result.mul(base);
            }
        }

        result.conjugate()
    }

    /// Perform Miller loop to compute f_{u,Q}(P)
    ///
    /// Algorithm: Optimized Miller Loop for BLS12-381
    fn miller_loop(g1_point: &G1Affine, g2_point: &G2Affine) -> Fp12 {
        let mut current_point = G2Projective::from(g2_point.clone());
        let mut accumulator = Fp12::one();

        for bit in (1..64).rev() {
            accumulator = accumulator.square();
            accumulator = accumulator.mul(&Self::line_evaluation(&mut current_point, g1_point));

            if ((MILLER_LOOP_CONSTANT >> bit) & 1) == 1 {
                current_point = current_point.add_affine(g2_point);
                accumulator = accumulator.mul(&Self::line_evaluation(&mut current_point, g1_point));
            }
        }

        if MILLER_LOOP_CONSTANT_IS_NEG {
            accumulator = accumulator.conjugate();
        }

        accumulator
    }

    /// Compute the line function evaluation l_{T,Q}(P)
    ///
    /// Formula: l_{T,Q}(P) = (y_P - y_T) - λ(x_P - x_T)
    /// where λ is the slope of the line through T and Q
    fn line_evaluation(current: &mut G2Projective, g1_point: &G1Affine) -> Fp12 {
        let (x, y, z) = (&mut current.x, &mut current.y, &mut current.z);

        // Compute λ (slope)
        let slope = y.square().mul(&&x.mul_base(&Integer::from(3))).normalize();
        let vertical = y.mul(z).mul_base(&Integer::from(2)).normalize();
        let chord = z
            .square()
            .mul(&slope)
            .mul_base(&Integer::from(3))
            .normalize();

        // Update current point (doubling step)
        *x = slope
            .square()
            .sub(&vertical.mul_base(&Integer::from(2)))
            .normalize();
        *y = slope.mul(&vertical.sub(x)).sub(&chord).normalize();
        *z = vertical.mul(z).normalize();

        // Compute line evaluation
        let t1 = Fp2::from_base(&g1_point.y)
            .mul(z)
            .sub(&y.mul(&Fp2::from_base(&g1_point.x)))
            .normalize();
        let t2 = x
            .mul(&Fp2::from_base(&g1_point.x))
            .sub(&slope.mul(z))
            .normalize();

        Fp12::new(
            Fp6::new(t1, t2, Fp2::zero()),
            Fp6::new(vertical.clone(), Fp2::zero(), Fp2::zero()),
        )
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::curves::bls12_381::{
        curves::{g1::G1Projective, g1_affine::G1Affine, g2::G2Projective},
        paring::BLS12_381Pairing,
        Bls12_381ScalarField,
    };

    #[test]
    fn test_bilinearity() {
        // let a = Integer::from_digits(&[1u64, 2, 3, 4], rug::integer::Order::Lsf);

        let a = Bls12_381ScalarField::from_raw([1, 2, 3, 4])
            .invert()
            .unwrap()
            .square();
        let b = Bls12_381ScalarField::from_raw([5, 6, 7, 8])
            .invert()
            .unwrap()
            .square();

        let c = a.mul(&b);

        let g = G1Projective::generator().mul_scalar(&a.0).to_affine();
        let h = G2Projective::generator().mul_scalar(&b.0).to_affine();

        let pairing = BLS12_381Pairing::pairing(&g, &h);

        // println!("pairing: {:#}", pairing);

        // let g = G1Projective::generator().mul_scalar(&a.0);
        // println!("g: {:#}", g);
        // let g =

        // println!("c: {}", c.0.to_string_radix(16));

        // let a_g = Bls12_381ScalarField::MULTIPLICATIVE_GENERATOR.mul(&a);
        // let b_g = Bls12_381ScalarField::MULTIPLICATIVE_GENERATOR.mul(&b);

        // let ab_g = Bls12_381ScalarField::MULTIPLICATIVE_GENERATOR.mul(&(a * b));

        // assert_eq!(ab_g, a_g.mul(&b));
        // assert_eq!(ab_g, b_g.mul(&a));
    }
}
