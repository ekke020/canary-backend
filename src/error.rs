use std::fmt;

pub trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    // fn cause(&self) -> Option<&Error> { /* ... */ }
    // fn source(&self) -> Option<&(Error + 'static)> { /* ... */ }
}

#[derive(Debug)]
pub struct PathError {
    _message: String,
}

impl PathError {
    pub fn new(message: &str) -> Self {
        Self {
            _message: message.to_string(),
        }
    }
}

impl Error for PathError {
    fn description(&self) -> &str {
        todo!()
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}