#[repr(C)]
pub struct Chip8State {
    left: i32,
    right: i32,
}

#[no_mangle]
pub extern "C" fn chip8_add(state: Chip8State) -> i32 {
    state.left + state.right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_add() {
        let state = Chip8State {left: 2, right: 2};
        let result = chip8_add(state);
        assert_eq!(result, 4);
    }
}
