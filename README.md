# chip8emu
Chip-8 emulator as a Rust library.

The main crate is the emulator code itself. The `chip8debug` crate is a terminal UI interactive debugger for the emulator (see below).

![chip8debugScreenshot](https://github.com/tac550/chip8emu/assets/9357248/785fe2f1-9888-4fe5-ab51-436e22820c1e)

Does not yet emulate the precise timing of any particular machine.

## Debugger

When running the debugger (`chip8debug` crate) pass the path to a chip8 ROM as the first command line argument to load a ROM.

# Building

To build a dynamically-linking emulator library using the C ABI (for interoperability with other languages) use the following command on the root crate: `cargo rustc --release --crate-type=cdylib`
