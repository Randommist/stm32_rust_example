use cortex_m::asm::nop;

pub fn main() -> ! {
    const RCC_APB2ENR: *mut u32 = 0x4002_1018 as *mut u32;
    const GPIO_C_CNF_LOW: *mut u32 = 0x4001_1000 as *mut u32;
    const GPIO_C_CNF_HIGH: *mut u32 = 0x4001_1004 as *mut u32;
    const GPIO_C_ODR: *mut u32 = 0x4001_100C as *mut u32;

    unsafe {
        // write_volatile(RCC_APB2ENR, 0b_0000_0000_0000_0000_0000_0000_0001_0000);
        *RCC_APB2ENR |= 0b_0000_0000_0000_0000_0000_0000_0001_0000;

        let cnf = *GPIO_C_CNF_HIGH | 0b_0000_0000_0010_0000_0000_0000_0000_0000;
        // write_volatile(GPIO_C_CNF_HIGH, cnf);
        *GPIO_C_CNF_HIGH = cnf;
    }

    let mut is_on: bool = false;
    loop {
        unsafe {
            // write_volatile(GPIO_C_ODR, (is_on as u32) << 13);
            *GPIO_C_ODR = (is_on as u32) << 13;
        }

        for _ in 0..100_000 {
            nop();
        }

        is_on = !is_on;
    }
}
