use std::error;
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

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MaxIter(_) => "maximal iteration",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::MaxIter(_) => None,
        }
    }
}
