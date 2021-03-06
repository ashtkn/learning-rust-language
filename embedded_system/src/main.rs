#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    let perip = stm32f407::Peripherals::take().unwrap();
    perip.RCC.ahb1enr.modify(|_, w| w.gpioden().set_bit());

    let gpiod = &perip.GPIOD;
    gpiod.moder.modify(|_, w| w.moder12().output());
    gpiod.moder.modify(|_, w| w.moder13().output());
    gpiod.moder.modify(|_, w| w.moder14().output());
    gpiod.moder.modify(|_, w| w.moder15().output());

    loop {
        gpiod.bsrr.write(|w| w.bs12().set_bit());
        gpiod.bsrr.write(|w| w.bs13().set_bit());
        gpiod.bsrr.write(|w| w.bs14().set_bit());
        gpiod.bsrr.write(|w| w.bs15().set_bit());

        delay(8_000_000);

        gpiod.bsrr.write(|w| w.br12().reset());
        gpiod.bsrr.write(|w| w.br13().reset());
        gpiod.bsrr.write(|w| w.br14().reset());
        gpiod.bsrr.write(|w| w.br15().reset());

        delay(8_000_000);
    }
}
