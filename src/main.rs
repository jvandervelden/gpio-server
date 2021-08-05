mod transport;
mod pin_manager;

use std::{
    time,
    thread
};

use pin_manager::PinType;
use pin_manager::PinManager;

fn main_loop() {
    let interval = time::Duration::from_millis(1000);
    let mut pin_manager = PinManager::new();

    pin_manager.create_handler(0, PinType::Pwm);
    pin_manager.set_handler_value(0, 0.5);

    loop {
    }
}

fn main() {
    transport::init();
    main_loop();
}