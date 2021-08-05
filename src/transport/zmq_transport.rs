use std::{thread, time};

use async_std::task;

pub struct ZmqTransport {
}

impl ZmqTransport {
    pub async fn start(self) {
        let interval = time::Duration::from_millis(1000);

        loop {
            task::sleep(interval).await;
            println!("Zmq Loop");
        }
    }
}