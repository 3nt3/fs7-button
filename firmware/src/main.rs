#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(riscv_ext_intrinsics)]

use ch32_hal::gpio::OutputOpenDrain;
use ch32_hal::println;
use defmt::info;
use embedded_hal::digital::InputPin;
use hal::delay::Delay;
use hal::gpio::{Input, Level, Output, Pull};

use crate::lanc::ButtonCmd;

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

    let mut camera_io = OutputOpenDrain::new(p.PC2, Level::High, ch32_hal::gpio::Speed::High);

    loop {
        // lanc::write_button(&mut camera_io, ButtonCmd::User4, &mut delay);

        let byte = ButtonCmd::User4.value().unwrap();

        lanc::write_lanc(
            &mut camera_io,
            &lanc::LancCmd {
                mode: lanc::THE_MODE,
                cmd: byte,
            },
            &mut delay,
        );

        delay.delay_ms(500);
    }
}
