use futures::{Stream, Future};
use futures::future::{loop_fn, Loop};
use futures::{stream, future};
use futures::sync::mpsc;

use tokio_core::net::{TcpListener, TcpStream, TcpStreamNew};
use tokio_core::io::{write_all, read_exact};
use tokio_core::reactor::{Core, Handle, Remote};

use tokio_timer::*;

use std::time::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, BufReader};
use std::iter;
use tokio_core::io::Io;
use std::{thread, time};

use connection::Connection;
use messages::*;

pub type ServerId = u8;
pub type ClientId = u8;

pub struct Server {
    id: ServerId,
    peers: HashMap<ServerId, Connection>,
    clients: HashMap<ClientId, Connection>,
}

impl Server {
    fn new(id: ServerId, peers: &HashMap<u8, SocketAddr>, core: &mut Core) {

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


            // rx.fold((), move |_, msg| {
            //     let server = server.borrow_mut();

            //     println!("Message:{:?}", msg);
            //     println!("{:?}", server.id);
            //     // TODO: execute actions

            //     future::ok(())
            // })
            println!("Starting timer");
            loop_fn((), |_| {
                Timer::default()
                    .sleep(Duration::from_millis(1000))
                    .and_then(|_| {
                        println!("ACTION!");
                        future::ok(Loop::Continue(()))
                    })
                    .map_err(|e| println!("Timer Error: {:?}", e))
            })
        });

        println!("Listening...");
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
            use std;
            std::process::exit(1);
                let size = vec![0, 0];
                let tx = tx.clone();

                let bytes = read_exact(reader, size).and_then(|(reader, len)| {
                    println!("reading");
                    let len = Message::get_len(len).unwrap();
                    let mut arr = Vec::with_capacity((len - 1) as usize);

                    for i in 0..len {
                        arr.push(0);
                    }
                    println!("hello");
                    read_exact(reader, arr)
                });

                bytes.and_then(move |(reader, bytes)| {
                    let msg = Message::decode(bytes).unwrap();
                    println!("Incoming message: {:?}", msg.message);

                    let tx = tx.clone();
                    tx.send(msg.message);
                    println!("sent message");

                    future::ok(reader)
                })
            })
            .map_err(|e| println!("Some Error: {:?}", e))
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

                let (tx, rx) = mpsc::unbounded::<MessageType>();

                let tx2 = tx.clone();
                let h2 = handle.clone();
                let p2 = peer.clone();

                let h2 = handle.clone();


                self.peers.insert(*id, Connection::new(tx)); // TODO: remove function as a method
                let client = future::loop_fn((), move |_| {
                    let message = message.clone();
                    let (tx, rx) = mpsc::unbounded::<MessageType>();

                    // Run the get_connection function and loop again regardless of its result
                    Server::get_connection(&p2, rx, message, &h2)
                        .map(|_| -> Loop<(), ()> { Loop::Continue(()) })
                });

                handle.spawn(client.map_err(|e| println!("Error: {:?}", e)));

                // handle.spawn(writer.map(|_| ()));


            }
        }
    }


    fn get_connection(peer: &SocketAddr,
                      rx: mpsc::UnboundedReceiver<MessageType>,
                      inital_message: Message,
                      handle: &Handle)
                      -> Box<Future<Item = (), Error = io::Error>> {
        println!("Connecting to peer {:?}", &peer);
        let tcp = TcpStream::connect(peer, handle)
            .map_err(|e| println!("Error tcp connect: {:?}", e));

        let connection = tcp.and_then(move |stream| {
                println!("Sending Connect Preamble");

                write_all(stream, inital_message.encode().unwrap())
                    .map_err(|e| println!("StdError: {:?}", e))
            })
            .map_err(|e| println!("Error: {:?}", e));


        let writer = connection.and_then(move |(stream, _)| {
            let (_, writer) = stream.split();

            println!("Entering wait period");
            rx.fold(writer, |writer, msg| {
                    let msg = Message::new(msg).encode().unwrap();

                    let w = write_all(writer, msg).map(|(writer, _)| writer);
                    w.map_err(|_| ())
                })
                .map(|_| ())
        });

        let client = writer.or_else(|_| {
            println!("connection refused");
            thread::sleep(time::Duration::from_millis(5000));
            future::ok(())
            // Err(io::Error::new(io::ErrorKind::Other, "connection refuse"))
        });

        client.boxed()
    }
}
