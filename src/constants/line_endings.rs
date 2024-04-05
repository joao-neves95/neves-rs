pub struct LineEndings {}

impl LineEndings {
    pub const CRLF_STR: &'static str = "\r\n";
    pub const CRLF_BYTES: &'static [u8] = b"\r\n";

    pub const LF_STR: &'static str = "\n";
    pub const LF_CHAR: char = '\n';
    pub const LF_CHAR_BYTE: u8 = b'\n';
    pub const LF_BYTES: &'static [u8] = b"\n";

    pub const CR_STR: &'static str = "\r";
    pub const CR_CHAR: char = '\r';
    pub const CR_CHAR_BYTE: u8 = b'\r';
    pub const CR_BYTES: &'static [u8] = b"\r";
}
