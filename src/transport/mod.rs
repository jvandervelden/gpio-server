mod zmq_transport;

use zmq_transport::ZmqTransport;

pub async fn init() {
    let zmq = ZmqTransport {};
    zmq.start().await;
}
