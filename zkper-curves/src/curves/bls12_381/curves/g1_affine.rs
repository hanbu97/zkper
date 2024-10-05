use super::*;

#[derive(Clone, Debug)]
pub struct G1Affine {
    pub x: Integer,
    pub y: Integer,
    pub infinity: bool,
}

impl fmt::Display for G1Affine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "G1Affine {{\n    x: {},\n    y: {},\n    infinity: {}\n}}",
                self.x, self.y, self.infinity
            )
        } else {
            write!(f, "G1Affine({}, {}, {})", self.x, self.y, self.infinity)
        }
    }
}

impl G1Affine {
    pub fn new(x: Integer, y: Integer, infinity: bool) -> Self {
        Self { x, y, infinity }
    }
}

impl G1Affine {
    /// Returns true if this point is the point at infinity.
    #[inline]
    pub fn is_identity(&self) -> bool {
        self.infinity
    }

    /// Returns the identity element (point at infinity).
    #[inline]
    pub fn identity() -> Self {
        G1Affine {
            x: Bls12_381BaseField::zero(),
            y: Bls12_381BaseField::one(),
            infinity: true,
        }
    }
}
