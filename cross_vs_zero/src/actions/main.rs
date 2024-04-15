use super::errors::ActionParseError;
use std::str::FromStr;

pub enum MainAction {
    Start,
    Quit,
}

impl FromStr for MainAction {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Self::Start),
            "2" => Ok(Self::Quit),
            _ => Err(ActionParseError::new(format!(
                "Unknown action with code: {s}"
            ))),
        }
    }
}
