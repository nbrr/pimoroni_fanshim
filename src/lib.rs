use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, Level, Level::*, OutputPin};

const CLK: u8 = 14;
const DAT: u8 = 15;
const BTN: u8 = 17;
const FAN: u8 = 18;

struct Fanshim {
    clk: OutputPin,
    dat: OutputPin,
    btn: InputPin,
    fan: OutputPin,
}

impl Fanshim {
    pub fn fan_on(&mut self) {
        self.fan.set_high();
    }

    pub fn fan_off(&mut self) {
        self.fan.set_low();
    }

    fn write_byte(&mut self, byte: u8) {
        let seq: Vec<Level> = (0..8u8)
            .map(move |bit| if ((byte >> bit) & 1) == 1 { High } else { Low })
            .collect();

        for level in seq {
            self.dat.write(level);
            self.clk.set_high();
            thread::sleep(Duration::from_nanos(500));
            self.clk.set_low();
            thread::sleep(Duration::from_nanos(500));
        }
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) {
        self.sof();
        self.write_byte(224 + 31);
        self.write_byte(b);
        self.write_byte(g);
        self.write_byte(r);
        self.eof();
    }

    fn sof(&mut self) {
        for _ in 0..4 {
            self.write_byte(000);
        }
    }

    fn eof(&mut self) {
        for _ in 0..4 {
            self.write_byte(255);
        }
    }

    fn led_off(&mut self) {
        self.sof();
        self.color(0, 0, 0);
        self.eof();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_led_rgb() {
        let fs: Fanshim = Default::default();

        fs.led_off();
        thread::sleep(Duration::from_millis(1000));
        fs.color(255, 0, 0);
        thread::sleep(Duration::from_millis(1000));
        fs.led_off();
        thread::sleep(Duration::from_millis(1000));
        fs.color(0, 255, 0);
        thread::sleep(Duration::from_millis(1000));
        fs.led_off();
        thread::sleep(Duration::from_millis(1000));
        fs.color(0, 0, 255);
        thread::sleep(Duration::from_millis(1000));
        fs.led_off();
    }
}
