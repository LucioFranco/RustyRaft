use futures::{Future};
use futures;
use tarpc::{server};
use tarpc::util::{Never, FirstSocketAddr};
use tokio_core::reactor::Handle;

use std::collections::HashMap;

service! {
    rpc establish(id: i8) -> i8;
}

#[derive(Clone)]
pub struct Server {
    id: i8,
    address: String,
    peers: HashMap<i8, String>,
}

impl FutureService for Server {
    type EstablishFut = futures::Finished<i8, Never>;

    fn establish(&self, id: i8) -> Self::EstablishFut {
        futures::finished(self.id)
    }
}

impl Server {
    pub fn new(id: i8, peers: &HashMap<i8, &str>) -> Server {
        // Gejt current peer id address
        let address = peers.get(&id).unwrap();

        Server {
            id: id,
            address: address.to_string(),
            peers: peers.iter().map(|(id, addr)| (*id, addr.to_string())).collect(),
        }
    }

    pub fn start(self, handle: &Handle, options: server::Options) -> Self {
        let addr = self.address.clone();
        let a = self.clone();
        self.listen(addr.as_str().first_socket_addr(), handle, options);

        a
    }
}
