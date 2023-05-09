use crate::Reg;

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
    /// Set VF = Vy . Vx; Set Vx = Vy - Vx
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
            _ => Self::NOP,
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
    }
}