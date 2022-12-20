use embedded_hal::{blocking::delay::DelayUs, blocking::serial::Write};
const BYTE_DELAY: u16 = 573;
const SWEDISH: [(char, char); 6] = [
    ('å', '}'),
    ('ä', '{'),
    ('ö', '|'),
    ('Å', ']'),
    ('Ä', '['),
    ('Ö', '\\'),
];

#[derive(Clone, Copy)]
pub enum Language {
    Default = 0x00,
    Swedish = 0x05,
}

impl Language {
    pub fn encode(&self, c: char) -> char {
        match self {
            Language::Default => c,
            Language::Swedish => self.translate(&SWEDISH, c),
        }
    }

    fn translate(&self, table: &[(char, char)], c: char) -> char {
        table
            .iter()
            .find_map(|s| (s.0 == c).then(|| s.1))
            .unwrap_or(c)
    }
}

pub struct ThermalPrinter<E, UART: Write<u8, Error = E>, TIM: DelayUs<u16>> {
    uart: UART,
    timer: TIM,
    lang: Language,
}

impl<E, UART: Write<u8, Error = E>, TIM: DelayUs<u16>> ThermalPrinter<E, UART, TIM> {
    pub fn new(uart: UART, timer: TIM) -> Result<Self, E> {
        let mut printer = Self {
            uart,
            timer,
            lang: Language::Default,
        };
        printer.init()?;
        Ok(printer)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), E> {
        for b in data {
            self.uart.bwrite_all(&[*b])?;
            self.timer.delay_us(BYTE_DELAY);
        }
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), E> {
        self.write(&[0x1B_u8, 0x40])?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), E> {
        self.reset()?;
        self.write(&[0x1B_u8, 0x13, 0x04, 0x08, 0x10, 0x14, 0x18, 0x1C, 0x00])?;
        Ok(())
    }

    pub fn set_language(&mut self, lang: Language) -> Result<(), E> {
        self.write(&[0x1B_u8, 0x52, lang as u8])?;
        self.lang = lang;
        Ok(())
    }

    pub fn print(&mut self, str: &str) -> Result<(), E> {
        let lang = self.lang;
        for c in str.chars().map(|c| lang.encode(c)) {
            self.write(&[c as u8])?;
        }
        Ok(())
    }
}
