use bevy::prelude::*;

use crate::utils::Health;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tank;

#[derive(Bundle)]
pub struct TankBundle {
    pub unit_type: Tank,
    pub health: Health,
}

fn spawn_tank(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),

            ..Default::default()
        })
        .id()
}
