use rand::{CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub struct ZkperRng(ChaCha20Rng);

impl ZkperRng {
    pub fn new() -> Self {
        Self(ChaCha20Rng::from_entropy())
    }

    pub fn new_test() -> Self {
        Self::from_seed(1234567890)
    }

    pub fn from_seed(seed: u64) -> Self {
        Self(ChaCha20Rng::seed_from_u64(seed))
    }
}

impl RngCore for ZkperRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl CryptoRng for ZkperRng {}
