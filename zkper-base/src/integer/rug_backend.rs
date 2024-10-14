use super::traits::ZkperIntegerTrait;
use num_traits::Pow;
use rug::Integer;

pub struct RugBackend(pub Integer);

impl From<Integer> for RugBackend {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl ZkperIntegerTrait for RugBackend {
    fn from_hex_str(hex_str: &str) -> Self {
        let value =
            Integer::from_str_radix(hex_str.strip_prefix("0x").unwrap_or(hex_str), 16).unwrap();
        Self(value)
    }

    fn from_u64(u: u64) -> Self {
        let value = Integer::from(u);
        Self(value)
    }

    fn add(&self, other: &Self) -> Self {
        Self(self.0.clone() + &other.0)
    }

    fn sub(&self, other: &Self) -> Self {
        Self(self.0.clone() - &other.0)
    }

    fn mul(&self, other: &Self) -> Self {
        Self(self.0.clone() * &other.0)
    }

    fn div(&self, other: &Self) -> Self {
        Self(self.0.clone() / &other.0)
    }

    fn neg(&self) -> Self {
        Self((-&self.0).into())
    }

    fn sqrt(&self) -> Self {
        Self(self.0.clone().sqrt())
    }
}
