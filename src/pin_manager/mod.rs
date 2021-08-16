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

use super::message_processor::QueryResult;

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

    pub fn assign_handler(&mut self, pin: u16, pin_type: PinType) {
        self.unassign_handler(pin);

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

    pub fn unassign_handler(&mut self, pin: u16) {
        let handler = self.handlers.get_mut(&pin);
        
        if handler.is_none() {
            return;
        }

        handler.unwrap().stop();
        self.handlers.remove(&pin);
    }

    pub fn set_handler_value(&mut self, pin: u16, value: f32) -> Result<(), String> {
        if self.handlers.contains_key(&pin) {
            let handler = self.handlers.get_mut(&pin).unwrap();
            handler.set_value(value);
            Ok(())
        } else {
            let err = format!("Pin {} not initialized", pin);
            println!("{}", err);
            Err(err)
        }
    }

    pub fn get_assigned_pins(&self) -> Vec<u16> {
        let mut pins = vec!();
        for pin in self.handlers.keys() {
            pins.push(*pin);
        }
        return pins;
    }

    pub fn get_query_results(&self) -> Vec<QueryResult> {
        let mut results = vec!();
        for handler in self.handlers.values() {
            results.push(QueryResult {
                pin: handler.get_pin(),
                value: handler.get_value(),
                pin_type: String::from(handler.get_type()),
            });
        }

        return results;
    }
}
