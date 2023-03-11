use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    // TODO: default is main menu
    MainMenu,
    Gameplay,
    Paused,
    #[default]
    Loading,
}
