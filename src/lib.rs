#[repr(C)]
pub struct Chip8State {
    left: u8,
    right: u8,
}

#[no_mangle]
pub extern "C" fn chip8_add(state: &mut Chip8State) -> i32 {
    (state.left + state.right).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_add() {
        let mut state = Chip8State {left: 2, right: 2};
        let result = chip8_add(&mut state);
        assert_eq!(result, 4);
    }
}
