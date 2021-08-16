use wiringpi::{
    WiringPi,
    pin,
    pin::{
        OutputPin,
        Value::{High, Low},
    }
};

use super::Handler;


pub struct SwitchHandler {
    w_pin: OutputPin<pin::WiringPi>,
    pin: u16,
    on: bool
}

impl SwitchHandler {
    pub fn new(pin: u16, pi: &WiringPi<pin::WiringPi>) -> Self { SwitchHandler {
        pin,
        w_pin: pi.output_pin(pin),
        on: false
    } }
}

impl Handler for SwitchHandler {
    fn get_pin(&self) -> u16 { self.pin }
    fn get_value(&self) -> f32 { if self.on { 1.0 } else { 0.0 } }
    fn get_type(&self) -> &str { "switch" }

    fn set_value(&mut self, value: f32) {
        if value >= 1.0 {
            self.on = true;
            self.w_pin.digital_write(High);
            println!("Turned Pin {} on", self.pin);
        } else {
            self.on = false;
            self.w_pin.digital_write(Low);
            println!("Turned Pin {} off", self.pin);
        }
    }

    fn start(&mut self) {
        println!("Switch doesn't need starting");
    }

    fn stop(&mut self) {
        self.on = false;
        self.w_pin.digital_write(Low);
    }
}
