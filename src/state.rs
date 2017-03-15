use log::LogIndex;
use server::ServerId;

pub enum ConsensusState {
    Leader(LeaderState),
    Follower(FollowerState),
    Candidate(CandidateState)
}

pub struct LeaderState {
    next_index: Vec<(ServerId, LogIndex)>,
    match_index: Vec<(ServerId, LogIndex)>
}

impl LeaderState {
    pub fn new() -> LeaderState {
        LeaderState {
            next_index: vec![],
            match_index: vec![]
        }
    }
}

pub struct FollowerState {
    
}

impl FollowerState {
    pub fn new() -> FollowerState {
         FollowerState {}
    }
}

pub struct CandidateState {

}

impl CandidateState {
    pub fn new() -> CandidateState {
        CandidateState {}
    }
}
