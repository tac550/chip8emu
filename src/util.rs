/// Shift left without risk of overflow errors
pub const fn shl_no(val: u8, shift: usize) -> u8 {
    [val << (shift & 7), 0][((shift & !7) != 0) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shl_no() {
        assert_eq!(shl_no(5, 6), 64);
        assert_eq!(shl_no(5, 8), 0)
    }
}