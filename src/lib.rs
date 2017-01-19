#![feature(conservative_impl_trait, plugin)]
#![plugin(tarpc_plugins)]

#[macro_use] extern crate tarpc;
extern crate tokio_core;
extern crate futures;

use futures::Future;
use tarpc::client::future::{Connect, Options};
use tarpc::util::{FirstSocketAddr, Never};
use tokio_core::reactor::{Core};

use std::collections::HashMap;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
pub struct Server;

impl FutureService for Server {
    type HelloFut = futures::Finished<String, Never>;

    fn hello(&self, name: String) -> Self::HelloFut {
        futures::finished(format!("Hello {}!", name))
    }
}

impl Server {
    pub fn new(id: i8, peers: &HashMap<i8, &str>) {
        // Start tokio core
        let mut core = Core::new().unwrap();

        // Get current peer id address
        let address = peers.get(&id).unwrap();

        // Listen for tarpc connections via the addr
        Server.listen_with(address.first_socket_addr(), core.handle()).unwrap();

        for peer in peers.values() {
            if peer != address {
                let options = Options::default().handle(core.handle());

                // Attempt to connect to Peer
                core.run(FutureClient::connect(peer.first_socket_addr(), options)
                         .map_err(tarpc::Error::from)
                         .and_then(|client| client.hello("World from server: #".to_string() + &format!("{}", id)))
                         .map(|resp| println!("{}", resp)))
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Server;

    #[test]
    fn it_works() { }
}

