use futures::future::{Future, loop_fn, Loop};
use futures::IntoFuture;
use futures::sync::mpsc;
use tokio_core::net::{TcpStream, TcpStreamNew};
use tokio_core::reactor::Handle;
use std::net::SocketAddr;
use std::io;



use messages::*;

pub struct Connection {
    tx: mpsc::UnboundedSender<MessageType>,
}

impl Connection {
    pub fn new(tx: mpsc::UnboundedSender<MessageType>) -> Self {
        Connection { tx: tx }
    }

    pub fn connect(peer: &SocketAddr,
                   handle: &Handle) {
        let p = peer.clone();
        let h = handle.clone();

        // fn reconnect(err: io::Error) -> TcpStreamNew {
        //     match err {
        //         _ => {
        //             let p2 = p.clone();
        //             let h2 = handle.clone();

        //             Connection::connect(&p2, &h2)
        //         }
        //     }
        // }

        // TcpStream::connect(peer, handle).or_else(reconnect);

        // TODO: figure out how to make this return TcpStreamNew


        loop_fn(TcpStream::connect(peer, handle), |_| {
            TcpStream::connect(peer, handle)
                .map(|stream| -> Loop<TcpStream, TcpStreamNew> {
                    Loop::Break(stream)
                })
        });
    }
}
