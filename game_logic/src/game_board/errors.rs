use crate::game_board::board_cell::BoardCellPosition;
use crate::Player;
use std::fmt::Display;

#[derive(Debug)]
pub struct FieldOccupiedError {
    position: BoardCellPosition,
    player: Player,
}

impl Display for FieldOccupiedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} already occupied by player{}",
            self.position, self.player
        )
    }
}

impl FieldOccupiedError {
    pub fn new(position: BoardCellPosition, player: Player) -> Self {
        Self { position, player }
    }
}
