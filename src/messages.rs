use log::LogIndex;
use raft::Term;
use server::ServerId;
use uuid::Uuid;

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
    pub id: Uuid,
    pub magic_number: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendEntries {
    pub term: Term,
    pub leader_id: ServerId,
    pub prev_log_index: LogIndex,
    pub prev_log_term: Term,
    pub entries: Vec<(Term, Vec<u8>)>,
    pub leader_commit_index: LogIndex,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendEntriesResponse {
    pub term: Term,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVote {
    pub term: Term,
    pub candidate_id: ServerId,
    pub last_log_index: LogIndex,
    pub last_log_term: Term,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestVoteResponse {
    pub term: Term,
    pub vote_granted: bool,
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
}
