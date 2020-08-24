#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4::stm32f446 as device;

#[cortex_m_rt::entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = device::Peripherals::take().unwrap();

    // Setup flash
    dp.FLASH.acr.modify(|_, w| {
        unsafe { w.latency().bits(0b1111) };
        w.prften().set_bit();
        w.icen().set_bit();
        w.dcen().set_bit()
    });

    // Enable HSE
    dp.RCC.cr.modify(|_, w| w.hseon().set_bit());
    while dp.RCC.cr.read().hserdy().bit_is_clear() {}

    // Configure PLL
    unsafe {
        dp.RCC.pllcfgr.modify(|_, w| {
            w.pllsrc().hse();
            w.pllm().bits(0b10000);
            w.plln().bits(0b101101000);
            w.pllp().bits(0b0);
            w.pllq().bits(0b0111)
        });
    }
    dp.RCC.cr.modify(|_, w| w.pllon().set_bit());
    while dp.RCC.cr.read().pllrdy().bit_is_clear() {}

    // Select PLL as system clock
    dp.RCC.cfgr.modify(|_, w| {
        w.hpre().div1();
        w.ppre1().div4();
        w.ppre2().div2();
        w.sw().pll()
    });
    while dp.RCC.cfgr.read().sws() != device::rcc::cfgr::SWS_A::PLL {}

    loop {
        cortex_m::asm::nop();
    }
}
