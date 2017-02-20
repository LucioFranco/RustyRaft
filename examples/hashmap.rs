extern crate rustyraft;
extern crate tokio_core;
extern crate futures;

use rustyraft::{Server};

use std::collections::HashMap;
use tokio_core::reactor::Core;
use std::net::{ToSocketAddrs, SocketAddr};
use std::env;
use std::str::FromStr;

use futures::Future;

fn main() {
    let mut args = env::args();
    args.next();

    let id = args.next().unwrap();

    // Start tokio core
    let mut core = Core::new().unwrap();

    let mut peers = HashMap::new();
    peers.insert(1, "localhost:10000".to_socket_addrs().unwrap().next().unwrap());
    peers.insert(2, "localhost:10001".to_socket_addrs().unwrap().next().unwrap());

    Server::run(i8::from_str(&id).unwrap(), &peers, &mut core);
}
