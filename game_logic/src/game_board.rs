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
        self.check_rows(player)?;

        self.check_columns(player)?;

        self.check_diagonals(player)?;

        self.check_for_draw()
    }

    fn check_rows(&self, player: Player) -> Option<GameResult> {
        let row_checker = |c: &Cell| -> bool { *c == Cell::Occupied(player) };

        self.cells
            .iter()
            .any(|r| r.iter().all(row_checker))
            .then_some(GameResult::Win(player))
    }

    fn check_columns(&self, player: Player) -> Option<GameResult> {
        let cell_checker = |cell: &Cell| -> bool { *cell == Cell::Occupied(player) };

        let column_checker = |row: &[Cell; BOARD_SIZE], col: usize| -> bool {
            row.get(col).is_some_and(cell_checker)
        };

        let column_indices = 0..BOARD_SIZE;
        column_indices
            .into_iter()
            .any(|col| self.cells.iter().all(|r| column_checker(r, col)))
            .then_some(GameResult::Win(player))
    }

    fn check_diagonals(&self, player: Player) -> Option<GameResult> {
        let cell_checker = |cell: &Cell| -> bool { *cell == Cell::Occupied(player) };

        let mut cell_in_diag_indx: usize = 0;

        // Checking from top-left to bottom-right diagonal
        let result = self
            .cells
            .iter()
            .all(|r| {
                let check_result = r.get(cell_in_diag_indx).is_some_and(cell_checker);
                cell_in_diag_indx += 1;
                check_result
            })
            .then_some(GameResult::Win(player));

        if result.is_some() {
            return result;
        }

        // Checking from top-rigth to bottom-left diagonal
        cell_in_diag_indx = 2;
        self.cells
            .iter()
            .all(|r| {
                let check_result = r.get(cell_in_diag_indx).is_some_and(cell_checker);
                cell_in_diag_indx -= 1;
                check_result
            })
            .then_some(GameResult::Win(player))
    }

    fn check_for_draw(&self) -> Option<GameResult> {
        self.cells
            .iter()
            .all(|r| r.iter().all(|c| *c != Cell::Empty))
            .then_some(GameResult::Draw)
    }
}
