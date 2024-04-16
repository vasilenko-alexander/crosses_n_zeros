use crate::Player;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "-"),
            Cell::Occupied(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardCellPosition {
    pub row: usize,
    pub column: usize,
}

impl FromStr for BoardCellPosition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos: Vec<Result<usize, ParseIntError>> =
            s.trim().split(" ").map(|n| n.parse::<usize>()).collect();

        let row = pos.first().cloned().unwrap()?;
        let column = pos.last().cloned().unwrap()?;

        Ok(BoardCellPosition { row, column })
    }
}

impl Display for BoardCellPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_position() {
        let pos = BoardCellPosition { row: 1, column: 2 };

        assert_eq!(pos.to_string(), "(1, 2)");
    }

    #[test]
    fn parse_position() {
        let user_input = "1 3";
        let expected_pos = BoardCellPosition { row: 1, column: 3 };
        let msg = format!("'{user_input}' should be parsed into cell position without an issue");
        let actual_pos: BoardCellPosition = user_input.parse().expect(&msg);

        assert_eq!(actual_pos, expected_pos);
    }

    #[test]
    fn parse_position_should_fail_for_non_numbers_as_row() {
        let user_input = "I 3";
        let parse_result = user_input.parse::<BoardCellPosition>();

        assert!(parse_result.is_err());
    }

    #[test]
    fn parse_position_should_fail_for_non_numbers_as_column() {
        let user_input = "1 B";
        let parse_result = user_input.parse::<BoardCellPosition>();

        assert!(parse_result.is_err());
    }
}
