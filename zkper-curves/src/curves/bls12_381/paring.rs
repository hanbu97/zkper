use super::{
    curves::{g1_affine::G1Affine, g2::G2Projective, g2_affine::G2Affine},
    fields::{fp12::Fp12, fp2::Fp2, fp6::Fp6, target::TargetField},
    BLS12_381_BASE, MILLER_LOOP_CONSTANT, MILLER_LOOP_CONSTANT_IS_NEG,
};

pub struct BLS12_381Pairing;

impl BLS12_381Pairing {
    /// Compute the optimal ate pairing e(P, Q) where P ∈ G1 and Q ∈ G2
    ///
    /// Mathematical formula: e(P, Q) = f_{u,Q}(P)^((p^12 - 1)/r)
    /// where u is the BLS parameter, p is the field characteristic, and r is the group order
    pub fn pairing(g1_point: &G1Affine, g2_point: &G2Affine) -> TargetField {
        if g1_point.is_identity() || g2_point.is_identity() {
            return TargetField::one();
        }

        let miller_loop_result = Self::miller_loop(&g1_point, &g2_point);
        // println!("miller_loop_result: {:#}", miller_loop_result);

        Self::final_exponentiation(&miller_loop_result)
    }

    /// Performs the multi-Miller loop for the optimal ate pairing on BLS12-381.
    ///
    /// This function computes the product of multiple pairings simultaneously,
    /// which is more efficient than computing each pairing separately and then
    /// multiplying the results.
    ///
    /// Mathematical background:
    /// For (P_i, Q_i) pairs where P_i ∈ G1, Q_i ∈ G2,
    /// the multi-Miller loop computes ∏_i f_{u,Q_i}(P_i) where:
    /// - u is the BLS parameter (x in this case)
    /// - f_{u,Q_i} is the function arising from Miller's algorithm
    ///
    /// The loop is optimized for the BLS12-381 curve parameters.
    pub fn multi_miller_loop(pairs: &[(&G1Affine, &G2Affine)]) -> Fp12 {
        let mut f = Fp12::one();
        let mut found_one = false;
        let mut current_points: Vec<G2Projective> =
            pairs.iter().map(|(_, q)| G2Projective::from(*q)).collect();

        for i in (0..64).rev() {
            let bit = ((MILLER_LOOP_CONSTANT >> 1) >> i) & 1 == 1;

            if !found_one {
                if bit {
                    found_one = true;
                }
                continue;
            }

            f = Self::multi_doubling_step(&mut current_points, &f, pairs);

            if bit {
                f = Self::multi_addition_step(&mut current_points, pairs, &f);
            }

            f = f.square();
        }

        f = Self::multi_doubling_step(&mut current_points, &f, pairs);

        if MILLER_LOOP_CONSTANT_IS_NEG {
            f = f.conjugate();
        }

        f
    }

    fn multi_doubling_step(
        current_points: &mut [G2Projective],
        f: &Fp12,
        pairs: &[(&G1Affine, &G2Affine)],
    ) -> Fp12 {
        let mut result = f.clone();
        for (current, (p, _)) in current_points.iter_mut().zip(pairs.iter()) {
            result = Self::doubling_step(current, &result, p);
        }
        result
    }

