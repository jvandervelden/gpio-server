use wiringpi::{
    WiringPi,
    pin,
    pin::{
        InputPin,
        Value::{High},
        Pull, Pull::{Down,Up,Off}
    },
};
use std::collections::HashMap;

use super::Handler;

pub struct ButtonHandler {
    w_pin: InputPin<pin::WiringPi>,
    pin: u16,
    pull: Pull,
}

impl ButtonHandler {
    pub fn new(pin: u16, pi: &WiringPi<pin::WiringPi>, options: &HashMap<String, String>) -> Self {
        let w_pin = pi.input_pin(pin);
        let pull_opt = options.get("pullUp");
        let pull = if pull_opt.is_some() && pull_opt.unwrap() == "true" { Up } else { Down };

        w_pin.pull_up_dn_control(pull);

        ButtonHandler {
            pin: pin,
            w_pin: w_pin,
            pull: pull,
        }
    }
}

impl Handler for ButtonHandler {
    fn get_pin(&self) -> u16 { self.pin }
    fn get_value(&self) -> f32 { 
        let read_value = self.w_pin.digital_read();
        match self.pull {
            Up => if read_value == High { 0.0 } else { 1.0 }
            Down => if read_value == High { 1.0 } else { 0.0 }
            Off => if read_value == High { 1.0 } else { 0.0 }
        }
    }
    fn get_type(&self) -> &str { "button" }
    fn set_value(&mut self, _value: f32) { println!("Cannot set value for button type"); }
    fn start(&mut self) { println!("Don't need to start button type"); }
    fn stop(&mut self) { println!("Don't need to stop button type"); }
}