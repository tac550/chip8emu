/// Shift left without risk of overflow errors
pub const fn shl_no(val: u8, shift: usize) -> u8 {
    [val << (shift & 7), 0][((shift & !7) != 0) as usize]
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCD {
    pub hundreds: u8,
    pub tens: u8,
    pub ones: u8,
}

impl From<u8> for BCD {
    fn from(value: u8) -> Self {
        let ones = value % 10;
        let value = value / 10;
        let tens = value % 10;
        let value = value / 10;
        let hundreds = value % 10;

        Self { hundreds, tens, ones }
    }
}

impl BCD {
    #[cfg(test)]
    pub fn new(hundreds: u8, tens: u8, ones: u8) -> Self {
        Self { hundreds, tens, ones }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shl_no() {
        assert_eq!(shl_no(5, 6), 64);
        assert_eq!(shl_no(5, 8), 0)
    }

    #[test]
    fn test_bcd_from_u8() {
        assert_eq!(BCD::from(0), BCD::new(0, 0, 0));
        assert_eq!(BCD::from(5), BCD::new(0, 0, 5));
        assert_eq!(BCD::from(50), BCD::new(0, 5, 0));
        assert_eq!(BCD::from(255), BCD::new(2, 5, 5));
        assert_eq!(BCD::from(202), BCD::new(2, 0, 2));
        assert_eq!(BCD::from(123), BCD::new(1, 2, 3))
    }
}