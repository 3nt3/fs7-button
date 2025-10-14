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

/// LANC module
///
/// [LANC](https://en.wikipedia.org/wiki/LANC) is a bidirectional serial open collector communication port, where two devices can communicate with each other. The camcorder or still video camera is able to receive commands and sends back its status.
/// A write up of various commands and the protocol in general has been attempted by
/// [boehmel.de](https://www.boehmel.de/lanc.htm), even though none of the commands used by my
/// [PXW-FS7](https://pro.sony/ue_US/products/handheld-camcorders/pxw-fs7) are listed there lol.
///
/// # Example
/// ```rust
/// use ch32_hal::{self as hal, gpio::{OutputOpenDrain, Level}};
///
/// let mut camera_io = OutputOpenDrain::new(p.PC2, Level::High, ch32_hal::gpio::Speed::High);
///
/// lanc::write_button(&mut camera_io, lanc::ButtonCmd::User4, &mut delay);
/// ```
pub mod lanc;

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSE;
    let p = hal::init(config);

    let mut delay = Delay;

    let mut led1 = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC3, Level::Low, Default::default());

    let btn1 = Input::new(p.PD4, Pull::Up);
    let btn2 = Input::new(p.PD3, Pull::Up);
    let btn3 = Input::new(p.PD2, Pull::Up);
    let switch = Input::new(p.PD5, Pull::Up);

    let mut camera_io = OutputOpenDrain::new(p.PC2, Level::High, ch32_hal::gpio::Speed::High);

    loop {
        if btn1.is_low() || btn2.is_low() || btn3.is_low() {
            led1.set_high();
            led2.set_high();
        } else {
            led1.set_low();
            led2.set_low();
        }

        if btn1.is_low() {
            info!("btn1 pressed");
            lanc::write_button(&mut camera_io, ButtonCmd::User4, &mut delay);
        }
        if btn2.is_low() {
            info!("btn2 pressed");
            lanc::write_button(&mut camera_io, ButtonCmd::User5, &mut delay);
        }
        if btn3.is_low() {
            info!("btn3 pressed");
            lanc::write_button(&mut camera_io, ButtonCmd::User6, &mut delay);
        }

        delay.delay_ms(10);
    }
}
