use bevy::prelude::*;

use crate::GameState;

pub struct SandboxLvlPlugin;

impl Plugin for SandboxLvlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_scene));
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 65.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
            ..Default::default()
        })
        .insert(Name::new("Ground".to_string()));

    // player base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(-20.0, 1.0, -20.0),
        ..Default::default()
    });

    // enemy base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(20.0, 1.0, 20.0),
        ..Default::default()
    });
}
