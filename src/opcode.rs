use crate::Reg;

#[allow(clippy::upper_case_acronyms)]
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
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}