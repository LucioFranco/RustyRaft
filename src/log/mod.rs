use raft::Term;

use std::error;

pub type LogIndex = u32;

/// `Log` is the trait that represents the persistent storage
pub trait Log {
    type Error: error::Error;

    /// Returns current term
    fn current_term(&self) -> Result<Term, Self::Error>;

    /// Update current term
    fn set_current_term(&mut self, term: Term) -> Result<(), Self::Error>;

    /// Get last candidate that state machine voted for
    fn voted_for(&self) -> Result<Term, Self::Error>;

    /// Update last candidate that statemachine voted for
    fn set_voted_for(&mut self, candidate: Term) -> Result<(), Self::Error>;

    /// Get latest log term
    fn latest_term(&self) -> Result<Term, Self::Error>;

    /// Get latest log index
    fn latest_index(&self) -> Result<LogIndex, Self::Error>;

    /// Get entry at index
    fn entry(&self, index: LogIndex) -> Result<(Term, &[u8]), Self::Error>;

    /// Appends the provided entries, and returns any errors
    fn append_entries(&mut self, from: LogIndex, entries: &[(Term, &[u8])]) -> Result<(), Self::Error>;
}
