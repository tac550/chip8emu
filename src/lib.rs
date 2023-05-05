#[repr(C)]
pub struct Chip8State {
    left: u8,
    right: u8,
    total: u8,

    memory: [u8; 4096],
}

#[no_mangle]
pub extern "C" fn chip8_add(state: &mut Chip8State) -> i32 {
    state.total = state.left + state.right;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_add() {
        let mut state = Chip8State {left: 2, right: 2, total: 0, memory: [0; 4096]};
        chip8_add(&mut state);
        assert_eq!(state.total, 4);
    }
}
