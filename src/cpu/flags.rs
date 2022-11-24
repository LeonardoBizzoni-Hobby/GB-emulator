#[derive(Copy, Clone)]
pub struct FlagsRegister {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << 7
            | (if flag.subtraction { 1 } else { 0 }) << 6
            | (if flag.half_carry { 1 } else { 0 }) << 5
            | (if flag.carry { 1 } else { 0 }) << 4
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero: bool = (byte >> 7) != 0;
        let subtraction: bool = (byte >> 6) != 0;
        let half_carry: bool = (byte >> 5) != 0;
        let carry: bool = (byte >> 4) != 0;

        FlagsRegister {
            zero,
            subtraction,
            half_carry,
            carry,
        }
    }
}
