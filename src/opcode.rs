use crate::{Reg, Chip8State, INSTR_SIZE};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// Clear the display
    CLS,
    /// Return from subroutine
    RET,
    /// Jump to location nnn
    JP(u16),
    /// Call subroutine at nnn
    CALL(u16),
    /// Skip next instruction if Vx == kk
    SEVB(Reg, u8),
    /// Skip next instruction if Vx != kk
    SNEVB(Reg, u8),
    /// Skip next instruction if Vx == Vy
    SEVV(Reg, Reg),
    /// Set Vx = kk
    LDVB(Reg, u8),
    /// Set Vx = Vx + kk
    ADDVB(Reg, u8),
    /// Set Vx = Vy
    LDVV(Reg, Reg),
    /// Set Vx = Vx | Vy
    ORVV(Reg, Reg),
    /// Set Vx = Vx & Vy
    ANDVV(Reg, Reg),
    /// Set Vx = Vx ^ Vy
    XORVV(Reg, Reg),
    /// Set Vx = Vx + Vy; Set VF = carry
    ADDVV(Reg, Reg),
    /// Set VF = Vx > Vy; Set Vx = Vx - Vy;
    SUB(Reg, Reg),
    /// Set VF = lsb Vx; Set Vx = Vx >> 1
    SHR(Reg),
    /// Set VF = Vy > Vx; Set Vx = Vy - Vx
    SUBN(Reg, Reg),
    /// Set VF = msb Vx; Set Vx = Vx << 1
    SHL(Reg),
    /// Skip next instruction if Vx != Vy
    SNEVV(Reg, Reg),
    /// Set I = nnn
    LDI(u16),
    /// Jump to location nnn + V0
    JPV0(u16),
    /// Set Vx = Random byte & kk
    RND(Reg, u8),
    /// Display n-byte sprite starting at memory location I at (Vx, Vy); Set VF = collision
    DRW(Reg, Reg, u8),
    /// Skip next instruction if key with the value of Vx is pressed
    SKP(Reg),
    /// Skip next instruction if key with the value of Vx is not pressed
    SKNP(Reg),
    /// Set Vx = delay timer value
    LDVDT(Reg),
    /// Wait for a key press, store the value of the key in Vx
    LDVK(Reg),
    /// Set delay timer = Vx
    LDDT(Reg),
    /// Set sound timer = Vx
    LDST(Reg),
    /// Set I = I + Vx
    ADDI(Reg),
    /// Set I = location of sprite for digit Vx
    LDF(Reg),
    /// Store BCD representation of Vx in memory locations I, I + 1, and I + 2
    LDB(Reg),
    /// Store V0 to Vx in memory starting at address I; Set I = I + x + 1
    LDIV(Reg),
    /// Fill V0 to Vx with values from memory starting at address I; Set I = I + x + 1
    LDVI(Reg),
    /// No operation
    NOP,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0x00E0 => Self::CLS,
            0x00EE => Self::RET,
            0x1000..=0x1FFF => Self::JP(value & 0x0FFF),
            0x2000..=0x2FFF => Self::CALL(value & 0x0FFF),
            0x3000..=0x3FFF => Self::SEVB(Reg::from(((value & 0x0F00) >> 8) as u8), (value & 0x00FF) as u8),
            0x4000..=0x4FFF => Self::SNEVB(Reg::from(((value & 0x0F00) >> 8) as u8), (value & 0x00FF) as u8),
            0x5000..=0x5FFF => Self::SEVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
            0x6000..=0x6FFF => Self::LDVB(Reg::from(((value & 0x0F00) >> 8) as u8), (value & 0x00FF) as u8),
            0x7000..=0x7FFF => Self::ADDVB(Reg::from(((value & 0x0F00) >> 8) as u8), (value & 0x00FF) as u8),
            0x8000..=0x8FFF => {
                match value & 0x000F {
                    0x0 => Self::LDVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x1 => Self::ORVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x2 => Self::ANDVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x3 => Self::XORVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x4 => Self::ADDVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x5 => Self::SUB(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0x6 => Self::SHR(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x7 => Self::SUBN(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
                    0xE => Self::SHL(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    _ => Self::NOP,
                }
            },
            0x9000..=0x9FFF => Self::SNEVV(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8)),
            0xA000..=0xAFFF => Self::LDI(value & 0x0FFF),
            0xB000..=0xBFFF => Self::JPV0(value & 0x0FFF),
            0xC000..=0xCFFF => Self::RND(Reg::from(((value & 0x0F00) >> 8) as u8), (value & 0x00FF) as u8),
            0xD000..=0xDFFF => Self::DRW(Reg::from(((value & 0x0F00) >> 8) as u8), Reg::from(((value & 0x00F0) >> 4) as u8), (value & 0x000F) as u8),
            0xE000..=0xEFFF => {
                match value & 0x00FF {
                    0x009E => Self::SKP(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x00A1 => Self::SKNP(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    _ => Self::NOP,
                }
            },
            0xF000..=0xFFFF => {
                match value & 0x00FF {
                    0x0007 => Self::LDVDT(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x000A => Self::LDVK(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0015 => Self::LDDT(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0018 => Self::LDST(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x001E => Self::ADDI(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0029 => Self::LDF(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0033 => Self::LDB(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0055 => Self::LDIV(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    0x0065 => Self::LDVI(Reg::from(((value & 0x0F00) >> 8) as u8)),
                    _ => Self::NOP,
                }
            },
            _ => Self::NOP,
        }
    }
}

impl Opcode {
    fn execute(&self, state: &mut Chip8State) {
        match self {
            Opcode::CLS => state.framebuffer.fill(0),
            Opcode::RET => {
                let ret_addr = state.pop_stack();
                state.jump_to_address(ret_addr);
            },
            Opcode::JP(addr) => state.jump_to_address(*addr),
            Opcode::CALL(addr) => {
                let ret_addr = state.pc + u16::from(INSTR_SIZE);
                state.push_stack(ret_addr);
                state.jump_to_address(*addr);
            },
            Opcode::SEVB(reg, byte) => if state.registers[*reg as usize] == *byte { state.pc += u16::from(INSTR_SIZE) },
            Opcode::SNEVB(reg, byte) => if state.registers[*reg as usize] != *byte { state.pc += u16::from(INSTR_SIZE) },
            Opcode::SEVV(reg1, reg2) => if state.registers[*reg1 as usize] == state.registers[*reg2 as usize] { state.pc += u16::from(INSTR_SIZE) },
            Opcode::LDVB(_, _) => todo!(),
            Opcode::ADDVB(_, _) => todo!(),
            Opcode::LDVV(_, _) => todo!(),
            Opcode::ORVV(_, _) => todo!(),
            Opcode::ANDVV(_, _) => todo!(),
            Opcode::XORVV(_, _) => todo!(),
            Opcode::ADDVV(_, _) => todo!(),
            Opcode::SUB(_, _) => todo!(),
            Opcode::SHR(_) => todo!(),
            Opcode::SUBN(_, _) => todo!(),
            Opcode::SHL(_) => todo!(),
            Opcode::SNEVV(_, _) => todo!(),
            Opcode::LDI(_) => todo!(),
            Opcode::JPV0(_) => todo!(),
            Opcode::RND(_, _) => todo!(),
            Opcode::DRW(_, _, _) => todo!(),
            Opcode::SKP(_) => todo!(),
            Opcode::SKNP(_) => todo!(),
            Opcode::LDVDT(_) => todo!(),
            Opcode::LDVK(_) => todo!(),
            Opcode::LDDT(_) => todo!(),
            Opcode::LDST(_) => todo!(),
            Opcode::ADDI(_) => todo!(),
            Opcode::LDF(_) => todo!(),
            Opcode::LDB(_) => todo!(),
            Opcode::LDIV(_) => todo!(),
            Opcode::LDVI(_) => todo!(),
            Opcode::NOP => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_u16() {
        assert_eq!(Opcode::from(0x00E0), Opcode::CLS);
        assert_eq!(Opcode::from(0x00EE), Opcode::RET);
        assert_eq!(Opcode::from(0x1FFF), Opcode::JP(0x0FFF));
        assert_eq!(Opcode::from(0x2123), Opcode::CALL(0x0123));
        assert_eq!(Opcode::from(0x35AB), Opcode::SEVB(Reg::V5, 0xAB));
        assert_eq!(Opcode::from(0x4F12), Opcode::SNEVB(Reg::VF, 0x12));
        assert_eq!(Opcode::from(0x5010), Opcode::SEVV(Reg::V0, Reg::V1));
        assert_eq!(Opcode::from(0x6AEE), Opcode::LDVB(Reg::VA, 0xEE));
        assert_eq!(Opcode::from(0x7F42), Opcode::ADDVB(Reg::VF, 0x42));
        assert_eq!(Opcode::from(0x89A0), Opcode::LDVV(Reg::V9, Reg::VA));
        assert_eq!(Opcode::from(0x8CD1), Opcode::ORVV(Reg::VC, Reg::VD));
        assert_eq!(Opcode::from(0x8322), Opcode::ANDVV(Reg::V3, Reg::V2));
        assert_eq!(Opcode::from(0x8773), Opcode::XORVV(Reg::V7, Reg::V7));
        assert_eq!(Opcode::from(0x8004), Opcode::ADDVV(Reg::V0, Reg::V0));
        assert_eq!(Opcode::from(0x8FE5), Opcode::SUB(Reg::VF, Reg::VE));
        assert_eq!(Opcode::from(0x8AB6), Opcode::SHR(Reg::VA));
        assert_eq!(Opcode::from(0x8AB7), Opcode::SUBN(Reg::VA, Reg::VB));
        assert_eq!(Opcode::from(0x844E), Opcode::SHL(Reg::V4));
        assert_eq!(Opcode::from(0x9560), Opcode::SNEVV(Reg::V5, Reg::V6));
        assert_eq!(Opcode::from(0xA380), Opcode::LDI(0x0380));
        assert_eq!(Opcode::from(0xB747), Opcode::JPV0(0x0747));
        assert_eq!(Opcode::from(0xC172), Opcode::RND(Reg::V1, 0x72));
        assert_eq!(Opcode::from(0xD789), Opcode::DRW(Reg::V7, Reg::V8, 0x09));
        assert_eq!(Opcode::from(0xE89E), Opcode::SKP(Reg::V8));
        assert_eq!(Opcode::from(0xE9A1), Opcode::SKNP(Reg::V9));
        assert_eq!(Opcode::from(0xFF07), Opcode::LDVDT(Reg::VF));
        assert_eq!(Opcode::from(0xF00A), Opcode::LDVK(Reg::V0));
        assert_eq!(Opcode::from(0xF115), Opcode::LDDT(Reg::V1));
        assert_eq!(Opcode::from(0xF218), Opcode::LDST(Reg::V2));
        assert_eq!(Opcode::from(0xF31E), Opcode::ADDI(Reg::V3));
        assert_eq!(Opcode::from(0xF429), Opcode::LDF(Reg::V4));
        assert_eq!(Opcode::from(0xF533), Opcode::LDB(Reg::V5));
        assert_eq!(Opcode::from(0xF655), Opcode::LDIV(Reg::V6));
        assert_eq!(Opcode::from(0xF765), Opcode::LDVI(Reg::V7))
    }

    #[test]
    fn test_op_cls() {
        let mut state = Chip8State::default();

        state.framebuffer[0x12] = 0xFF;
        state.framebuffer[0x00] = 0xAB;
        state.framebuffer[0xFF] = 0xCD;

        Opcode::CLS.execute(&mut state);

        assert_eq!(state.framebuffer[0x12], 0x00);
        assert_eq!(state.framebuffer[0x00], 0x00);
        assert_eq!(state.framebuffer[0xFF], 0x00)
    }

    #[test]
    fn test_op_jmp() {
        let mut state = Chip8State::default();

        state.memory[0x300] = 0x61;
        state.memory[0x301] = 0x23;

        Opcode::JP(0x300).execute(&mut state);

        assert_eq!(state.decode_opcode(), Opcode::LDVB(Reg::V1, 0x23))
    }

    #[test]
    fn test_op_call() {
        let mut state = Chip8State::default();

        Opcode::CALL(0x0123).execute(&mut state);

        assert_eq!(state.stack[0], 0x02);
        assert_eq!(state.stack[1], 0x02);
        assert_eq!(state.sp, 0x02);
        assert_eq!(state.pc, 0x0123)
    }

    #[test]
    fn test_op_ret() {
        let mut state = Chip8State::default();

        Opcode::CALL(0x0ABC).execute(&mut state);
        Opcode::RET.execute(&mut state);

        assert_eq!(state.sp, 0x00);
        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_sevb() {
        let mut state = Chip8State::default();

        state.registers[Reg::V0 as usize] = 0x78;

        Opcode::SEVB(Reg::V0, 0x78).execute(&mut state);
        assert_eq!(state.pc, 0x0202);

        Opcode::SEVB(Reg::V0, 0x22).execute(&mut state);
        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_snevb() {
        let mut state = Chip8State::default();

        state.registers[Reg::V0 as usize] = 0x78;

        Opcode::SNEVB(Reg::V0, 0x78).execute(&mut state);
        assert_eq!(state.pc, 0x0200);
        
        Opcode::SNEVB(Reg::V0, 0x22).execute(&mut state);
        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_sevv() {
        let mut state = Chip8State::default();

        state.registers[Reg::V0 as usize] = 0x78;
        state.registers[Reg::V2 as usize] = 0x78;

        Opcode::SEVV(Reg::V0, Reg::V1).execute(&mut state);
        assert_eq!(state.pc, 0x0200);
        
        Opcode::SEVV(Reg::V0, Reg::V2).execute(&mut state);
        assert_eq!(state.pc, 0x0202)
    }
}