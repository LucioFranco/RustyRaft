extern crate futures;
extern crate rustyraft;
extern crate tokio;

use rustyraft::Server;

use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use futures::Future;

fn main() {
    let mut args = env::args();
    args.next();

    let id = args.next().unwrap();

    let mut peers = HashMap::new();
    peers.insert(
        1,
        "localhost:10000".to_socket_addrs().unwrap().next().unwrap(),
    );
    peers.insert(
        2,
        "localhost:10001".to_socket_addrs().unwrap().next().unwrap(),
    );

    // Server::run(u8::from_str(&id).unwrap(), &peers);
}
