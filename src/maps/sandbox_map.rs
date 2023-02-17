use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

use crate::{
    buildings::{
        base::{Base, BaseBundle},
        buildings::Ground,
    },
    units::{
        tank::Tank,
        unit_types::UnitType,
        units::{Speed, Unit, UnitBundle},
    },
    utils::{AttackCooldown, Damage, EnemyOwned, Health, PlayerOwned, Range},
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
    //let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 70.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(35.0, 0.1, 35.0))
        .insert(Ground)
        .insert(Name::new("Ground".to_string()));

    // player base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(-25.0, 2.5, 25.0),
            ..Default::default()
        })
        .insert(BaseBundle {
            building_type: Base,
            health: Health(10.0),
        })
        .insert(PlayerOwned)
        //.insert(PickableBundle::default())
        .insert(Collider::cuboid(2.5, 2.5, 2.5))
        .insert(Name::new("Player Base".to_string()));

    // enemy base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(25.0, 2.5, -25.0),
            ..Default::default()
        })
        .insert(BaseBundle {
            building_type: Base,
            health: Health(10.0),
        })
        .insert(EnemyOwned)
        //.insert(PickableBundle::default())
        .insert(Collider::cuboid(2.5, 2.5, 2.5))
        .insert(Name::new("Enemy Base".to_string()));

    // player tank
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.3, 0.9).into()),
            transform: Transform::from_translation(
                Vec3::ZERO
                    + Vec3 {
                        x: 0.0,
                        y: 0.5,
                        z: 0.0,
                    },
            ),
            ..Default::default()
        })
        .insert(UnitBundle {
            health: Health(5.0),
            range: Range(8),
            atk_cd: AttackCooldown(Timer::from_seconds(1.5, TimerMode::Once)),
            damage: Damage(2),
            unit_type: UnitType::Tank,
            speed: Speed(5),
        })
        .insert(Tank)
        .insert(Name::new("Tank".to_string()))
        .insert(PlayerOwned)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Unit);

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
