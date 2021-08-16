use std::error::Error;
use std::convert::TryInto;
use zeromq::Socket;
use zeromq::SocketRecv;
use zeromq::SocketSend;

pub struct ZmqTransport {
    socket: zeromq::RepSocket
}

impl ZmqTransport {
    pub fn new() -> Self {
        ZmqTransport {
            socket: zeromq::RepSocket::new(),
        }
    }

    pub async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Binding to localhost:5555...");
        self.socket.bind("tcp://127.0.0.1:5555").await?;
        Ok(())
    }

    pub async fn send(&mut self, return_message: &str) -> Result<(), Box<dyn Error>> {
        self.socket.send(return_message.into()).await?;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<String, Box<dyn Error>> {
        println!("Listening on zmq socket");
        let repl: String = self.socket.recv().await?.try_into()?;
        println!("Received {}", repl);
        return Ok(repl);
    }
}