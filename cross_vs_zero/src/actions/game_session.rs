use super::errors::ActionParseError;
use std::str::FromStr;

pub enum GameSessionAction {
    MarkBoardField,
    AdmitDefeat,
    Quit,
}

impl FromStr for GameSessionAction {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Self::MarkBoardField),
            "2" => Ok(Self::AdmitDefeat),
            "3" => Ok(Self::Quit),
            _ => Err(ActionParseError::new(format!(
                "Unknown action with code: {s}"
            ))),
        }
    }
}
