use async_trait::async_trait;

use super::Handler;

pub struct SwitchHandler {
    pin: usize,
    on: bool
}

impl SwitchHandler {
    pub fn new(pin: usize) -> Self { SwitchHandler { pin, on: false } }
}

#[async_trait]
impl Handler for SwitchHandler {
    fn set_value(&mut self, value: f32) {
        if value >= 1.0 {
            self.on = true;
            println!("Turned Pin {} on", self.pin);
        } else {
            self.on = false;
            println!("Turned Pin {} off", self.pin);
        }
    }

    async fn start(&mut self) {
        println!("Switch doesn't need starting");
    }

    fn stop(&mut self) {
        println!("Switch doesn't need stopping");
    }
}
