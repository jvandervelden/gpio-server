mod transport;
mod pin_manager;
mod message_processor;

use async_std::{main,test};
use pin_manager::{PinManager,PinType};
use message_processor::MessageProcessor;

#[async_std::main]
async fn main() {
    let transport = transport::init();
    let pin_manager = PinManager::new();
    let message_processor = MessageProcessor::new(pin_manager, transport);

    message_processor.start().await;
}