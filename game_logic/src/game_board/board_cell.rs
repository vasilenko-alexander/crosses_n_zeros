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

#[derive(Clone, Copy, Debug)]
pub struct BoardCellPosition {
    pub row: usize,
    pub column: usize,
}

impl FromStr for BoardCellPosition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let col: Vec<usize> = s
            .trim()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Ok(BoardCellPosition {
            row: *col.first().unwrap(),
            column: *col.last().unwrap(),
        })
    }
}

impl Display for BoardCellPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}
