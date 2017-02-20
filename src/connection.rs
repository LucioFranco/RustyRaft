use tokio_core::net::{TcpStreamNew, TcpStream};
use futures::Future;
use std::io;

pub struct Connection {
    stream_new: Option<Box<Future<Item=TcpStream, Error=io::Error>>>,
    stream: Option<TcpStream>,
}

impl Connection {
    pub fn new_stream(stream: Box<Future<Item=TcpStream, Error=io::Error>>) -> Connection {
        Connection {
            stream_new: Some(stream),
            stream:  None
        }
    }

    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream_new: None,
            stream: Some(stream)
        }
    }

    fn get_stream(&mut self) -> bool {
        if let None = self.stream {
            if let Some(ref mut tcp_fut) = self.stream_new {
                self.stream = Some(tcp_fut.wait().unwrap());
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn send_message(&mut self) {
        if !self.get_stream() {
            return;
        }

    }
}
