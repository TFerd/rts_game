use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    assets::AssetMaps,
    buildings::buildings::Ground,
    common::{AttackCooldown, Damage, Health, PlayerOwned, Range},
    units::units::{Speed, UnitMarker, UnitType, UnitsConfig},
    GameState,
};

pub struct SandboxMapPlugin;

impl Plugin for SandboxMapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_scene));
        app.add_system(spawn_scene.in_schedule(OnEnter(GameState::Gameplay)));
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut std_meshes: ResMut<Assets<Mesh>>,
    asset_maps: Res<AssetMaps>,
    unit_config: Res<UnitsConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    //let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands
        .spawn(PbrBundle {
            mesh: std_meshes.add(Mesh::from(shape::Plane {
                size: 70.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(35.0, 0.1, 35.0))
        .insert(Ground)
        .insert(Name::new("Ground".to_string()));

    // player base
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
    //         material: materials.add(Color::BLUE.into()),
    //         transform: Transform::from_xyz(-25.0, 2.5, 25.0),
    //         ..Default::default()
    //     })
    //     .insert(BaseBundle {
    //         building_type: Base,
    //         health: Health(10.0),
    //     })
    //     .insert(PlayerOwned)
    //     //.insert(PickableBundle::default())
    //     .insert(Collider::cuboid(2.5, 2.5, 2.5))
    //     .insert(Name::new("Player Base".to_string()));

    // enemy base

    let tank_config = unit_config.0.get(&UnitType::Tank).unwrap(); // TODO: handle unwrap or else
    info!("asset maps in sandbox map: {:?}", asset_maps.unit_meshes);
    info!(
        "fuck: {:?}",
        asset_maps.unit_meshes.get(&UnitType::Tank).unwrap()
    );
    // player tank
    commands
        .spawn(SceneBundle {
            // scene: asset_maps.unit_meshes.get(&UnitType::Tank).unwrap().clone(),
            scene: server.load("meshes/tank.glb#Scene0"),
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: 1.5,
                z: 0.0,
            }),
            ..Default::default()
        })
        .insert((
            Health(tank_config.health),
            Range(tank_config.range as u32),
            AttackCooldown(Timer::from_seconds(tank_config.atk_cd, TimerMode::Once)),
            Damage(tank_config.damage as u32),
            Speed(tank_config.speed as u32),
            PlayerOwned,
            UnitMarker,
            Collider::cuboid(0.5, 0.5, 0.5),
            Name::new("PlayerTank".to_string()),
        ));
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.3, 0.3, 0.9).into()),
    //         transform: Transform::from_translation(
    //             Vec3::ZERO
    //                 + Vec3 {
    //                     x: 0.0,
    //                     y: 0.5,
    //                     z: 0.0,
    //                 },
    //         ),
    //         ..Default::default()
    //     })
    //     .insert(Unit {
    //         health: Health(5.0),
    //         range: Range(8),
    //         atk_cd: AttackCooldown(Timer::from_seconds(1.5, TimerMode::Once)),
    //         damage: Damage(2),
    //         unit_type: UnitType::Tank,
    //         speed: Speed(5),
    //     })
    //     .insert(Name::new("Tank".to_string()))
    //     .insert(PlayerOwned)
    //     .insert(Collider::cuboid(0.5, 0.5, 0.5))
    //     .insert(UnitMarker);

    // TODO: add sun
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
