use core::arch::riscv32::nop;

use embedded_hal::digital::{InputPin, OutputPin};
use hal::delay::Delay;
use hal::gpio::{Input, Level, Output, Pull};

use {ch32_hal as hal, panic_halt as _};

/// LANC command structure
pub struct LancCmd {
    pub mode: u8,
    pub cmd: u8,
}

/// LANC button commands
pub enum ButtonCmd {
    User4,
    User5,
    User6,
}

/// Magic mode for sending buttons. Unknown meaning
pub static THE_MODE: u8 = 0xd7;

impl ButtonCmd {
    pub fn value(&self) -> Result<u8, &str> {
        match self {
            ButtonCmd::User4 => Ok(0x48),
            ButtonCmd::User5 => Ok(0x49),
            ButtonCmd::User6 => Ok(0x50),
        }
    }
}

/// Write a button command to the LANC bus
///
/// # Arguments
/// * `io` - The GPIO pin connected to the LANC button (must be open-drain)
/// * `cmd` - The button command to send
/// * `delay` - A delay provider
pub fn write_button<P: InputPin + OutputPin>(io: &mut P, cmd: ButtonCmd, delay: &mut Delay) {
    let mode = THE_MODE;
    let cmd_value = cmd.value().unwrap();

    write_lanc(
        io,
        &LancCmd {
            mode,
            cmd: cmd_value,
        },
        delay,
    );
}

fn write_byte<P: InputPin + OutputPin>(io: &mut P, byte: u8, delay: &mut Delay) {
    let theoretical_delay_us = 104;
    let write_duration_us = 3;

    let delay_us = theoretical_delay_us - write_duration_us;

    for i in 0..8 {
        if (byte >> i) & 1 == 1 {
            io.set_high().ok();
        } else {
            io.set_low().ok();
        }
        delay.delay_us(delay_us);
    }
}

pub fn write_lanc<P: InputPin + OutputPin>(io: &mut P, cmd: &LancCmd, delay: &mut Delay) {
    let repeat_count = 30;

    for _ in 0..repeat_count {
        // wait for start bit
        loop {
            let last_low = embassy_time::Instant::now();
            while !io.is_low().unwrap() {
                nop();
            }
            let elapsed = last_low.elapsed();

            if elapsed.as_micros() > 5000 {
                // debug!("Start bit detected after {} us", elapsed.as_micros());
                break;
            }
        }
        delay.delay_us(104);

        write_byte(io, cmd.mode, delay);
        let _ = io.set_high();
        loop {
            let last_low = embassy_time::Instant::now();
            while !io.is_low().unwrap() {
                nop();
            }
            let elapsed = last_low.elapsed();

            if elapsed.as_micros() > 200 {
                break;
            }
        }
        delay.delay_us(104);
        write_byte(io, cmd.cmd, delay);
        let _ = io.set_high();
    }
}
