pub fn concat_u32(left: u32, right: u32) -> Option<u32> {
    let pow_result = 10u32.checked_pow(u32_count(right));

    if pow_result.is_none() {
        return None;
    }

    Some((left * pow_result.unwrap()) + right)
}

pub fn u32_count(value: u32) -> u32 {
    let mut counter = 1;
    let mut width_meter = 10;

    while width_meter <= value {
        width_meter *= 10;
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn passes() {}
}
