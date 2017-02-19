use server::FutureClient;
use futures::Future;
use futures;
use tarpc::{client};
use tarpc;
use tarpc::util::{FirstSocketAddr, Never};
use tokio_core::reactor::{Handle};
use tarpc::client::future::ClientExt;

use std::collections::HashMap;

pub struct Client {
    peer_id: i8,
    peer_address: String,
    pub client: Option<FutureClient>
}

impl Client {
    pub fn new(id: i8, peer_address: &str) -> Client {
        Client {
            peer_id: id,
            peer_address: peer_address.into(),
            client: None
        }
    }

    pub fn connect(&mut self, handle: &Handle) -> Result<i8, tarpc::Error<tarpc::util::Never>> {
        let addr = self.peer_address.clone();
        let opt = client::Options::default().handle(handle.clone());

        FutureClient::connect(addr.as_str().first_socket_addr(), opt)
            .map_err(|err| match err {
                _ => tarpc::Error::from(err),
            })
            .and_then(|c| {
                self.client = Some(c);
                let a = self.client.clone().unwrap();
                a.establish(self.peer_id)
            })
            .wait()
    }

    pub fn connect_to_peers(address: &str, peers: HashMap<i8, &str>, handle: Handle) {
        for (key, peer) in peers.iter() {
            if *peer != address {
                let options = client::Options::default().handle(handle.clone());

                let client = Client::new(*key, peer);
            }
        }
    }
}
