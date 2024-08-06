use num::{Float, FromPrimitive};

use super::floats::FloatNumber;

// TODO: Add validation for "divide by 0" protection.

/// Returns a percentage of the original number.
/// E.g.: subtract_fractional_percentage(150, 20) == 30
pub fn subtract_percentage<TFloat: Float + FromPrimitive>(
    total: TFloat,
    percentage: TFloat,
) -> TFloat {
    subtract_fractional_percentage(total, percentage / FloatNumber::one_hundred())
}

/// Returns a percentage of the original number.
/// E.g.: subtract_fractional_percentage(150, 0.20) == 30
pub fn subtract_fractional_percentage<TFloat: Float + FromPrimitive>(
    total: TFloat,
    fractional_percentage: TFloat,
) -> TFloat {
    total * fractional_percentage
}

/// Returns the percentage representation in 0-100 format.
/// E.g.: percentage_of_total(10, 100) == 10
pub fn percentage_of_total<TFloat: Float + FromPrimitive>(part: TFloat, total: TFloat) -> TFloat {
    (part * FloatNumber::one_hundred()) / total
}

/// Returns the percentage representation in 0-1 format.
/// E.g.: fractional_percentage_of_total(10, 100) == 0.1
pub fn fractional_percentage_of_total<TFloat: Float + FromPrimitive>(
    part: TFloat,
    total: TFloat,
) -> TFloat {
    percentage_of_total(part, total) / FloatNumber::one_hundred()
}
