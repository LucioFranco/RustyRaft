extern crate rustyraft;
extern crate tarpc;
extern crate tokio_core;
extern crate futures;

use rustyraft::{Server, Client};

use std::collections::HashMap;
use tarpc::{server, client};
use tarpc::client::future::{ClientExt};
use tokio_core::reactor::Core;
use tarpc::util::FirstSocketAddr;
use futures::Future;

fn main() {
    // Start tokio core
    let mut core = Core::new().unwrap();

    let peers: HashMap<i8, &str> = [
        (1, "localhost:10000"),
        (2, "localhost:10001")
    ].iter().cloned().collect();

    let s = Server::new(1, &peers);

    let addr = s.start(&core.handle(), server::Options::default());


    let s2 = Server::new(2, &peers);

    s2.start(&core.handle(), server::Options::default());

    let mut client = Client::new(1, "localhost:10000");
    client.connect(&core.handle()).unwrap();
}
