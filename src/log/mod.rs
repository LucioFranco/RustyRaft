use raft::Term;

use std::error;

pub type LogIndex = u32;

/// `Log` is the trait that represents the persistent storage
pub trait Log {
    type Error: error::Error;

    /// Returns current term
    fn current_term(&self) -> Result<Term, Self::Error>;

    fn set_current_term(&mut self, term: Term) -> Result<(), Self::Error>;

    /// Appends the provided entries, and returns any errors
    fn append_entries(&mut self, from: LogIndex, entries: &[(Term, &[u8])]) -> Result<(), Self::Error>;
}
