#![no_std]
#![no_main]

mod hal;
mod mcu;
mod pac;

use cortex_m_rt::entry;
use hal::main as hal_main;
use mcu::main as mcu_main;
use pac::main as pac_main;
use panic_halt as _;

#[entry]
fn main() -> ! {
    mcu_main();
    // pac_main();
    // hal_main();
}
