use crate::Chip8State;

const DEF_SPRITE_HEIGHT: usize = 5;

pub struct DefaultSize {
    pub rows: [u8; DEF_SPRITE_HEIGHT],
}

#[allow(clippy::unreadable_literal)]
pub static DEFAULT_SPRITES: &[DefaultSize; 16] = &[
    DefaultSize {
        rows: [
            0b11110000,
            0b10010000,
            0b10010000,
            0b10010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b00100000,
            0b01100000,
            0b00100000,
            0b00100000,
            0b01110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b00010000,
            0b11110000,
            0b10000000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b00010000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b10010000,
            0b10010000,
            0b11110000,
            0b00010000,
            0b00010000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10000000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10000000,
            0b11110000,
            0b10010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b00010000,
            0b00100000,
            0b01000000,
            0b01000000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b10010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b10010000,
            0b10010000,
        ],
    },
    DefaultSize {
        rows: [
            0b11100000,
            0b10010000,
            0b11100000,
            0b10010000,
            0b11100000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10000000,
            0b10000000,
            0b10000000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11100000,
            0b10010000,
            0b10010000,
            0b10010000,
            0b11100000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10000000,
            0b11110000,
            0b10000000,
            0b11110000,
        ],
    },
    DefaultSize {
        rows: [
            0b11110000,
            0b10000000,
            0b11110000,
            0b10000000,
            0b10000000,
        ],
    },
];

pub fn store_default_sprites(state: &mut Chip8State) {
    for (s, sprite) in DEFAULT_SPRITES.iter().enumerate() {
        for i in 0..DEF_SPRITE_HEIGHT {
            state.memory[s * DEF_SPRITE_HEIGHT + i] = sprite.rows[i];
        }
    }
}