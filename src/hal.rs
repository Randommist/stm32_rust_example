use cortex_m::asm::nop;
use hal::pac;
use stm32f1xx_hal::{self as hal, gpio::GpioExt};

pub fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut gpioc = p.GPIOC.split();
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut is_on: bool = false;
    loop {
        pc13.toggle();

        for _ in 0..1_000_000 {
            nop();
        }

        is_on = !is_on;
    }
}
