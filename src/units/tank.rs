use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::{utils::Health, GameState};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tank;

#[derive(Bundle)]
pub struct TankBundle {
    pub unit_type: Tank,
    pub health: Health,
    pub pickable: PickableBundle,
}

// @TODO: make a general spawn unit event?
pub struct SpawnTankEvent {
    pub is_player: bool,
    pub position: Vec3,
}

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tank>()
            .add_event::<SpawnTankEvent>()
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(spawn_tank));
    }
}

// systems can only have SPECIAL PARAMS
fn spawn_tank(
    mut ev_spawntank: EventReader<SpawnTankEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_spawntank.iter() {
        info!("Spawning tank.");
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(if ev.is_player {
                    Color::rgb(0.9, 0.3, 0.3).into()
                } else {
                    Color::rgb(0.3, 0.3, 0.9).into()
                }),
                transform: Transform::from_translation(ev.position),
                ..Default::default()
            },
            TankBundle {
                unit_type: Tank,
                health: Health(5.0),
                pickable: PickableBundle::default(),
            },
        ));
    }
}
