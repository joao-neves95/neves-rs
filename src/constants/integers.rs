pub struct U8number {}

impl U8number {
    pub const ZERO: u32 = 0;
    pub const ONE: u32 = 1;
    pub const FIVE: u8 = 5;
    pub const SEVEN: u8 = 7;
}

pub struct U32number {}

impl U32number {
    pub const ZERO: u32 = U8number::ZERO as u32;
    pub const ONE: u32 = U8number::ONE as u32;
}

pub struct U128number {}

impl U128number {
    pub const ZERO: u128 = U8number::ZERO as u128;
    pub const ONE: u128 = U8number::ONE as u128;
}

pub struct U128PrimeNumbers {}

impl U128PrimeNumbers {
    pub const ZERO: u128 = U8number::ZERO as u128;
    pub const ONE: u128 = U8number::ONE as u128;
    pub const FIVE: u128 = U8number::FIVE as u128;
    pub const SEVEN: u128 = U8number::SEVEN as u128;
}
