use std::{
    fmt::Display,
    io::Error,
    num::{IntErrorKind, ParseIntError},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub struct GameBoard {
    cells: [[Cell; 3]; 3],
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

pub enum GameResult {
    Win(Player),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    PlayerX(char),
    PlayerO(char),
}

impl Player {
    pub fn create_player_x() -> Self {
        Self::PlayerX('X')
    }

    pub fn create_player_o() -> Self {
        Self::PlayerO('O')
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_mark = match self {
            Player::PlayerX(x) => x,
            Player::PlayerO(o) => o,
        };
        write!(f, "{}", player_mark)
    }
}

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

impl GameBoard {
    pub fn new(size: usize) -> Self {
        Self {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    pub fn size(&self) -> usize {
        3
    }

    pub fn occupy_cell(
        &mut self,
        position: BoardCellPosition,
        player: Player,
    ) -> Result<(), FieldOccupiedError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = GameBoard::new(3);
        assert_eq!(board.size(), 3);
    }

    #[test]
    fn print_board() {
        let board = GameBoard::new(3);
        println!("{board}");
    }
}
