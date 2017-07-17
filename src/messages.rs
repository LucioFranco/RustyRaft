use bincode::{SizeLimit, serialize, deserialize, ErrorKind};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};
use std::io::Cursor;
use std::io as stdio;

use server::{ServerId};
use raft::{Term};
use log::LogIndex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    // Connect Preamble
    Connect(Connect),

    // AppendEntries/Heartbeat message
    AppendEntries(AppendEntries),
    AppendEntriesResponse(AppendEntriesResponse),

    // Request vote messages
    RequestVote(RequestVote),
    RequestVoteResponse(RequestVoteResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connect {
    pub id: u8,
    pub magic_number: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendEntries {
    pub term: Term,
    pub leader_id: ServerId,
    pub prev_log_index: LogIndex,
    pub prev_log_term: Term,
    pub entries: Vec<(Term, Vec<u8>)>,
    pub leader_commit_index: LogIndex
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendEntriesResponse {
    pub term: Term,
    pub success: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVote {
    pub term: Term,
    pub candidate_id: ServerId,
    pub last_log_index: LogIndex,
    pub last_log_term: Term
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVoteResponse {
    pub term: Term,
    pub vote_granted: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub version: i8,
    pub message: MessageType,
}

impl Message {
    pub fn new(message: MessageType) -> Message {
        Message {
            version: 1,
            message: message,
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
