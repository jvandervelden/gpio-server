use wiringpi;
use wiringpi::WiringPi;
use wiringpi::pin;

mod handlers;

pub use handlers::Handler;
use handlers::SoftPwmHandler;
use handlers::SwitchHandler;
use async_trait::async_trait;
use async_std::prelude::FuturesUnordered;
use async_std::prelude::Future;
use async_std::prelude::FutureExt;
use async_std::task;

const HARDWARE_PWM_PIN: usize = 18;

pub enum PinType {
    Switch,
    Pwm,
}

pub struct PinManager {
    pi: WiringPi<pin::WiringPi>,
    handlers: Vec<Box<dyn Handler>>,
    runners: FuturesUnordered, // Vec<std::pin::Pin<Box<dyn Future<Output = ()>>>>,
}

#[derive(Clone)]
struct UnInitializedHandler {}

#[async_trait]
impl Handler for UnInitializedHandler {
    fn set_value(&mut self, value: f32) {
        println!("Pin not initialized");
    }

    async fn start(&mut self) {
        println!("Pin not initialized");
    }

    fn stop(&mut self)  {
        println!("Pin not initialized");
    }
}

impl PinManager {  
    pub fn new() -> Self { 
        let mut handlers: Vec<Box<dyn Handler>> = Vec::with_capacity(40);

        for _i in 0..40 {
            handlers.push(Box::new(UnInitializedHandler {}));
        }

        PinManager {
            pi: wiringpi::setup(),
            handlers: handlers,
            runners: Vec::new()
        }
    }

    pub async fn start(&self) {
        loop {
            for runner in &self.runners {
                runner.get_ref().poll();
            }
        }
    }

    pub fn create_handler(&mut self, pin: usize, pin_type: PinType) {
        let handler: Box<dyn Handler>;
        match pin_type {
            PinType::Pwm => {
                if pin == HARDWARE_PWM_PIN { handler = Box::new(SoftPwmHandler::new(pin)) }
                else { handler = Box::new(SoftPwmHandler::new(pin)) }
            }
            PinType::Switch => { handler = Box::new(SwitchHandler::new(pin)) }
        }

        self.runners.push(handler.start());
        self.handlers[pin] = handler;
    }

    pub fn set_handler_value(&mut self, pin: usize, value: f32) {
        self.handlers[pin].set_value(value);
    }
}