    fn multi_addition_step(
        current_points: &mut [G2Projective],
        pairs: &[(&G1Affine, &G2Affine)],
        f: &Fp12,
    ) -> Fp12 {
        let mut result = f.clone();
        for (current, (p, q)) in current_points.iter_mut().zip(pairs.iter()) {
            result = Self::addition_step(current, q, &result, p);
        }
        result
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
        let mut t0 = miller_loop_result
            .clone()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map()
            .frobenius_map();

        if let Some(mut t1) = miller_loop_result.invert() {
            let mut t2 = t0.mul(&t1);
            t1 = t2.clone();
            t2 = t2.frobenius_map().frobenius_map();
            t2 = t2.mul(&t1);
            t1 = Self::cyclotomic_square(&t2).conjugate();
            let mut t3 = Self::cyclotomic_exp(&t2);
            let mut t4 = Self::cyclotomic_square(&t3);
            let mut t5 = t1.mul(&t3);
            t1 = Self::cyclotomic_exp(&t5);
            t0 = Self::cyclotomic_exp(&t1);
            let mut t6 = Self::cyclotomic_exp(&t0);
            t6 = t6.mul(&t4);
            t4 = Self::cyclotomic_exp(&t6);
            t5 = t5.conjugate();
            t4 = t4.mul(&t5.mul(&t2));
            t5 = t2.conjugate();
            t1 = t1.mul(&t2).frobenius_map().frobenius_map().frobenius_map();
            t6 = t6.mul(&t5).frobenius_map();
            t3 = t3
                .mul(&t0)
                .frobenius_map()
                .frobenius_map()
                .mul(&t1)
                .mul(&t6);

            TargetField(t3.mul(&t4))
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

    /// Performs the Miller loop for the optimal ate pairing on BLS12-381.
    ///
    /// Mathematical background:
    /// For P ∈ G1, Q ∈ G2, the Miller loop computes f_{u,Q}(P) where:
    /// - u is the BLS parameter (x in this case)
    /// - f_{u,Q} is the function arising from Miller's algorithm
    ///
    /// The loop is optimized for the BLS12-381 curve parameters.
    pub fn miller_loop(p: &G1Affine, q: &G2Affine) -> Fp12 {
        let mut f = Fp12::one();
        let mut found_one = false;
        let mut current = G2Projective::from(q.clone());

        for i in (0..64).rev() {
            let bit = ((MILLER_LOOP_CONSTANT >> 1) >> i) & 1 == 1;

            if !found_one {
                if bit {
                    found_one = true;
                }
                continue;
            }

            f = Self::doubling_step(&mut current, &f, p);

            if bit {
                f = Self::addition_step(&mut current, q, &f, p);
            }

            f = f.square();
        }

        f = Self::doubling_step(&mut current, &f, p);

        if MILLER_LOOP_CONSTANT_IS_NEG {
            f = f.conjugate();
        }

        f
    }

    /// Performs the addition step in Miller's algorithm.
    ///
    /// This step computes the line function arising from adding two points in G2
    /// and evaluates it at the G1 point.
    ///
    /// Formula: l_{T,Q}(P) where T is the current G2 point, Q is the fixed G2 point,
    /// and P is the G1 point.
    fn addition_step(current: &mut G2Projective, q: &G2Affine, f: &Fp12, p: &G1Affine) -> Fp12 {
        let line_coeffs = Self::compute_addition_coefficients(current, q);

        // println!(
        //     "line_coeffs: {}\n {}\n {}\n",
        //     line_coeffs.0, line_coeffs.1, line_coeffs.2
        // );

        Self::evaluate_line(f.clone(), &line_coeffs, p)
    }

    /// Computes the coefficients of the line function for point addition in G2.
    ///
    /// Returns (a, b, c) where the line function is ax + by + c = 0.
    fn compute_addition_coefficients(r: &mut G2Projective, q: &G2Affine) -> (Fp2, Fp2, Fp2) {
        let zsquared = r.z.square();
        let ysquared = q.y.square();
        let t0 = zsquared.mul(&q.x);
        let t1 =
            q.y.add(&r.z)
                .square()
                .sub(&ysquared)
                .sub(&zsquared)
                .mul(&zsquared);
        let t2 = t0.sub(&r.x);
        let t3 = t2.square();

        let t4 = t3.add(&t3);
        let t4 = t4.add(&t4);
        let t5 = t4.mul(&t2);
        let t6 = t1.sub(&r.y).sub(&r.y);
        let t9 = t6.mul(&q.x);
        let t7 = t4.mul(&r.x);
        r.x = t6.square().sub(&t5).sub(&t7).sub(&t7);
        r.z = r.z.add(&t2).square().sub(&zsquared).sub(&t3);
        let t10 = q.y.add(&r.z);
        let t8 = t7.sub(&r.x).mul(&t6);
        let t0 = r.y.mul(&t5);
        let t0 = t0.add(&t0);
        r.y = t8.sub(&t0);
        let t10 = t10.square().sub(&ysquared);
        let ztsquared = r.z.square();
        let t10 = t10.sub(&ztsquared);
        let t9 = t9.add(&t9).sub(&t10);
        let t10 = r.z.add(&r.z);
        let t6 = t6.neg();
        let t1 = t6.add(&t6);

        (t10, t1, t9)
    }

    /// Performs the doubling step in Miller's algorithm.
    ///
    /// This step computes the line function arising from doubling a point in G2
    /// and evaluates it at the G1 point.
    ///
    /// Formula: l_{T,T}(P) where T is the current G2 point and P is the G1 point.
    fn doubling_step(current: &mut G2Projective, f: &Fp12, p: &G1Affine) -> Fp12 {
        let line_coeffs = Self::compute_doubling_coefficients(current);
        Self::evaluate_line(f.clone(), &line_coeffs, p)
    }

    /// Evaluates the line function l(P) = a * y_p + b * x_p + c at the point P.
    ///
    /// Parameters:
    /// - line_coeffs: (a, b, c) coefficients of the line function
    /// - p: Point in G1 to evaluate the line function at
    /// - f: Current value of the Miller loop accumulator
    fn evaluate_line(f: Fp12, line_coeffs: &(Fp2, Fp2, Fp2), p: &G1Affine) -> Fp12 {
        let mut c0 = line_coeffs.0.clone();
        let mut c1 = line_coeffs.1.clone();

        c0.c0 = BLS12_381_BASE.mul(c0.c0, &p.y);
        c0.c1 = BLS12_381_BASE.mul(c0.c1, &p.y);

        c1.c0 = BLS12_381_BASE.mul(c1.c0, &p.x);
        c1.c1 = BLS12_381_BASE.mul(c1.c1, &p.x);

        // Use the optimized multiplication function
        f.mul_by_c0_c1_c4(&line_coeffs.2, &c1, &c0)
    }

    /// Computes the coefficients of the line function for point doubling in G2.
    ///
    /// Returns (a, b, c) where the line function is ax + by + c = 0.
    fn compute_doubling_coefficients(r: &mut G2Projective) -> (Fp2, Fp2, Fp2) {
        let tmp0 = r.x.square();
        let tmp1 = r.y.square();
        let tmp2 = tmp1.square();
        let tmp3 = tmp1.add(&r.x).square().sub(&tmp0).sub(&tmp2);
        let tmp3 = tmp3.add(&tmp3);
        let tmp4 = tmp0.add(&tmp0).add(&tmp0);
        let tmp6 = r.x.add(&tmp4);
        let tmp5 = tmp4.square();
        let zsquared = r.z.square();
        r.x = tmp5.sub(&tmp3).sub(&tmp3);
        r.z = r.z.add(&r.y).square().sub(&tmp1).sub(&zsquared);

        r.y = tmp3.sub(&r.x).mul(&tmp4);
        let tmp2 = tmp2.add(&tmp2);
        let tmp2 = tmp2.add(&tmp2);
        let tmp2 = tmp2.add(&tmp2);
        r.y = r.y.sub(&tmp2);

        let mut tmp3 = tmp4.mul(&zsquared);
        tmp3 = tmp3.add(&tmp3);
        tmp3 = tmp3.neg();
        let tmp6 = tmp6.square().sub(&tmp0).sub(&tmp5);
        let tmp1 = tmp1.add(&tmp1).add(&tmp1).add(&tmp1);
        let tmp6 = tmp6.sub(&tmp1);
        let tmp0 = r.z.mul(&zsquared);
        let tmp0 = tmp0.add(&tmp0);

        (tmp0, tmp3, tmp6)
    }

    // /// Compute the line function evaluation l_{T,Q}(P)
    // ///
    // /// Formula: l_{T,Q}(P) = (y_P - y_T) - λ(x_P - x_T)
    // /// where λ is the slope of the line through T and Q
    // fn line_evaluation(current: &mut G2Projective, g1_point: &G1Affine) -> Fp12 {
    //     let (x, y, z) = (&mut current.x, &mut current.y, &mut current.z);

    //     // Compute λ (slope)
    //     let slope = y.square().mul(&&x.mul_base(&Integer::from(3))).normalize();
    //     let vertical = y.mul(z).mul_base(&Integer::from(2)).normalize();
    //     let chord = z
    //         .square()
    //         .mul(&slope)
    //         .mul_base(&Integer::from(3))
    //         .normalize();

    //     // Update current point (doubling step)
    //     *x = slope
    //         .square()
    //         .sub(&vertical.mul_base(&Integer::from(2)))
    //         .normalize();
    //     *y = slope.mul(&vertical.sub(x)).sub(&chord).normalize();
    //     *z = vertical.mul(z).normalize();

    //     // Compute line evaluation
    //     let t1 = Fp2::from_base(&g1_point.y)
    //         .mul(z)
    //         .sub(&y.mul(&Fp2::from_base(&g1_point.x)))
    //         .normalize();
    //     let t2 = x
    //         .mul(&Fp2::from_base(&g1_point.x))
    //         .sub(&slope.mul(z))
    //         .normalize();

    //     Fp12::new(
    //         Fp6::new(t1, t2, Fp2::zero()),
    //         Fp6::new(vertical.clone(), Fp2::zero(), Fp2::zero()),
    //     )
    // }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::curves::bls12_381::{
        curves::{g1::G1Projective, g1_affine::G1Affine, g2::G2Projective, g2_affine::G2Affine},
        fields::target::TargetField,
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
        assert!(pairing != TargetField::one());

        let expected = G1Projective::generator().mul_scalar(&c.0).to_affine();

        assert_eq!(
            pairing,
            BLS12_381Pairing::pairing(&expected, &G2Affine::generator())
        );
        assert_eq!(
            pairing,
            BLS12_381Pairing::pairing(&G1Affine::generator(), &G2Affine::generator())
                .mul_scalar(&c.0)
        );
    }
}
