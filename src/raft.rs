use connection::Connection;
use server::{ServerId, ClientId};
use state::{self, ConsensusState, LeaderState, CandidateState, FollowerState};
use messages::{self, Message, MessageType};
use log::LogIndex;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::cmp::min;

pub type Term = u16;

/// `Actions` handles async actions the server must execute
pub struct Actions {
    /// Messages for peers
    pub peer_messages: Vec<(ServerId, Message)>,

    // Messages for clients
    pub client_messages: Vec<(ClientId, Message)>, // TODO: add timeout
}

pub struct Raft {
    id: ServerId,
    peers: HashMap<ServerId, SocketAddr>,
    state: ConsensusState,
    current_term: Term,
    commit_index: LogIndex,
    last_applied: LogIndex,
}

impl Raft {
    pub fn new(id: ServerId, peers: HashMap<ServerId, SocketAddr>) -> Raft {
        Raft {
            id: id,
            peers: peers,
            state: ConsensusState::Candidate(CandidateState::new()),
            current_term: 0,
            commit_index: 0,
            last_applied: 0,
        }
    }

    pub fn init(&self) -> Actions {
        Actions {
            peer_messages: vec![],
            client_messages: vec![],
        }
    }

    pub fn apply_peer_message(&mut self,
                              from: ServerId,
                              message: MessageType,
                              actions: &mut Actions) {
        match message {
            MessageType::AppendEntries(msg) => self.append_entries(from, msg, actions),

            _ => panic!("Wrong message received"),
        };
    }

    fn append_entries(&mut self,
                      from: ServerId,
                      message: messages::AppendEntries,
                      actions: &mut Actions) {
        let leader_term = message.term;
        let current_term = self.current_term;

        match self.state {
            ConsensusState::Follower(ref mut state) => {
                // In the follower state the sender is the leader or should be ;)
                let leader = from;

                if leader_term < current_term {
                    let false_message = Message::new(MessageType::AppendEntriesResponse(
                        messages::AppendEntriesResponse {
                            term: current_term,
                            success: false
                        }));
                    actions.peer_messages.push((leader, false_message));
                    return;
                }

                // TODO: Reply false if log doesn’t contain an entry at prevLogIndex
                //       whose term matches prevLogTerm

                // TODO: If an existing entry conflicts with a new one (same index
                //       but different terms), delete the existing entry and all that
                //       follow it

                // TODO: Append any new entries not already in the log

                // Update commit index based on last log if we got lost
                if message.leader_commit_index > self.commit_index {
                    // TODO: make this min of (leaderCommit, index of last new entry)
                    self.commit_index = min(message.leader_commit_index, 0);
                }

                let success_message = Message::new(MessageType::AppendEntriesResponse(
                    messages::AppendEntriesResponse {
                        term: current_term,
                        success: true
                    }));
                actions.peer_messages.push((leader, success_message))
            }

            // TODO: figure out if the other states need to handle anything
            // with AppenedEntries

            _ => panic!("State not implemented"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use std::net::ToSocketAddrs;

    fn gen_peers() -> HashMap<ServerId, SocketAddr> {
        let mut peers = HashMap::new();
        peers.insert(1,
                     "localhost:10000".to_socket_addrs().unwrap().next().unwrap());
        peers.insert(2,
                     "localhost:10001".to_socket_addrs().unwrap().next().unwrap());

        peers
    }

    #[test]
    fn init() {
        let raft = Raft::new(1, gen_peers());

        let actions = raft.init();

        assert_eq!(actions.peer_messages.len(), 0);
        assert_eq!(actions.client_messages.len(), 0);
    }

    fn request_vote() {}

    #[test]
    fn apply_peer_message() {
        let mut raft = Raft::new(1, gen_peers());

        let mut actions = raft.init();

        let message = Message::new(MessageType::AppendEntries(messages::AppendEntries {
            term: 0,
            leader_id: 1,
            prev_log_index: 0,
            prev_log_term: 0,
            entries: vec![],
            leader_commit_index: 0,
        }));

        raft.apply_peer_message(2, message.message, &mut actions);

    }
}
