use std::fmt;

#[derive(Debug)]
pub enum Error {
    MaxIter(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MaxIter(max) => write!(f, "Maximal iteration ({}) reached", max),
        }
    }
}
