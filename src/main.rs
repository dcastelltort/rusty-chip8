
#![feature(fs_read_write)] //TO REMOVE with ugraded nightly or stable

#[macro_use] extern crate failure;

pub mod cpu;

use cpu::Chip8;

fn main() {
    println!("Rusty CHIP 8 Emulator");

    let mut emulator = Chip8::new();
    emulator.boot("placeholderfileame.rom");
    emulator.run();
}
