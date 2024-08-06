use crate::constants::floats::F32number;

use num::{Float, FromPrimitive, ToPrimitive};

pub struct FloatNumber {}

impl FloatNumber {
    pub fn one_hundred<TFloat: Float + FromPrimitive>() -> TFloat {
        FloatNumber::from_primitive(F32number::ONE_HUNDRED)
    }

    pub fn from_primitive<TPrimitive: ToPrimitive, TFloat: Float + FromPrimitive>(
        num: TPrimitive,
    ) -> TFloat {
        TFloat::from(num).unwrap()
    }
}
