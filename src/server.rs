use futures::{Stream, Future};
use futures::{stream, future};
use futures::sync::mpsc;

use tokio_core::net::{TcpListener, TcpStream, TcpStreamNew};
use tokio_core::io::{write_all, read_exact};
use tokio_core::reactor::{Core, Handle, Remote};

use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, BufReader};
use std::iter;
use tokio_core::io::Io;

use connection::Connection;
use messages::*;

type ServerId = i8;
type ClientId = i8;

pub struct Server {
    id: ServerId,
    peers: HashMap<ServerId, Connection>,
    clients: HashMap<ClientId, Connection>,
}

impl Server {
    fn new(id: ServerId, peers: &HashMap<i8, SocketAddr>, core: &mut Core) {

        let addr = peers.get(&id).unwrap();
        let handle = &core.handle();
        let listener = TcpListener::bind(addr, handle).unwrap();


        let mut server = Rc::new(RefCell::new(Server {
            id: id,
            peers: HashMap::new(),
            clients: HashMap::new(),
        }));

        server.borrow_mut().connect_to_peers(peers, handle);


        // Create incoming message queue
        let (tx, rx) = mpsc::unbounded::<MessageType>();


        let h2 = handle.clone();
        let s2 = server.clone();

        let s = listener.incoming()
            .for_each(move |(sock, _)| {
                println!("New connection");

                let server = s2.clone();
                let tx = tx.clone();
                h2.spawn(server.borrow_mut().new_client(sock, tx));

                Ok(())
            })
            .map_err(|e| panic!("Error: {:?}", e));

        let server = server.clone();
        handle.spawn_fn(move || {
            // TODO: create raft


            rx.fold((), move |_, msg| {
                let server = server.borrow_mut();

                println!("Message:{:?}", msg);
                println!("{:?}", server.id);
                // TODO: execute actions

                future::ok(())
            })
        });

        core.run(s).unwrap();
    }

    fn new_client(&mut self,
                  socket: TcpStream,
                  tx: mpsc::UnboundedSender<MessageType>)
                  -> Box<Future<Item = (), Error = ()>> {
        let (reader, _) = socket.split();
        let reader = BufReader::new(reader);

        let iter = stream::iter(iter::repeat(()).map(Ok::<(), io::Error>));
        let socket_reader = iter.fold(reader, move |reader, _| {
                let size = vec![0, 0];
                let tx = tx.clone();

                let bytes = read_exact(reader, size).and_then(|(reader, len)| {
                    let len = Message::get_len(len).unwrap();
                    let mut bytes = Vec::with_capacity(len as usize);

                    for i in 0..len {
                        bytes.push(0);
                    }

                    read_exact(reader, bytes)
                });

                bytes.and_then(move |(reader, bytes)| {
                    let msg = Message::decode(bytes).unwrap();
                    println!("{:?}", msg);

                    let tx = tx.clone();
                    tx.send(msg.message);

                    future::ok(reader)
                })
            })
            .map_err(|_| ())
            .map(|_| ());

        socket_reader.boxed()
    }

    pub fn run(id: ServerId, peers: &HashMap<ServerId, SocketAddr>, core: &mut Core) {
        Server::new(id, peers, core);
    }

    fn connect_to_peers(&mut self, peers: &HashMap<ServerId, SocketAddr>, handle: &Handle) {
        for (id, peer) in peers.iter() {
            if *id != self.id {
                let connect_msg = Connect {
                    id: self.id,
                    magic_number: 1818,
                };
                let message = Message::new(MessageType::Connect(connect_msg));
                let remote = handle.remote();

                let (tx, rx) = mpsc::unbounded::<MessageType>();

                let tx2 = tx.clone();

                let connection = TcpStream::connect(peer, handle)
                    .map_err(|_| ())
                    // .or_else(move |e| -> TcpStreamNew {
                    //     //let handle = remote.handle().unwrap();
                    //     TcpStream::connect(peer, &remote.handle())
                    // })
                    .and_then(move |stream| {
                        write_all(stream, message.encode().unwrap())
                            .map_err(|_| ())
                    });


                let writer = connection.and_then(move |(stream, _)| {
                    let (_, writer) = stream.split();

                    rx.fold(writer, |writer, msg| {
                        let msg = Message::new(msg).encode().unwrap();

                        let w = write_all(writer, msg).map(|(writer, _)| writer);
                        w.map_err(|_| ())
                    })
                });

                handle.spawn(writer.map(|_| ()));

                self.peers.insert(*id, Connection::new(tx2));
            }
        }
    }
}
