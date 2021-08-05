mod transport;
mod pin_manager;

use async_std::prelude::FutureExt;
use wiringpi::pin::Value::{High, Low};
use wiringpi::WiringPi;
use wiringpi::pin;
use std::{time};
use async_std::task;

use pin_manager::PinType;
use pin_manager::PinManager;

async fn main_loop() {
    let interval = time::Duration::from_millis(1000);
    let mut pin_manager = PinManager::new();

    pin_manager.create_handler(4, PinType::Switch);

    loop {
        pin_manager.set_handler_value(4, 1.0);
        task::sleep(interval).await;

        pin_manager.set_handler_value(4, 0.0);
        task::sleep(interval).await;

        println!("Main Loop");
    }
}

#[async_std::main]
async fn main() {
    //let pi = wiringpi::setup();

    //Use WiringPi pin 0 as output
    //let pin = pi.output_pin(0);

    let _fut = transport::init();
    let fut2 = main_loop();

    _fut.join(fut2).await;

    //Set pin 0 to high and wait one second
    //pin.digital_write(High);
    //Set pin 0 to low and wait one second
    //pin.digital_write(Low);
}