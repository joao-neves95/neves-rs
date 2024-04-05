pub fn parse_u32_from_char_slice(source: &[char]) -> Option<u32> {
    let mut parsed_int = 0;

    for i in 0..source.len() {
        let this_int = match source[i].to_digit(10) {
            None => return None,
            Some(x) => x,
        };

        parsed_int = match concat_u32(parsed_int, this_int) {
            None => return None,
            Some(x) => x,
        }
    }

    Some(parsed_int)
}

pub fn parse_u32_from_u8_chars_slice(source: &[u8]) -> Option<u32> {
    let mut parsed_int = 0;

    for i in 0..source.len() {
        let this_int = match (source[i] as char).to_digit(10) {
            None => return None,
            Some(x) => x,
        };

        parsed_int = match concat_u32(parsed_int, this_int) {
            None => return None,
            Some(x) => x,
        }
    }

    Some(parsed_int)
}
