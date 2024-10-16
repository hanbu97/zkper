use rand::RngCore;
use rug::{
    rand::{RandGen, ThreadRandGen},
    Integer,
};

use crate::ZkperRng;

impl RandGen for ZkperRng {
    fn gen(&mut self) -> u32 {
        self.next_u32()
    }

    fn gen_bits(&mut self, bits: u32) -> u32 {
        if bits == 0 {
            0
        } else if bits < 32 {
            self.next_u32() >> (32 - bits)
        } else {
            self.next_u32()
        }
    }

    fn seed(&mut self, seed: &Integer) {
        let seed_u64 = seed.to_u64_wrapping();
        *self = ZkperRng::from_seed(seed_u64);
    }
}

impl ThreadRandGen for ZkperRng {
    fn gen(&mut self) -> u32 {
        self.next_u32()
    }

    fn gen_bits(&mut self, bits: u32) -> u32 {
        if bits == 0 {
            0
        } else if bits < 32 {
            self.next_u32() >> (32 - bits)
        } else {
            self.next_u32()
        }
    }

    fn seed(&mut self, seed: &Integer) {
        let seed_u64 = seed.to_u64_wrapping();
        *self = ZkperRng::from_seed(seed_u64);
    }
}
