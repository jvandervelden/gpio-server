use wiringpi::{
    WiringPi,
    pin,
    pin::{
        PwmPin,
    },
};

use super::Handler;

pub struct PwmHandler {
    w_pin: PwmPin<pin::WiringPi>,
    pin: u16,
    value: f32,
}

impl PwmHandler {
    pub fn new(pin: u16, pi: &WiringPi<pin::WiringPi>) -> Self { 
        let w_pin = pi.output_pin(pin).into_pwm();

        w_pin.write(0);

        PwmHandler {
            w_pin,
            pin,
            value: 0.0,
        }
    }
}

impl Handler for PwmHandler {
    fn set_value(&mut self, value: f32) {
        self.value = value;
        self.w_pin.write((self.value * 1024.0) as u16);
    }

    fn start(&mut self) {
        println!("Pwm already started on {}", self.pin);
    }

    fn stop(&mut self) {
        self.value = 0.0;
        self.w_pin.write(0);
        println!("Stopping pwm handler on {}.", self.pin);
    }
}
