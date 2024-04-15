mod board_cell;
mod errors;

use crate::{GameResult, Player};
use std::fmt::Display;

pub use board_cell::{BoardCellPosition, Cell};
pub use errors::FieldOccupiedError;

const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy)]
pub struct GameBoard {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.cells.iter() {
            for c in r.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn size(&self) -> usize {
        BOARD_SIZE
    }

    pub fn occupy_cell(
        &mut self,
        position: BoardCellPosition,
        player: Player,
    ) -> Result<(), FieldOccupiedError> {
        // TODO add proper error handling here for out of range errors
        let cell = self
            .cells
            .get_mut(position.row)
            .expect("cell does not exist")
            .get_mut(position.column)
            .expect("cell does not exist");

        match cell {
            Cell::Occupied(other_player) => {
                return Err(FieldOccupiedError::new(position, other_player.to_owned()));
            }
            Cell::Empty => {
                *cell = Cell::Occupied(player);
            }
        }

        Ok(())
    }

    pub fn check_for_result(&self, player: Player) -> Option<GameResult> {
        self.check_rows(player)

        //self.check_columns(player)?;

        //self.check_diagonals(player)?;

        //self.check_for_draw()?
    }

    fn check_rows(&self, player: Player) -> Option<GameResult> {
        let row_check = |c: &Cell| -> bool {
            if let Cell::Occupied(p) = c {
                *p == player
            } else {
                false
            }
        };

        self.cells
            .iter()
            .any(|r| r.iter().all(row_check))
            .then_some(GameResult::Win(player))
    }
}
