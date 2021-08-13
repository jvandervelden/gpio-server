use std::error::Error;
use std::convert::TryInto;
use zeromq::Socket;
use zeromq::SocketRecv;
use zeromq::SocketSend;

pub struct ZmqTransport {
}

impl ZmqTransport {
    pub async fn start<F>(&self, message_handler: F) -> Result<(), Box<dyn Error>>
        where F: Fn(String) -> (bool, String)
    {
        println!("Starting transport");
        let mut socket = zeromq::RepSocket::new();
        socket.bind("tcp://127.0.0.1:5555").await?;

        loop {
            println!("Listening on zmq socket");
            let repl: String = socket.recv().await?.try_into()?;
            println!("Received {}", repl);
            
            let (exit, return_message) = message_handler(repl);
            socket.send(return_message.into()).await?;

            if exit {
                return Ok(());
            }
        }
    }
}