use async_std::task;
use async_std::task::JoinHandle;
use async_trait::async_trait;
use std::{time};

use wiringpi::pin::Pin;

use super::Handler;

pub struct SoftPwmHandler {
    pin: usize,
    value: f32,
    running: bool,
    pwmThread: JoinHandle<()>
}

impl SoftPwmHandler {
    pub fn new(pin: usize) -> Self { 
        SoftPwmHandler { 
            pin,
            value: 0.0,
            running: false,
            pwmThread: task::spawn(async {})
        }
    }
}

#[async_trait]
impl Handler for SoftPwmHandler {
    fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    async fn start(&mut self) {
        self.running = true;
        println!("Starting pwm handler on {}.", self.pin);
        let interval = time::Duration::from_millis(1000);
        while self.running {
            task::sleep(interval).await;
            println!("Pwm Loop");
        }
    }

    fn stop(&mut self) {
        self.running = false;
        println!("Stopping pwm handler on {}.", self.pin);
    }
}
