/// Shift left without risk of overflow errors
pub const fn shl_no(val: u8, shift: usize) -> u8 {
    [val << (shift & 7), 0][((shift & !7) != 0) as usize]
}

#[derive(Debug, PartialEq)]
pub struct BCD {
    hundreds: u8,
    tens: u8,
    ones: u8,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shl_no() {
        assert_eq!(shl_no(5, 6), 64);
        assert_eq!(shl_no(5, 8), 0)
    }

    #[test]
    fn test_u8_to_bcd() {
        assert_eq!(BCD::from(0), BCD {hundreds: 0, tens: 0, ones: 0});
        assert_eq!(BCD::from(5), BCD {hundreds: 0, tens: 0, ones: 5});
        assert_eq!(BCD::from(50), BCD {hundreds: 0, tens: 5, ones: 0});
        assert_eq!(BCD::from(255), BCD {hundreds: 2, tens: 5, ones: 5});
        assert_eq!(BCD::from(202), BCD {hundreds: 2, tens: 0, ones: 2});
        assert_eq!(BCD::from(123), BCD {hundreds: 1, tens: 2, ones: 3})
    }
}