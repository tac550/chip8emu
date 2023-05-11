use crate::Chip8State;

const DEF_SPRITE_HEIGHT: usize = 5;

pub struct DefSprite {
    rows: [u8; DEF_SPRITE_HEIGHT],
}

static DEFAULT_SPRITES: &[DefSprite] = &[
    DefSprite {
        rows: [
            0b11110000,
            0b10010000,
            0b10010000,
            0b10010000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b00100000,
            0b01100000,
            0b00100000,
            0b00100000,
            0b01110000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b00010000,
            0b11110000,
            0b10000000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b00010000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b10010000,
            0b10010000,
            0b11110000,
            0b00010000,
            0b00010000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b00010000,
            0b00100000,
            0b01000000,
            0b01000000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b10010000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b00010000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10010000,
            0b11110000,
            0b10010000,
            0b10010000,
        ],
    },
    DefSprite {
        rows: [
            0b11100000,
            0b10010000,
            0b11100000,
            0b10010000,
            0b11100000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10000000,
            0b10000000,
            0b10000000,
            0b11110000,
        ],
    },
    DefSprite {
        rows: [
            0b11100000,
            0b10010000,
            0b10010000,
            0b10010000,
            0b11100000,
        ],
    },
    DefSprite {
        rows: [
            0b11110000,
            0b10000000,
            0b11110000,
            0b10000000,
            0b11110000,
        ],
    },
    DefSprite {
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
    for s in 0..16 {
        for i in 0..DEF_SPRITE_HEIGHT {
            state.memory[s * DEF_SPRITE_HEIGHT + i] = DEFAULT_SPRITES[s].rows[i];
        }
    }
}