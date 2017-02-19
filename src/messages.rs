use futures::{Future, Finished};
use tarpc::{server};
use tarpc::util::{Never};

service! {
    rpc establish(id: i8) -> i8;
}

#[derive(Clone)]
pub struct Message;

impl FutureService for Message {
    type EstablishFut = Finished<i8, Never>;

    fn establish(&self, id: i8) -> Self::EstablishFut {
        
    }
}
