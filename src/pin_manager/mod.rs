use wiringpi::{
    WiringPi,
    pin,
};
use std::collections::HashMap;

mod handlers;

pub use handlers::Handler;
use handlers::PwmHandler;
use handlers::SoftPwmHandler;
use handlers::SwitchHandler;

const HARDWARE_PWM_PIN: u16 = 18;

pub enum PinType {
    Switch,
    Pwm,
}

pub struct PinManager {
    pi: WiringPi<pin::WiringPi>,
    handlers: HashMap<u16, Box<dyn Handler>>,
}

impl PinManager {  
    pub fn new() -> Self {
        PinManager {
            pi: wiringpi::setup(),
            handlers: HashMap::new(),
        }
    }

    pub fn create_handler(&mut self, pin: u16, pin_type: PinType) {
        let mut handler: Box<dyn Handler>;
        match pin_type {
            PinType::Pwm => {
                if pin == HARDWARE_PWM_PIN { handler = Box::new(PwmHandler::new(pin, &self.pi)) }
                else { handler = Box::new(SoftPwmHandler::new(pin, &self.pi)) }
            }
            PinType::Switch => { handler = Box::new(SwitchHandler::new(pin, &self.pi)) }
        }

        handler.start();
        self.handlers.insert(pin, handler);
    }

    pub fn set_handler_value(&mut self, pin: u16, value: f32) {
        if self.handlers.contains_key(&pin) {
            let handler = self.handlers.get_mut(&pin).unwrap();
            handler.set_value(value);
        } else {
            println!("Pin {} not initialized", pin);
        }
    }
}
