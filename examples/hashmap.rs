extern crate rustyraft;

use rustyraft::Server;
use std::collections::HashMap;

fn main() {
    let peers: HashMap<i8, &str> = [
        (1, "localhost:10000"),
        (2, "localhost:10001")
    ].iter().cloned().collect();

    Server::new(1, &peers);
    Server::new(2, &peers);
}
