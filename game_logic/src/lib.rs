pub struct GameBoard {
    size: u32,
}

impl GameBoard {
    fn new(size: u32) -> Self {
        Self { size }
    }

    fn size(&self) -> u32 {
        self.size
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
}
