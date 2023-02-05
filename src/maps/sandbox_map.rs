use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

use crate::{
    buildings::base::{Base, BaseBundle},
    utils::{EnemyOwned, Health, PlayerOwned},
    GameState,
};

pub struct SandboxMapPlugin;

impl Plugin for SandboxMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_scene));
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 65.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(65.0, 0.1, 65.0))
        .insert(Name::new("Ground".to_string()));

    // player base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(-25.0, 1.0, 25.0),
            ..Default::default()
        })
        .insert(BaseBundle {
            building_type: Base,
            health: Health(10.0),
        })
        .insert(PlayerOwned)
        .insert(PickableBundle::default())
        .insert(Name::new("Player Base".to_string()));

    // enemy base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(25.0, 1.0, -25.0),
            ..Default::default()
        })
        .insert(BaseBundle {
            building_type: Base,
            health: Health(10.0),
        })
        .insert(EnemyOwned)
        .insert(PickableBundle::default())
        .insert(Name::new("Enemy Base".to_string()));

    // @TODO: add sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: (1500.0),
            shadows_enabled: (true),
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
