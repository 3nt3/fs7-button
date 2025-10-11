use core::arch::riscv32::nop;

use embedded_hal::digital::{InputPin, OutputPin};
use hal::delay::Delay;
use hal::gpio::{Input, Level, Output, Pull};

use {ch32_hal as hal, panic_halt as _};

pub struct LancCmd {
    pub mode: u8,
    pub cmd: u8,
}

pub enum ButtonCmd {
    User4,
    Invalid,
}

// magic mode for sending buttons. idk what it means
pub static THE_MODE: u8 = 0xd7;

impl ButtonCmd {
    fn value(&self) -> Result<u8, &str> {
        match self {
            ButtonCmd::User4 => Ok(0x48),
            ButtonCmd::Invalid => Err("invalid button"),
        }
    }
}

pub fn write_byte<P: OutputPin>(out: &mut P, byte: u8, delay: &mut Delay) {
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

pub fn write_lanc<P: OutputPin, I: InputPin>(
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
                nop();
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
                nop();
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
