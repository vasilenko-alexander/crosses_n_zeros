use crate::screens::Screen;

pub enum GameStateAction {
    ChangeScreen(Box<dyn Screen>),
    Quit,
}
