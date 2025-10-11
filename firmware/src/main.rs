#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(riscv_ext_intrinsics)]

use hal::delay::Delay;
use hal::gpio::{Input, Level, Output, Pull};

use {ch32_hal as hal, panic_halt as _};

mod lanc;

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSE;
    let p = hal::init(config);

    let mut delay = Delay;

    let mut led1 = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC3, Level::High, Default::default());

    let btn1 = Input::new(p.PD2, Pull::Up);
    let btn2 = Input::new(p.PD3, Pull::Up);
    let btn3 = Input::new(p.PD4, Pull::Up);
    let switch = Input::new(p.PD5, Pull::Up);

    let mut i = 10;

    loop {
        if i == 0 {
            led1.toggle();
            led2.toggle();
            i = 10;
        } else {
            i -= 1;
        }

        delay.delay_ms(10);

        // delay.delay_ms(100);
        let val = hal::pac::SYSTICK.cnt().read();
        hal::println!(
            "{} {} {} {}",
            btn1.is_low(),
            btn2.is_low(),
            btn3.is_low(),
            switch.is_low()
        );
    }
}
