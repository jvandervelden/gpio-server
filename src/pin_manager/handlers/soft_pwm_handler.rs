use wiringpi::{
    WiringPi,
    pin,
    pin::{
        SoftPwmPin,
    },
};

use super::Handler;

pub struct SoftPwmHandler {
    w_pin: SoftPwmPin<pin::WiringPi>,
    pin: u16,
    value: f32,
}

impl SoftPwmHandler {
    pub fn new(pin: u16, pi: &WiringPi<pin::WiringPi>) -> Self { 
        let w_pin = pi.output_pin(pin).into_soft_pwm();

        w_pin.pwm_write(0);

        SoftPwmHandler {
            w_pin,
            pin,
            value: 0.0,
        }
    }
}

impl Handler for SoftPwmHandler {
    fn get_pin(&self) -> u16 { self.pin }
    fn get_value(&self) -> f32 { self.value }
    fn get_type(&self) -> &str { "pwm" }

    fn set_value(&mut self, value: f32) {
        self.value = value;
        self.w_pin.pwm_write((self.value * 100.0) as i32);
    }

    fn start(&mut self) {
        println!("Soft pwm already started on {}", self.pin);
    }

    fn stop(&mut self) {
        self.value = 0.0;
        self.w_pin.pwm_write((self.value * 100.0) as i32);
        println!("Stopping pwm handler on {}.", self.pin);
    }
}
