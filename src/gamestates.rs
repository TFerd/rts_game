#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Gameplay,
    Paused,
    Loading,
}