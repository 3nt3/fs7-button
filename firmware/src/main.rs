#![no_std]
#![no_main]

mod fmt;

use defmt::{info, warn};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{
    exti::{Channel, ExtiInput},
    gpio::{Input, Level, Output, OutputOpenDrain, Pin, Pull, Speed},
    rcc::{self, Hse},
    time::Hertz,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(8),
        mode: rcc::HseMode::Oscillator,
    });

    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let mut input = ExtiInput::new(p.PB11, p.EXTI11, Pull::Up);
    let mut io = OutputOpenDrain::new(p.PB10, Level::High, Speed::Low);

    // let mut usart_config = Config::default();
    // usart_config.baudrate = 9600;
    // usart_config.parity = embassy_stm32::usart::Parity::ParityNone;

    // // let mut usart = UartRx::new_blocking(p.USART3, p.PB11, usart_config).unwrap();
    // let mut usart = UartTx::new_blocking(p.USART3, p.PB10, usart_config).unwrap();

    let packet = [0xd7, 0x48, 0xff, 0xff, 0xfe, 0x4f, 0xff, 0xff];

    loop {
        let mut last_low = embassy_time::Instant::now();
        input.wait_for_low().await;
        let elapsed = last_low.elapsed();

        // detect if start bit
        if elapsed > embassy_time::Duration::from_millis(5) {
            info!("Start bit detected, sending packet");

            // pull low for 100ms
            io.set_low();
            embassy_time::Timer::after(embassy_time::Duration::from_micros(2000)).await;
            io.set_high();
        }
    }
}
