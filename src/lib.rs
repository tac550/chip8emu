/// For conveniently accessing registers in a 16-byte buffer
enum Reg {
    V0 = 0, V1 = 1, V2 = 2, V3 = 3, V4 = 4, V5 = 5, V6 = 6, V7 = 7,
    V8 = 8, V9 = 9, VA = 10, VB = 11, VC = 12, VD = 13, VE = 14, VF = 15,
}

#[repr(C)]
pub struct Chip8State {
    registers: [u8; 16],    // General-Purpose Registers
    index: u16,             // Index Register
    stack: [u8; 64],        // 64-Byte Stack
    sp: u8,                 // Stack pointer
    pc: u16,                // Program Counter
    dt: u8,                 // Delay Timer
    st: u8,                 // Sound Timer
    framebuffer: [u8; 256], // 64x32-Bit Frame Buffer (Monochrome)

    memory: [u8; 4096],     // 4K Memory; Programs start at 0x200
}

impl Default for Chip8State {
    fn default() -> Self {
        Self { registers: Default::default(), index: Default::default(), stack: [0; 64], sp: Default::default(), pc: Default::default(), dt: Default::default(), st: Default::default(), framebuffer: [0; 256], memory: [0; 4096] }
    }
}

impl Chip8State {
    fn read_instruction(&self, addr: u16) -> u16 {
        let addr = addr % 4096;
        let hi = self.memory[addr as usize];
        let lo = self.memory[(addr + 1) as usize];
        lo as u16 | ((hi as u16) << 8)
    }

    fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn chip8_tick(state: &mut Chip8State) -> i32 {
    state.read_instruction(0x0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8() {

    }

    #[test]
    fn test_read_instruction() {
        let mut state = Chip8State::default();

        state.memory[0x0] = 0x12;
        state.memory[0x1] = 0x34;
        
        assert_eq!(state.read_instruction(0x0), 0x1234);
    }

    #[test]
    fn test_decrement_timers() {
        let mut state = Chip8State::default();

        state.dt = 1;
        state.st = 10;
        state.decrement_timers();

        assert_eq!((state.dt, state.st), (0, 9));

        for _ in 0..10 {
            state.decrement_timers();
        }

        assert_eq!((state.dt, state.st), (0, 0))
    }
}
