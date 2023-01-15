use bevy::prelude::*;

use crate::{units::tank::SpawnTankEvent, GameState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay).with_system(gameplay_controls),
        );
    }
}

// fn menu_controls

// fn gameplay controls
fn gameplay_controls(keyboard: Res<Input<KeyCode>>, mut ev_spawntank: EventWriter<SpawnTankEvent>) {
    if keyboard.just_pressed(KeyCode::Space) {
        ev_spawntank.send(SpawnTankEvent {
            is_player: true,
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        });
    }
}
