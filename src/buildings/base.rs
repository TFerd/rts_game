use crate::{player::Player, units::units::SpawnUnitEvent, *};

use bevy_mod_picking::PickableBundle;

use crate::{units::tank::SpawnTankEvent, utils::Health, GameState};
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Base;

#[derive(Bundle)]
pub struct BaseBundle {
    pub building_type: Base,
    pub health: Health,
}

pub struct BaseBuildEvent {
    is_player: bool,
    position: Vec3,
}

pub struct BaseDeathEvent; // gameover?

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Base>()
            .add_event::<BaseBuildEvent>()
            .add_event::<BaseDeathEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(base_death)
                    .with_system(build_base)
                    .with_system(train_tank),
            );
    }
}

fn base_death(
    mut commands: Commands,
    bases: Query<(Entity, &Health), With<Base>>,
    mut base_death_ew: EventWriter<BaseDeathEvent>,
) {
    for (entity, health) in bases.iter() {
        if health.0 <= 0.0 {
            // base destroyed
            // gameover?
            base_death_ew.send(BaseDeathEvent);
            commands.entity(entity).despawn_recursive();

            // play death animation
        }
    }
}

pub fn build_base(
    mut ev_basebuild: EventReader<BaseBuildEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_basebuild.iter() {
        info!(
            "Building is_player: {:?} | position: {:?}",
            ev.is_player, ev.position
        );

        // do other stuff before/after spawning entity?

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
                material: materials.add(if ev.is_player {
                    Color::rgb(0.1, 0.1, 1.0).into()
                } else {
                    Color::rgb(1.0, 0.1, 0.1).into()
                }),
                transform: Transform::from_translation(ev.position),
                ..Default::default()
            },
            BaseBundle {
                building_type: Base,
                health: Health(10.0),
            },
            PickableBundle::default(),
        ));

        // do other stuff before/after spawning entity?
    }
}

// @TODO: make this general?
fn train_tank(
    mut ev_spawnunit: EventWriter<SpawnUnitEvent>,
    query: Query<(&Selection, &Transform), With<Base>>,
    mut player: Query<&mut Player>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut player = player
        .get_single_mut()
        .expect("Could not find ONE player only!");

    if keyboard.just_pressed(KeyCode::Space) {
        for (selection, transform) in &query {
            if player.money >= 100 {
                if selection.selected() {
                    ev_spawnunit.send(SpawnUnitEvent {
                        is_player: true,
                        position: transform.translation
                            - Vec3 {
                                x: 0.0,
                                y: 0.0,
                                z: -5.0,
                            },
                        unit_type: units::units::UnitType::Tank,
                    });

                    player.money = player.money - 100;
                }
            }
        }
    }
}

fn train_miner() {
    todo!();
}
// @TODO: use this when we have assets?
// pub fn spawn_base_entity(
//     commands: &mut Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     position: Vec3,
//     is_player: bool,
// ) -> Entity {
//     commands
//         .spawn((
//             PbrBundle {
//                 mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
//                 material: materials.add(if is_player {
//                     Color::rgb(0.1, 0.1, 1.0).into()
//                 } else {
//                     Color::rgb(1.0, 0.1, 0.1).into()
//                 }),
//                 transform: Transform::from_translation(position),
//                 ..Default::default()
//             },
//             BaseBundle {
//                 building_type: Base,
//                 health: Health(10.0),
//             },
//             PickableBundle::default(),
//         ))
//         .id()
// }
