use bytes::{BufMut, BytesMut};
use messages::Message;
use tokio::codec::{Decoder, Encoder};

use super::NetError;

pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = NetError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>, NetError> {
        if buf.len() > 0 {
            let decode_msg = bincode::deserialize(&buf[..])?;
            Ok(Some(decode_msg))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for MessageCodec {
    type Item = Message;
    type Error = NetError;

    fn encode(&mut self, data: Message, buf: &mut BytesMut) -> Result<(), NetError> {
        let bytes = bincode::serialize(&data)?;
        buf.put(&bytes[..]);
        Ok(())
    }
}
