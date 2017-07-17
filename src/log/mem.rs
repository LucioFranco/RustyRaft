use log::{Log, LogIndex};
use raft::Term;
use server::ServerId;

use std::{error, fmt};

#[derive(Debug)]
pub struct Error { }

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        unreachable!()
    }

    fn cause(&self) -> Option<&error::Error> {
        unreachable!()
    }
}

pub struct MemLog {
    current_term: Term,
    voted_for: Option<u8>,
    data: Vec<(Term, Vec<u8>)>,
}

impl Log for MemLog {
    type Error = Error;

    fn current_term(&self) -> Result<Term, Self::Error> {
        Ok(self.current_term)
    }

    fn set_current_term(&mut self, term: Term) -> Result<(), Self::Error>{
        self.current_term = term;
        Ok(())
    }

    fn voted_for(&self) -> Result<Option<ServerId>, Self::Error> {
       Ok(self.voted_for)
    }

    fn set_voted_for(&mut self, candidate: ServerId) -> Result<(), Self::Error> {
        self.voted_for = Some(candidate);
        Ok(())
    }

    fn latest_term(&self) -> Result<Term, Self::Error> {
        let term = self.data[self.data.len() - 1].0;
        Ok(term)
    }

    fn latest_index(&self) -> Result<LogIndex, Self::Error> {
        Ok(self.data.len() as u32 - 1) 
    }

    fn entry(&self, index: LogIndex) -> Result<(Term, Vec<u8>), Self::Error> {
        let data = &self.data[index as usize];
        let term = data.0;
        let entry_data = data.1.clone(); //.into_boxed_slice();

        Ok((term, entry_data))
    }

    fn append_entries(&mut self, from: LogIndex, entries: &[(Term, &[u8])]) -> Result<(), Self::Error> {
        if let Ok(latest_index) = self.latest_index() {
            assert!(latest_index >= from);
            self.data.truncate(from as usize);

            let mut entries: Vec<(u16, Vec<u8>)> = entries.into_iter()
                .map(|e| (e.0, Vec::from(e.1)))
                .collect();

            self.data.append(&mut entries);

            Ok(())
        } else {
            // TODO: actually return an error here
            Ok(())
        }
    }
}
