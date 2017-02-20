use futures::sync::mpsc;
use messages::*;

pub struct Connection {
    tx: mpsc::UnboundedSender<MessageType>,
}

impl Connection {
    pub fn new(tx: mpsc::UnboundedSender<MessageType>) -> Self {
        Connection { tx: tx }
    }
}
