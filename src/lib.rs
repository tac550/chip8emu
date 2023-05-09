mod opcode;
mod util;

/// For conveniently accessing registers in a 16-byte buffer
#[derive(Debug, PartialEq)]
pub enum Reg {
    V0 = 0, V1 = 1, V2 = 2, V3 = 3, V4 = 4, V5 = 5, V6 = 6, V7 = 7,
    V8 = 8, V9 = 9, VA = 10, VB = 11, VC = 12, VD = 13, VE = 14, VF = 15,
}

impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => Self::V0, 1 => Self::V1, 2 => Self::V2, 3 => Self::V3,
            4 => Self::V4, 5 => Self::V5, 6 => Self::V6, 7 => Self::V7,
            8 => Self::V8, 9 => Self::V9, 10 => Self::VA, 11 => Self::VB,
            12 => Self::VC, 13 => Self::VD, 14 => Self::VE, _ => Self::VF,
        }
    }
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
        Self { registers: Default::default(), index: Default::default(), stack: [0; 64],
            sp: Default::default(), pc: Default::default(), dt: Default::default(),
            st: Default::default(), framebuffer: [0; 256], memory: [0; 4096] }
    }
}

impl Chip8State {
    fn read_instruction(&self, addr: u16) -> u16 {
        let addr = addr % 4096;
        let hi = self.memory[addr as usize];
        let lo = self.memory[(addr + 1) as usize];
        u16::from(lo) | (u16::from(hi) << 8)
    }

    fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    /// XORs one byte to the framebuffer at specified location.
    /// Returns true if any active bit was overwritten.
    /// 
    /// # Arguments
    ///
    /// * `x` and `y` coordinates - The leftmost destination pixel of the byte being drawn
    fn write_fb(&mut self, pixels: u8, x: u8, y: u8) -> bool {
        let x_offset = (x % 8) as usize; // First operation because we shadow the original value of x after this.
        let x = x / 8; // Eliminate non-byte-aligned values for calculating relevant byte indices in fb

        let x = x % 8;
        let y = y % 32;

        let first_index = ((8 * y) + x) as usize;

        if x < 7 {
            let second_index = first_index + 1;

            let left_pixels = pixels >> x_offset;
            let right_pixels = util::shl_no(pixels, 8 - x_offset);
            let left_oldval = self.framebuffer[first_index];
            let right_oldval = self.framebuffer[second_index];

            self.framebuffer[first_index] = left_oldval ^ left_pixels;
            self.framebuffer[second_index] = right_oldval ^ right_pixels;

            left_oldval & left_pixels != 0 || right_oldval & right_pixels != 0
        } else {
            let pixels = pixels >> x_offset;
            let oldval = self.framebuffer[first_index];
            self.framebuffer[first_index] = oldval ^ pixels;
    
            oldval & pixels != 0
        }
    }
}

#[no_mangle]
pub extern "C" fn chip8_tick(state: &mut Chip8State) -> i32 {
    i32::from(state.read_instruction(0x0))
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
    
    #[test]
    fn test_write_fb_byte_aligned() {
        let mut state = Chip8State::default();

        assert!(!state.write_fb(0xFF, 0, 0));
        assert!(!state.write_fb(0xAB, 8, 1));
        assert!(!state.write_fb(0x12, 56, 31));
        assert!(state.write_fb(0x2, 56, 31));

        assert_eq!(state.framebuffer[0], 0xFF);
        assert_eq!(state.framebuffer[9], 0xAB);
        assert_eq!(state.framebuffer[255], 0x10)
    }
    #[test]
    fn test_write_fb_not_byte_aligned() {
        let mut state = Chip8State::default();

        assert!(!state.write_fb(0xFF, 1, 0));
        assert!(state.write_fb(0xEF, 5, 0));
        assert!(!state.write_fb(0xFF, 59, 10));

        assert_eq!(state.framebuffer[0], 0x78);
        assert_eq!(state.framebuffer[1], 0xF8);
        assert_eq!(state.framebuffer[87], 0x1F);
        assert_eq!(state.framebuffer[88], 0x00)
    }
}
