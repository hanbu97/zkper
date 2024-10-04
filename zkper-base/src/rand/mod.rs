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

// use rand_chacha::ChaCha20Rng;
// use rand_core::{RngCore, SeedableRng};

// pub struct RandRng;
// pub struct SeedRng;

// pub struct ZkperRng(ChaCha20Rng);

// impl ZkperRng {
//     pub fn new() -> Self {
//         Self(ChaCha20Rng::from_entropy())
//     }

//     pub fn new_with_seed_u64(seed: u64) -> Self {
//         Self(ChaCha20Rng::seed_from_u64(seed))
//     }

//     pub fn new_test() -> Self {
//         Self::new_with_seed_u64(1234567890)
//     }
// }

// impl RandRng {
//     pub fn rng() -> ZkperRng {
//         ZkperRng::new()
//     }
// }

// impl SeedRng {
//     pub fn rng(seed: u64) -> ZkperRng {
//         ZkperRng::new_with_seed_u64(seed)
//     }
// }

// impl RngCore for ZkperRng {
//     fn next_u32(&mut self) -> u32 {
//         self.0.next_u32()
//     }

//     fn next_u64(&mut self) -> u64 {
//         self.0.next_u64()
//     }

//     fn fill_bytes(&mut self, dest: &mut [u8]) {
//         self.0.fill_bytes(dest)
//     }

//     fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
//         self.0.try_fill_bytes(dest)
//     }
// }

// pub trait ZkperRandom: Sized {
//     fn ergo_random<R: RngCore + ?Sized>(rng: &mut R) -> Self;
// }

// use rand::Rng;

// impl<T> ZkperRandom for T
// where
//     rand::distributions::Standard: rand::distributions::Distribution<T>,
// {
//     fn ergo_random<R: RngCore + ?Sized>(rng: &mut R) -> Self {
//         rng.sample(rand::distributions::Standard)
//     }
// }

// macro_rules! create_type_rng {
//     ($name:ident, $type:ty) => {
//         pub struct $name;
//         impl $name {
//             pub fn gen<R: RngCore>(rng: &mut R) -> $type {
//                 <$type as ZkperRandom>::ergo_random(rng)
//             }
//         }
//     };
// }

// create_type_rng!(U8Rand, u8);
// create_type_rng!(U16Rand, u16);
// create_type_rng!(U32Rand, u32);
// create_type_rng!(U64Rand, u64);
// create_type_rng!(U128Rand, u128);
// create_type_rng!(I8Rand, i8);
// create_type_rng!(I16Rand, i16);
// create_type_rng!(I32Rand, i32);
// create_type_rng!(I64Rand, i64);
// create_type_rng!(I128Rand, i128);
// create_type_rng!(F32Rand, f32);
// create_type_rng!(F64Rand, f64);
// create_type_rng!(BoolRand, bool);

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rand::RngCore;

//     #[test]
//     fn test_rand() {
//         let mut rng = RandRng::rng();
//         let _u8 = U8Rand::gen(&mut rng);
//         let _u16 = U16Rand::gen(&mut rng);
//         let _u32 = U32Rand::gen(&mut rng);
//         let _u64 = U64Rand::gen(&mut rng);
//         let _u128 = U128Rand::gen(&mut rng);
//         let _i8 = I8Rand::gen(&mut rng);
//         let _i16 = I16Rand::gen(&mut rng);
//         let _i32 = I32Rand::gen(&mut rng);
//         let _i64 = I64Rand::gen(&mut rng);
//         let _i128 = I128Rand::gen(&mut rng);
//         let _f32 = F32Rand::gen(&mut rng);
//         let _f64 = F64Rand::gen(&mut rng);
//         let _bool = BoolRand::gen(&mut rng);

//         println!("u8: {}", _u8);
//         println!("u16: {}", _u16);
//         println!("u32: {}", _u32);
//         println!("u64: {}", _u64);
//         println!("u128: {}", _u128);
//         println!("i8: {}", _i8);
//         println!("i16: {}", _i16);
//         println!("i32: {}", _i32);
//         println!("i64: {}", _i64);
//         println!("i128: {}", _i128);
//         println!("f32: {}", _f32);
//         println!("f64: {}", _f64);
//         println!("bool: {}", _bool);
//     }

//     #[test]
//     fn test_seed() {
//         let seed = 1234567890;
//         let mut rng = SeedRng::rng(seed);
//         let _u8 = U8Rand::gen(&mut rng);
//         let _u16 = U16Rand::gen(&mut rng);
//         let _u32 = U32Rand::gen(&mut rng);
//         let _u64 = U64Rand::gen(&mut rng);
//         let _u128 = U128Rand::gen(&mut rng);
//         let _i8 = I8Rand::gen(&mut rng);
//         let _i16 = I16Rand::gen(&mut rng);
//         let _i32 = I32Rand::gen(&mut rng);
//         let _i64 = I64Rand::gen(&mut rng);
//         let _i128 = I128Rand::gen(&mut rng);
//         let _f32 = F32Rand::gen(&mut rng);
//         let _f64 = F64Rand::gen(&mut rng);
//         let _bool = BoolRand::gen(&mut rng);

//         println!("u8: {}", _u8);
//         println!("u16: {}", _u16);
//         println!("u32: {}", _u32);
//         println!("u64: {}", _u64);
//         println!("u128: {}", _u128);
//         println!("i8: {}", _i8);
//         println!("i16: {}", _i16);
//         println!("i32: {}", _i32);
//         println!("i64: {}", _i64);
//         println!("i128: {}", _i128);
//         println!("f32: {}", _f32);
//         println!("f64: {}", _f64);
//         println!("bool: {}", _bool);
//     }

//     #[test]
//     fn test_fill_bytes() {
//         let mut rng = RandRng::rng();
//         let mut bytes = [0u8; 10];
//         rng.fill_bytes(&mut bytes);
//         println!("{:?}", bytes);

//         let mut rng = SeedRng::rng(1234567890);
//         let mut bytes = [0u8; 10];
//         rng.fill_bytes(&mut bytes);
//         println!("{:?}", bytes);

//         rng.fill_bytes(&mut bytes);
//         println!("{:?}", bytes);
//     }
// }
