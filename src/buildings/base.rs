use crate::{
    player::{Player, PlayerSelected},
    units::{events::SpawnUnitEvent, unit_types::UnitType},
    *,
};

use bevy_mod_picking::PickableBundle;

use crate::{utils::Health, GameState};

use super::{building_types::BuildingType, events::BuildEvent};
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

fn send_building_events(
    rapier_context: Res<RapierContext>,
    mut ev_buildbuilding: EventWriter<BuildEvent>,
    window: Res<Windows>,
    keyboard: Res<Input<KeyCode>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut player: Query<&mut Player>,
    selected: Query<&PlayerSelected>,
) {
    let selected = selected.single();
    let mut player = player
        .get_single_mut()
        .expect("Could not find ONE player only!");
    let (camera, camera_transform) = camera.single();
    let window = window.get_primary().unwrap();
    let cursor = window.cursor_position().expect("Cursor not found?");
    let ray = camera.viewport_to_world(camera_transform, cursor);

    if keyboard.just_pressed(KeyCode::Key1) {
        // must check if something is there, and if player has money
        ev_buildbuilding.send(BuildEvent {
            player: true,
            building_type: BuildingType::Barracks,
            position: Vec3::ZERO,
        })
    }
}

// TODO: make this general?
fn train_tank(
    mut ev_spawnunit: EventWriter<SpawnUnitEvent>,
    query: Query<(&PlayerSelected, &Transform), With<Base>>,
    mut player: Query<&mut Player>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut player = player
        .get_single_mut()
        .expect("Could not find ONE player only!");

    if keyboard.just_pressed(KeyCode::Space) {
        info!("Train tank");
    }

    if keyboard.just_pressed(KeyCode::E) {
        info!("Train enemy tank, debug purposes");
    }
}

fn train_miner() {
    todo!();
}
// TODO: use this when we have assets?
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

// fires off unit training depending on the input
fn handle_input(keyboard: Res<Input<KeyCode>>) {}
