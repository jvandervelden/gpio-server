mod zmq_transport;

pub use zmq_transport::ZmqTransport;

pub fn init() -> ZmqTransport {
    let zmq = ZmqTransport::new();
    return zmq;
}
