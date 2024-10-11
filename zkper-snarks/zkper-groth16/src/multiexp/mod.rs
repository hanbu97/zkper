use std::sync::Arc;

use rug::Integer;
use zkper_curves::curves::bls12_381::curves::{
    g1::G1Projective, g1_affine::G1Affine, g2::G2Projective, g2_affine::G2Affine,
};

use crate::prover::Density;

struct BaseSource {
    bases: Arc<Vec<G1Affine>>,
    current_index: usize,
}

impl BaseSource {
    fn new(bases: Arc<Vec<G1Affine>>) -> Self {
        BaseSource {
            bases,
            current_index: 0,
        }
    }

    fn new_with_start_idx(bases: Arc<Vec<G1Affine>>, start_idx: usize) -> Self {
        BaseSource {
            bases,
            current_index: start_idx,
        }
    }

    fn next(&mut self) -> Option<&G1Affine> {
        if self.current_index < self.bases.len() {
            let base = &self.bases[self.current_index];
            self.current_index += 1;
            Some(base)
        } else {
            None
        }
    }

    fn skip(&mut self, amt: usize) {
        self.current_index += amt;
    }
}

struct BaseSourceG2 {
    bases: Arc<Vec<G2Affine>>,
    current_index: usize,
}

impl BaseSourceG2 {
    fn new(bases: Arc<Vec<G2Affine>>) -> Self {
        Self {
            bases,
            current_index: 0,
        }
    }

    fn new_with_start_idx(bases: Arc<Vec<G2Affine>>, start_idx: usize) -> Self {
        Self {
            bases,
            current_index: start_idx,
        }
    }

    fn next(&mut self) -> Option<&G2Affine> {
        if self.current_index < self.bases.len() {
            let base = &self.bases[self.current_index];
            self.current_index += 1;
            Some(base)
        } else {
            None
        }
    }

    fn skip(&mut self, amt: usize) {
        self.current_index += amt;
    }
}

/// Perform multi-exponentiation.
pub fn multiexp(
    bases: Arc<Vec<G1Affine>>,
    bases_start_idx: Option<usize>,
    density_map: Option<&Density>,
    exponents: Arc<Vec<Integer>>,
) -> anyhow::Result<G1Projective> {
    let mut acc = G1Projective::identity();
    // let mut base_iter = bases.iter();
    let mut base_source = if let Some(start_idx) = bases_start_idx {
        BaseSource::new_with_start_idx(bases, start_idx)
    } else {
        BaseSource::new(bases)
    };

    if let Some(density_map) = density_map {
        // If the density map has a known query size, it should not be
        // inconsistent with the number of exponents.
        assert_eq!(density_map.0.len(), exponents.len());

        for (exponent, density) in exponents.iter().zip(density_map.0.iter()) {
            if *density {
                if !exponent.is_zero() {
                    if let Some(base) = base_source.next() {
                        let mut current = G1Projective::from(base);
                        if *exponent != Integer::from(1) {
                            current = current.mul_scalar(exponent);
                        }
                        acc = acc.add(&current);
                    } else {
                        return Err(anyhow::anyhow!("Ran out of bases"));
                    }
                } else {
                    base_source.skip(1);
                }
            }
        }
    } else {
        for exponent in exponents.iter() {
            if !exponent.is_zero() {
                if let Some(base) = base_source.next() {
                    let mut current = G1Projective::identity().add(&base.into());

                    if *exponent != Integer::from(1) {
                        current = current.mul_scalar(exponent);
                    }
                    acc = acc.add(&current);
                } else {
                    return Err(anyhow::anyhow!("Ran out of bases"));
                }
            } else {
                base_source.skip(1);
            }
        }
    }

    Ok(acc)
}

/// Perform multi-exponentiation.
pub fn multiexp_g2(
    bases: Arc<Vec<G2Affine>>,
    bases_start_idx: Option<usize>,
    density_map: Option<&Density>,
    exponents: Arc<Vec<Integer>>,
) -> anyhow::Result<G2Projective> {
    let mut acc = G2Projective::identity();
    // let mut base_iter = bases.iter();
    let mut base_source = if let Some(start_idx) = bases_start_idx {
        BaseSourceG2::new_with_start_idx(bases, start_idx)
    } else {
        BaseSourceG2::new(bases)
    };

    if let Some(density_map) = density_map {
        // If the density map has a known query size, it should not be
        // inconsistent with the number of exponents.
        assert_eq!(density_map.0.len(), exponents.len());

        for (exponent, density) in exponents.iter().zip(density_map.0.iter()) {
            if *density {
                if !exponent.is_zero() {
                    if let Some(base) = base_source.next() {
                        let mut current = G2Projective::from(base);
                        if *exponent != Integer::from(1) {
                            current = current.mul_scalar(exponent);
                        }
                        acc = acc.add(&current);
                    } else {
                        return Err(anyhow::anyhow!("Ran out of bases"));
                    }
                } else {
                    base_source.skip(1);
                }
            }
        }
    } else {
        for exponent in exponents.iter() {
            if !exponent.is_zero() {
                if let Some(base) = base_source.next() {
                    let mut current = G2Projective::identity().add(&base.into());

                    if *exponent != Integer::from(1) {
                        current = current.mul_scalar(exponent);
                    }
                    acc = acc.add(&current);
                } else {
                    return Err(anyhow::anyhow!("Ran out of bases"));
                }
            } else {
                base_source.skip(1);
            }
        }
    }

    Ok(acc)
}
