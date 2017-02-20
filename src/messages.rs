use bincode::{SizeLimit, serialize, deserialize, ErrorKind};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use std::io::Cursor;
use std::io as stdio;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub version: i8,
    pub message: MessageType
}

impl Message {
    pub fn new(message: MessageType) -> Message {
        Message {
            version: 1,
            message: message
        }
    }

    pub fn encode(self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let mut message = serialize(&self, SizeLimit::Infinite)?;
        let mut bytes = Vec::with_capacity(message.len() + 2);

        bytes.write_u16::<NetworkEndian>(message.len() as u16);
        bytes.append(&mut message);

        Ok(bytes)
    }

    pub fn get_len(bytes: Vec<u8>) -> Result<u16, stdio::Error> {
        let mut rdr = Cursor::new(bytes);

        rdr.read_u16::<NetworkEndian>()
    }

    pub fn decode(bytes: Vec<u8>) -> Result<Message, Box<ErrorKind>> {
        deserialize(&bytes)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Connect(Connect)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    pub id: i8,
    pub magic_number: i8
}
