use std::fmt::Display;

pub struct ActionParseError {
    msg: String,
}

impl Display for ActionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ActionParseError {
    pub fn new(error_msg: String) -> Self {
        ActionParseError { msg: error_msg }
    }
}
