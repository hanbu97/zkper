use super::*;

#[derive(Clone, Debug)]
pub struct G1Projective {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

impl fmt::Display for G1Projective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "G1Projective {{\n    x: {},\n    y: {},\n    z: {}\n}}",
                self.x, self.y, self.z
            )
        } else {
            write!(f, "G1Projective({}, {}, {})", self.x, self.y, self.z)
        }
    }
}

impl G1Projective {
    pub fn new(x: Integer, y: Integer, z: Integer) -> Self {
        Self { x, y, z }
    }
}

impl G1Projective {
    /// Returns the identity element (point at infinity).
    pub fn identity() -> Self {
        G1Projective {
            x: Bls12_381BaseField::zero(),
            y: Bls12_381BaseField::one(),
            z: Bls12_381BaseField::zero(),
        }
    }

    /// Returns true if this element is the identity (the point at infinity).
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        // loop {
        let x: Bls12_381BaseField = rng.gen();

        let flip_sign = rng.next_u32() % 2 != 0;

        // Compute y = sqrt(x^3 + 4)
        // let y_squared = x.square().mul(&x).add_u64(4);

        Self::identity()

        // }
    }
}

#[cfg(test)]
mod tests {
    use zkper_base::rand::ZkperRng;

    use super::*;

    #[test]
    fn test_g1_projective_random() {
        let mut rng = ZkperRng::new_test();
        let random = G1Projective::random(&mut rng);

        println!("random: {:#}", random);
        // assert!(!random.is_identity());
    }
}
