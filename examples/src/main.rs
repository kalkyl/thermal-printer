use core::time::Duration;
use rppal::uart::{Parity, Uart};
use std::{error::Error, io, thread};
use thermal_printer::{Language, ThermalPrinter};

struct Timer;
impl embedded_hal::blocking::delay::DelayUs<u16> for Timer {
    fn delay_us(&mut self, duration: u16) {
        thread::sleep(Duration::from_micros(duration as u64));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(19_200, Parity::None, 8, 1)?;
    uart.set_write_mode(true)?;

    let mut printer = ThermalPrinter::new(uart, Timer)?;
    printer.set_language(Language::Swedish)?;

    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            printer.print(input.as_str()).ok();
        }
    }
}
