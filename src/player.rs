use bevy::prelude::*;

use crate::GameState;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub money: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(init_player));
    }
}

fn init_player(mut commands: Commands) {
    commands.spawn((Player { money: 500 }, Name::new("Player")));
}
