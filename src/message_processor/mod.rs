mod message;

pub use message::{Message, MessageResult, QueryResult, StatusResult};
use std::result::Result;
use std::collections::HashMap;

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

    fn handle_assign(&mut self, message: &Message) -> Result<(bool, &str), String> {
        if message.pin.is_none() || message.pin_type.is_none() {
            return Err(String::from("Must provide pin and pinType for 'create' type."));
        }

        let pin = message.pin.unwrap();
        let pin_type: PinType = match message.pin_type.as_ref().unwrap().as_str() {
            "button" => PinType::Button,
            "switch" => PinType::Switch,
            "pwm" => PinType::Pwm,
            _ => return Err(String::from("Unknown pinType")),
        };
        let mut options: HashMap<String, String> = HashMap::new();
        if message.options.is_some() {
            for option in message.options.as_ref().unwrap() {
                options.insert(String::from(option.0), option.1.to_string());
            }
        }

        self.pin_manager.assign_handler(pin, pin_type, &options);

        if message.value.is_some() {
            let res = self.pin_manager.set_handler_value(pin, message.value.unwrap());
            if res.is_err() {
                return Err(format!("Pin assigned but value not set: {}", res.unwrap_err()));
            }
        }

        return Ok((false, "Created Successful"));
    }

    fn handle_set(&mut self, message: &Message) -> Result<(bool, &str), String> {
        if message.pin.is_none() || message.value.is_none() {
            return Err(String::from("Must provide pin and value for 'set' type."));
        }

        let pin = message.pin.unwrap();
        let value = message.value.unwrap();

        let set_res = self.pin_manager.set_handler_value(pin, value);
        if set_res.is_err() {
            return Err(format!("Unable to set handler value: {}", set_res.unwrap_err()));
        }

        Ok((false, "Value set successfully"))
    }

    fn handle_unassign(&mut self, message: &Message) -> Result<(bool, &str), String> {
        if message.pin.is_none() {
            return Err(String::from("Must provide pin for 'unassign' type."));
        }

        self.pin_manager.unassign_handler(message.pin.unwrap());

        Ok((false, "Unassigned Successfully"))
    }

    fn handle_unassign_all(&mut self) -> Result<(bool, &str), String> {
        for pin in self.pin_manager.get_assigned_pins().iter() {
            self.pin_manager.unassign_handler(*pin);
        }

        Ok((false, "Unassigned Successfully"))
    }

    fn handle_status(&mut self) -> Result<(bool, &str, StatusResult), String> {
        let pin_count = self.pin_manager.get_assigned_pins().len();

        Ok((false, "Daemon Status", StatusResult {
            pin_count: pin_count,
        }))
    }

    fn handle_clear(&mut self, pin: u16) -> Result<(bool, &str), String> {
        let res = self.pin_manager.set_handler_value(pin, 0.0);
        if res.is_err() {
            return Err(format!("Unable to clear pin to 0: {}", res.unwrap_err()));
        }        

        Ok((false, "Set Pin Value To 0"))
    }

    fn handle_clear_all(&mut self) -> Result<(bool, &str), String> {
        let mut any_failed: bool = false;
        let mut errors: String = String::new();
        for pin in self.pin_manager.get_assigned_pins() {
            let res = self.pin_manager.set_handler_value(pin, 0.0);
            if res.is_err() {
                any_failed = true;
                errors.push_str(&format!("| Error setting pin {} to 0: {} |", pin, res.unwrap_err()));
            }
        }

        if any_failed {
            return Err(errors);
        }
        
        Ok((false, "Set all pin values To 0"))
    }

    fn handle_query(&self) -> Result<(bool, &str, Vec<QueryResult>), String> {
        Ok((false, "Assigned Pins", self.pin_manager.get_query_results()))
    }

    fn process_simple_message(&mut self, message: &Message) -> Result<(bool, &str, Option<StatusResult>, Option<Vec<QueryResult>>), String> {
        let res = match message.message_type.as_str() {
            "assign" => {
                self.handle_assign(&message)
            },
            "clear" => {
                if message.pin.is_some() { self.handle_clear(message.pin.unwrap()) } else { self.handle_clear_all() }
            },
            "quit" => {
                Ok((true, "Quitting..."))
            },
            "set" => {
                self.handle_set(&message)
            },
            "unassign" => {
                self.handle_unassign(&message)
            },
            "unassignAll" => {
                self.handle_unassign_all()
            },
            _ => return Err(String::from("Unknown type"))
        };
        if res.is_err() { return Err(res.unwrap_err()); }
        let (exit, message) = res.unwrap();        

        Ok((exit, message, None, None))
    }

    fn process_status_message(&mut self) -> Result<(bool, &str, Option<StatusResult>, Option<Vec<QueryResult>>), String> {
        let res = self.handle_status();
        if res.is_err() { return Err(res.unwrap_err()); }
        let (exit, message, status) = res.unwrap();        

        Ok((exit, message, Some(status), None))
    }

    fn process_query_message(&mut self) -> Result<(bool, &str, Option<StatusResult>, Option<Vec<QueryResult>>), String> {
        let res = self.handle_query();
        if res.is_err() { return Err(res.unwrap_err()); }
        let (exit, message, query_results) = res.unwrap();        

        Ok((exit, message, None, Some(query_results)))
    }

    fn process_message(&mut self, message: &Message) -> Result<(bool, &str, Option<StatusResult>, Option<Vec<QueryResult>>), String> {
        match message.message_type.as_str() {
            "assign" | "clear" | "quit" | "set" | "unassign" | "unassignAll" => {
                self.process_simple_message(message)
            },
            "query" => {
                self.process_query_message()
            },
            "status" => {
                self.process_status_message()
            }
            _ => return Err(String::from("Unknown type"))
        }
    }

    fn message_handler(&mut self, message: String) -> (bool, String) {
        println!("Received message {}", message);

        let parse_result: serde_json::Result<Message> = serde_json::from_str(&message);
        let mut reply: MessageResult = MessageResult { status: String::new(), message: String::new(), daemon_status: None, query_result: None };
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
                reply.message.push_str(&process_result.unwrap_err());
            } else {
                reply.status.push_str("ok");
                let (do_exit, processed_message, daemon_status, query_result) = process_result.unwrap();
                reply.message.push_str(processed_message);
                reply.daemon_status = daemon_status;
                reply.query_result = query_result;
                exit = do_exit;
            }
        }

        let response: String = serde_json::to_string(&reply).unwrap_or(String::from("{status: \"fatal\",message: \"failed to serialize reply json\"}"));
        return (exit, response);
    }

    pub async fn start(&mut self) {
        println!("Starting processor");
        self.transport.listen().await.expect("Failed for transport to listen");
        
        loop {
            let message = self.transport.next().await.expect("Failed to start processor");
            let (exit, reply) = self.message_handler(message);
            self.transport.send(&reply).await.expect("Failed to send reply");

            if exit {
                break;
            } 
        }
    }
}