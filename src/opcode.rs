use rand::random;

use crate::{Reg, Chip8State, INSTR_SIZE, util::BCD};

#[derive(Debug, PartialEq)]
pub enum WaitStatus {
    Waiting,
    Running,
}

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
    fn execute(&self, state: &mut Chip8State) -> WaitStatus {
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
            Opcode::LDVB(reg, byte) => state.registers[*reg as usize] = *byte,
            Opcode::ADDVB(reg, byte) => state.registers[*reg as usize] += *byte,
            Opcode::LDVV(reg1, reg2) => state.registers[*reg1 as usize] = state.registers[*reg2 as usize],
            Opcode::ORVV(reg1, reg2) => state.registers[*reg1 as usize] |= state.registers[*reg2 as usize],
            Opcode::ANDVV(reg1, reg2) => state.registers[*reg1 as usize] &= state.registers[*reg2 as usize],
            Opcode::XORVV(reg1, reg2) => state.registers[*reg1 as usize] ^= state.registers[*reg2 as usize],
            Opcode::ADDVV(reg1, reg2) => {
                let result = state.registers[*reg1 as usize].wrapping_add(state.registers[*reg2 as usize]);
                state.registers[Reg::VF as usize] = u8::from(result < state.registers[*reg1 as usize]);
                state.registers[*reg1 as usize] = result;
            },
            Opcode::SUB(reg1, reg2) => {
                state.registers[Reg::VF as usize] = u8::from(state.registers[*reg1 as usize] > state.registers[*reg2 as usize]);
                state.registers[*reg1 as usize] = state.registers[*reg1 as usize].wrapping_sub(state.registers[*reg2 as usize]);
            },
            Opcode::SHR(reg) => {
                state.registers[Reg::VF as usize] = state.registers[*reg as usize] & 0x01;
                state.registers[*reg as usize] >>= 1;
            },
            Opcode::SUBN(reg1, reg2) => {
                state.registers[Reg::VF as usize] = u8::from(state.registers[*reg2 as usize] > state.registers[*reg1 as usize]);
                state.registers[*reg1 as usize] = state.registers[*reg2 as usize].wrapping_sub(state.registers[*reg1 as usize]);
            },
            Opcode::SHL(reg) => {
                state.registers[Reg::VF as usize] = (state.registers[*reg as usize] & 0x80) >> 7;
                state.registers[*reg as usize] <<= 1;
            },
            Opcode::SNEVV(reg1, reg2) => if state.registers[*reg1 as usize] != state.registers[*reg2 as usize] { state.pc += u16::from(INSTR_SIZE) },
            Opcode::LDI(val) => state.index = *val,
            Opcode::JPV0(addr) => state.jump_to_address((addr & 0x0FFF) + u16::from(state.registers[Reg::V0 as usize])),
            Opcode::RND(reg, mask) => state.registers[*reg as usize] = random::<u8>() & mask,
            Opcode::DRW(x_reg, y_reg, rows) => {
                let mut overwrite = false;
                for i in 0..*rows {
                    overwrite |= state.write_fb(
                        state.memory[(state.index + u16::from(i)) as usize],
                        state.registers[*x_reg as usize],
                        state.registers[*y_reg as usize] + i
                    );
                }
                state.registers[Reg::VF as usize] = u8::from(overwrite);
            },
            Opcode::SKP(reg) => if state.read_input(u16::from(state.registers[*reg as usize])) { state.pc += u16::from(INSTR_SIZE) },
            Opcode::SKNP(reg) => if !state.read_input(u16::from(state.registers[*reg as usize])) { state.pc += u16::from(INSTR_SIZE) },
            Opcode::LDVDT(reg) => state.registers[*reg as usize] = state.dt,
            Opcode::LDVK(reg) =>
            if state.input == 0 {
                return WaitStatus::Waiting
            } else {
                state.registers[*reg as usize] = state.input.trailing_zeros() as u8
            },
            Opcode::LDDT(reg) => state.dt = state.registers[*reg as usize],
            Opcode::LDST(reg) => state.st = state.registers[*reg as usize],
            Opcode::ADDI(reg) => state.index += u16::from(state.registers[*reg as usize]),
            Opcode::LDF(reg) => state.index = u16::from(state.registers[*reg as usize]) * 5,
            Opcode::LDB(reg) => state.store_bcd(BCD::from(state.registers[*reg as usize]), state.index),
            Opcode::LDIV(_) => todo!(),
            Opcode::LDVI(_) => todo!(),
            Opcode::NOP => todo!(),
        }
        
        WaitStatus::Running
    }
}

#[cfg(test)]
mod tests {
    use crate::sprite::DEFAULT_SPRITES;

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
    fn test_op_jp() {
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

    #[test]
    fn test_op_ldvb() {
        let mut state = Chip8State::default();

        Opcode::LDVB(Reg::VA, 0x12).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0x12)
    }

    #[test]
    fn test_op_addvb() {
        let mut state = Chip8State::default();

        Opcode::ADDVB(Reg::VB, 0x02).execute(&mut state);
        Opcode::ADDVB(Reg::VB, 0x03).execute(&mut state);

        assert_eq!(state.registers[Reg::VB as usize], 0x05)
    }

