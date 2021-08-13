mod message;

use message::{Message, Reply};
use std::result::Result;
use std::rc::Rc;

use super::{
    pin_manager::{PinManager, PinType},
    transport::ZmqTransport
};

pub struct MessageProcessor {
    pin_manager: PinManager,
    transport: ZmqTransport,
}

impl MessageProcessor {
    pub fn new(pin_manager: PinManager, transport: ZmqTransport) -> Self {
        MessageProcessor {
            pin_manager,
            transport
        }
    }

    fn handle_create(&mut self, message: &Message) -> Result<(bool, &str), &str> {
        if message.pin.is_none() || message.pin_type.is_none() {
            return Err("Must provide pin and pinType for 'create' type.");
        }

        let pin = message.pin.unwrap();
        let pin_type: PinType = match message.pin_type.as_ref().unwrap().as_str() {
            "switch" => PinType::Switch,
            "pwm" => PinType::Pwm,
            _ => return Err("Unknown pinType"),
        };

        self.pin_manager.create_handler(pin, pin_type);

        if message.value.is_some() {
            self.pin_manager.set_handler_value(pin, message.value.unwrap());
        }

        return Ok((false, "Created Successful"));
    }

    fn process_message(&mut self, message: &Message) -> Result<(bool, &str), &str> {
        match message.message_type.as_str() {
            // "clear" => {

            // },
            "create" => {
                self.handle_create(&message)
            },
            // "delete" => {

            // },
            // "deleteAll" => {

            // },
            // "query" => {

            // },
            "quit" => {
                Ok((true, "Quitting..."))
            },
            // "set" => {
            //     // if message.set.is_none() { Err("Must specify create object for create type") } else { self.handle_create(message.create.unwrap()) }
            // },
            // "status" => {

            // }
            _ => return Err("Unknown type")
        }
    }

    fn message_handler(&mut self, message: String) -> (bool, String) {
        println!("Received message {}", message);

        let parse_result: serde_json::Result<Message> = serde_json::from_str(&message);
        let mut reply: Reply = Reply { status: String::new(), message: String::new() };
        let mut exit = false;

        if parse_result.is_err() {
            let error = parse_result.unwrap_err();
            reply.status.push_str("error");
            reply.message.push_str(&format!("Unable to parse message: {}", error));
        } else {
            let message: Message = parse_result.unwrap();

            let process_result = self.process_message(&message);
            
            if process_result.is_err() {
                reply.status.push_str("error");
                reply.message.push_str(&format!("Invalid Message: {}", process_result.unwrap_err()));
            } else {
                reply.status.push_str("ok");
                let (do_exit, processed_message) = process_result.unwrap();
                reply.message.push_str(processed_message);
                exit = do_exit;
            }
        }

        let response: String = serde_json::to_string(&reply).unwrap_or(String::from("{status: \"fatal\",message: \"failed to serialize reply json\"}"));
        return (exit, response);
    }

    pub async fn start(&mut self) {
        println!("Starting processor");
        let me = Box::new(self);
        self.transport.start(|message: String| -> (bool, String) {
            return me.message_handler(message);
        }).await.expect("Failed to start processor");
    }
}