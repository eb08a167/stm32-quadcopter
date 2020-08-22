#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal::stm32;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    loop {
        //cortex_m::asm::wfi();
    }
}
