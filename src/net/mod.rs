mod codec;
mod node;

pub use self::node::Node;

#[derive(Debug)]
pub enum NetError {
    Bincode(bincode::Error),
    Io(std::io::Error),
}

impl From<bincode::Error> for NetError {
    fn from(err: bincode::Error) -> Self {
        NetError::Bincode(err)
    }
}

impl From<std::io::Error> for NetError {
    fn from(err: std::io::Error) -> Self {
        NetError::Io(err)
    }
}
