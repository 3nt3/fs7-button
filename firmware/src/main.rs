#![no_std]
#![no_main]

mod fmt;

use cortex_m::delay::Delay;
use defmt::{debug, info, warn};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, OutputOpenDrain, Pull, Speed},
    rcc::{self, Hse},
    time::Hertz,
};
use embedded_hal::digital::{InputPin, OutputPin};

struct LancCmd {
    pub mode: u8,
    pub cmd: u8,
}

enum ButtonCmd {
    User4,
    Invalid,
}

// magic mode for sending buttons. idk what it means
static THE_MODE: u8 = 0xd7;

impl ButtonCmd {
    fn value(&self) -> Result<u8, &str> {
        match self {
            ButtonCmd::User4 => Ok(0x48),
            ButtonCmd::Invalid => Err("invalid button"),
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(8),
        mode: rcc::HseMode::Oscillator,
    });
    config.rcc.pll = Some(rcc::Pll {
        src: rcc::PllSource::HSE,
        prediv: rcc::PllPreDiv::DIV1,
        mul: rcc::PllMul::MUL9, // 8 MHz * 9 = 72 MHz
    });

    let p = embassy_stm32::init(config);

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let mut input = ExtiInput::new(p.PB11, p.EXTI11, Pull::Up);
    let mut io = OutputOpenDrain::new(p.PB10, Level::High, Speed::Low);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 72_000_00);

    // let mut usart_config = Config::default();
    // usart_config.baudrate = 9600;
    // usart_config.parity = embassy_stm32::usart::Parity::ParityNone;

    // // let mut usart = UartRx::new_blocking(p.USART3, p.PB11, usart_config).unwrap();
    // let mut usart = UartTx::new_blocking(p.USART3, p.PB10, usart_config).unwrap();

    // let packet = [0xd7, 0x48];
    let packet = [0xd7, 0x48];

    let start = embassy_time::Instant::now();
    loop {
        write_button(&mut io, &mut input, ButtonCmd::User4, &mut delay);
        led.set_low();

        delay.delay_ms(500);

        // write_byte(&mut io, packet[0], &mut delay);
    }
}

fn write_button<P: OutputPin, I: InputPin>(
    out: &mut P,
    input: &mut I,
    cmd: ButtonCmd,
    delay: &mut Delay,
) {
    let mode = THE_MODE;
    let cmd_value = cmd.value().unwrap();

    write_lanc(
        out,
        input,
        &LancCmd {
            mode,
            cmd: cmd_value,
        },
        delay,
    );
}

fn write_byte<P: OutputPin>(out: &mut P, byte: u8, delay: &mut Delay) {
    let theoretical_delay_us = 104;
    let write_duration_us = 19;

    let delay_us = theoretical_delay_us - write_duration_us;

    for i in 0..8 {
        if (byte >> i) & 1 == 1 {
            out.set_high().ok();
        } else {
            out.set_low().ok();
        }
        delay.delay_us(delay_us);
    }
}

fn write_lanc<P: OutputPin, I: InputPin>(
    out: &mut P,
    input: &mut I,
    cmd: &LancCmd,
    delay: &mut Delay,
) {
    let repeat_count = 30;

    for i in 0..repeat_count {
        // wait for start bit
        // debug!("Waiting for start bit...");
        // debug!("Iteration {}", i + 1);
        loop {
            let last_low = embassy_time::Instant::now();
            while !input.is_low().unwrap() {
                cortex_m::asm::nop(); // wait for low signal
            }
            let elapsed = last_low.elapsed();

            if elapsed.as_micros() > 5000 {
                // debug!("Start bit detected after {} us", elapsed.as_micros());
                break;
            }
        }
        delay.delay_us(104);

        write_byte(out, cmd.mode, delay);
        out.set_high();
        // // delay.delay_us(10);
        //
        loop {
            let last_low = embassy_time::Instant::now();
            while !input.is_low().unwrap() {
                cortex_m::asm::nop(); // wait for low signal
            }
            let elapsed = last_low.elapsed();

            if elapsed.as_micros() > 200 {
                // debug!("Start bit detected after {} us", elapsed.as_micros());
                break;
            }
        }
        delay.delay_us(104);
        write_byte(out, cmd.cmd, delay);
        out.set_high();

        // // debug!("Waiting for new stop bit...");
        // // while input.is_low().unwrap() {
        // //     cortex_m::asm::nop(); // wait for low signal
        // // }
        // // debug!("Sending command byte...");
        // delay.delay_us(104 * 5);
        //
        // write_byte(out, cmd.cmd, delay);
        //
        // out.set_high().ok();
    }
}