    #[test]
    fn test_op_ldvv() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        Opcode::LDVV(Reg::V6, Reg::V0).execute(&mut state);

        assert_eq!(state.registers[Reg::V6 as usize], 0x55)
    }

    #[test]
    fn test_op_orvv() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        state.registers[Reg::V1 as usize] = 0x25;
        Opcode::ORVV(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x75)
    }

    #[test]
    fn test_op_andvv() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        state.registers[Reg::V1 as usize] = 0x25;
        Opcode::ANDVV(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x05)
    }

    #[test]
    fn test_op_xorvv() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        state.registers[Reg::V1 as usize] = 0x27;
        Opcode::XORVV(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x72)
    }

    #[test]
    fn test_op_addvv() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        state.registers[Reg::V1 as usize] = 0x27;
        Opcode::ADDVV(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x7C);
        assert_eq!(state.registers[Reg::VF as usize], 0x00);

        state.registers[Reg::VA as usize] = 0x55;
        state.registers[Reg::VB as usize] = 0xC7;
        Opcode::ADDVV(Reg::VA, Reg::VB).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0x1C);
        assert_eq!(state.registers[Reg::VF as usize], 0x01)
    }

    #[test]
    fn test_op_sub() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x55;
        state.registers[Reg::V1 as usize] = 0x27;
        Opcode::SUB(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x2E);
        assert_eq!(state.registers[Reg::VF as usize], 0x01);

        state.registers[Reg::VA as usize] = 0x55;
        state.registers[Reg::VB as usize] = 0x56;
        Opcode::SUB(Reg::VA, Reg::VB).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0xFF);
        assert_eq!(state.registers[Reg::VF as usize], 0x00)
    }

    #[test]
    fn test_op_shr() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x00;
        Opcode::SHR(Reg::V0).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x00);
        assert_eq!(state.registers[Reg::VF as usize], 0x00);

        state.registers[Reg::VA as usize] = 0x01;
        Opcode::SHR(Reg::VA).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0x00);
        assert_eq!(state.registers[Reg::VF as usize], 0x01);

        state.registers[Reg::VE as usize] = 0x40;
        Opcode::SHR(Reg::VE).execute(&mut state);

        assert_eq!(state.registers[Reg::VE as usize], 0x20);
        assert_eq!(state.registers[Reg::VF as usize], 0x00)
    }

    #[test]
    fn test_op_subn() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x27;
        state.registers[Reg::V1 as usize] = 0x55;
        Opcode::SUBN(Reg::V0, Reg::V1).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x2E);
        assert_eq!(state.registers[Reg::VF as usize], 0x01);

        state.registers[Reg::VA as usize] = 0x56;
        state.registers[Reg::VB as usize] = 0x55;
        Opcode::SUBN(Reg::VA, Reg::VB).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0xFF);
        assert_eq!(state.registers[Reg::VF as usize], 0x00)
    }

    #[test]
    fn test_op_shl() {
        let mut state = Chip8State::default();
        
        state.registers[Reg::V0 as usize] = 0x00;
        Opcode::SHL(Reg::V0).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0x00);
        assert_eq!(state.registers[Reg::VF as usize], 0x00);

        state.registers[Reg::VA as usize] = 0x80;
        Opcode::SHL(Reg::VA).execute(&mut state);

        assert_eq!(state.registers[Reg::VA as usize], 0x00);
        assert_eq!(state.registers[Reg::VF as usize], 0x01);

        state.registers[Reg::VE as usize] = 0x7F;
        Opcode::SHL(Reg::VE).execute(&mut state);

        assert_eq!(state.registers[Reg::VE as usize], 0xFE);
        assert_eq!(state.registers[Reg::VF as usize], 0x00)
    }

    #[test]
    fn test_op_snevv() {
        let mut state = Chip8State::default();

        state.registers[Reg::V0 as usize] = 0x78;
        state.registers[Reg::V2 as usize] = 0x78;

        Opcode::SNEVV(Reg::V0, Reg::V1).execute(&mut state);
        assert_eq!(state.pc, 0x0202);
        
        Opcode::SNEVV(Reg::V0, Reg::V2).execute(&mut state);
        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_ldi() {
        let mut state = Chip8State::default();

        Opcode::LDI(0x0ABC).execute(&mut state);
        assert_eq!(state.index, 0x0ABC)
    }

    #[test]
    fn test_op_jpv0() {
        let mut state = Chip8State::default();

        state.registers[Reg::V0 as usize] = 0xA0;
        Opcode::JPV0(0x0ABC).execute(&mut state);
        assert_eq!(state.pc, 0x0B5C)
    }

    #[test]
    fn test_op_rnd() {
        let mut state = Chip8State::default();

        Opcode::RND(Reg::V0, 0x0F).execute(&mut state);
        assert!(state.registers[Reg::V0 as usize] <= 0x0F);

        Opcode::RND(Reg::V0, 0x03).execute(&mut state);
        assert!(state.registers[Reg::V0 as usize] <= 0x03)
    }

    #[test]
    fn test_op_drw() {
        let mut state = Chip8State::default();

        Opcode::DRW(Reg::V0, Reg::V1, 0x05).execute(&mut state);
        for i in 0..5 {
            assert_eq!(state.framebuffer[(8 * i) + 0], DEFAULT_SPRITES[0].rows[i]);
        }
        assert_eq!(state.registers[Reg::VF as usize], 0x00);

        Opcode::LDI(0x005).execute(&mut state);
        Opcode::DRW(Reg::V0, Reg::V1, 0x05).execute(&mut state);
        for i in 0..5 {
            assert_eq!(state.framebuffer[(8 * i) + 0], DEFAULT_SPRITES[0].rows[i] ^ DEFAULT_SPRITES[1].rows[i]);
        }
        assert_eq!(state.registers[Reg::VF as usize], 0x01);

        state.registers[Reg::V0 as usize] = 0x10;
        state.registers[Reg::V1 as usize] = 0x10;
        Opcode::LDI(0x00F).execute(&mut state);
        Opcode::DRW(Reg::V0, Reg::V1, 0x0A).execute(&mut state);
        for i in 0..10 {
            assert_eq!(state.framebuffer[(8 * (i + 16)) + 2], DEFAULT_SPRITES[if i < 5 { 3 } else { 4 }].rows[i % 5]);
        }
        assert_eq!(state.registers[Reg::VF as usize], 0x00);
    }

    #[test]
    fn test_op_skp() {
        let mut state = Chip8State::default();

        state.input = 0b0000000000001000;
        state.registers[Reg::V5 as usize] = 0x03;

        Opcode::SKP(Reg::V5).execute(&mut state);

        assert_eq!(state.pc, 0x0202);

        Opcode::SKP(Reg::V6).execute(&mut state);

        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_sknp() {
        let mut state = Chip8State::default();

        state.input = 0b0000000000001000;
        state.registers[Reg::V5 as usize] = 0x03;

        Opcode::SKNP(Reg::V5).execute(&mut state);

        assert_eq!(state.pc, 0x0200);

        Opcode::SKNP(Reg::V6).execute(&mut state);

        assert_eq!(state.pc, 0x0202)
    }

    #[test]
    fn test_op_ldvdt() {
        let mut state = Chip8State::default();

        state.dt = 0xAB;

        Opcode::LDVDT(Reg::V0).execute(&mut state);

        assert_eq!(state.registers[Reg::V0 as usize], 0xAB)
    }

    #[test]
    fn test_op_ldvk() {
        let mut state = Chip8State::default();

        state.input = 0b0000000000000000;

        assert_eq!(Opcode::LDVK(Reg::V0).execute(&mut state), WaitStatus::Waiting);
        assert_eq!(state.registers[Reg::V0 as usize], 0x00);

        state.input = 0b0000000000001000;

        assert_eq!(Opcode::LDVK(Reg::V0).execute(&mut state), WaitStatus::Running);
        assert_eq!(state.registers[Reg::V0 as usize], 0x03);

        state.input = 0b0000100001100000;

        assert_eq!(Opcode::LDVK(Reg::V0).execute(&mut state), WaitStatus::Running);
        assert_eq!(state.registers[Reg::V0 as usize], 0x05);

        state.input = 0b0000100001100001;

        assert_eq!(Opcode::LDVK(Reg::V0).execute(&mut state), WaitStatus::Running);
        assert_eq!(state.registers[Reg::V0 as usize], 0x00)
    }

    #[test]
    fn test_op_lddt() {
        let mut state = Chip8State::default();

        state.registers[Reg::V3 as usize] = 0x12;
        Opcode::LDDT(Reg::V3).execute(&mut state);

        assert_eq!(state.dt, 0x12)
    }

    #[test]
    fn test_op_ldst() {
        let mut state = Chip8State::default();

        state.registers[Reg::V3 as usize] = 0x12;
        Opcode::LDST(Reg::V3).execute(&mut state);

        assert_eq!(state.st, 0x12)
    }

    #[test]
    fn test_op_addi() {
        let mut state = Chip8State::default();

        state.index = 0x0123;
        state.registers[Reg::V4 as usize] = 0x30;
        Opcode::ADDI(Reg::V4).execute(&mut state);

        assert_eq!(state.index, 0x0153)
    }

    #[test]
    fn test_op_ldf() {
        let mut state = Chip8State::default();

        state.registers[Reg::V5 as usize] = 0x0;
        Opcode::LDF(Reg::V5).execute(&mut state);

        assert_eq!(state.index, 0x00);

        state.registers[Reg::V5 as usize] = 0x2;
        Opcode::LDF(Reg::V5).execute(&mut state);

        assert_eq!(state.index, 0x0A)
    }

    #[test]
    fn test_op_ldb() {
        let mut state = Chip8State::default();

        state.index = 0x0300;
        state.registers[Reg::V6 as usize] = 0x89;

        Opcode::LDB(Reg::V6).execute(&mut state);

        assert_eq!(state.memory[0x0300], 0x01);
        assert_eq!(state.memory[0x0301], 0x03);
        assert_eq!(state.memory[0x0302], 0x07)
    }
}