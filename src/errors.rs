use std::fmt;

#[derive(Debug)]
pub(crate) struct NoPathFoundError();

impl std::error::Error for NoPathFoundError {}

impl fmt::Display for NoPathFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no path found error")
    }
}


