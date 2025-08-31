#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal::gpio::{Input, Pull};
use hal::delay::Delay;
use hal::gpio::{Level, Output};

use {ch32_hal as hal, panic_halt as _};

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    let mut delay = Delay;

    let mut led1 = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC3, Level::High, Default::default());

    let btn1 = Input::new(p.PD2, Pull::Up);
    let btn2 = Input::new(p.PD3, Pull::Up);
    let btn3 = Input::new(p.PD4, Pull::Up);
    let switch = Input::new(p.PD5, Pull::Up);

    loop {
        led1.toggle();
        led2.toggle();

        delay.delay_ms(100);

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
