use cortex_m::asm::nop;
use stm32f1::stm32f103::Peripherals;

pub fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let rcc = &p.RCC;
    let gpio = &p.GPIOC;

    rcc.apb2enr.modify(|_, w| w.iopcen().set_bit());
    gpio.crh.modify(|_, w| w.mode13().output());
    gpio.crh.modify(|_, w| w.cnf13().open_drain());

    let mut is_on: bool = false;
    loop {
        gpio.odr.modify(|_, w| w.odr13().bit(is_on));

        for _ in 0..500_000 {
            nop();
        }

        is_on = !is_on;
    }
}
