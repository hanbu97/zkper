use crate::traits::field::FieldTrait;
use rand::{
    distributions::{Distribution, Standard},
    RngCore,
};
use rug::Integer;
use std::fmt::Display;

pub mod base;
pub mod fp12;
pub mod fp2;
pub mod fp6;
pub mod scalar;
pub mod target;
