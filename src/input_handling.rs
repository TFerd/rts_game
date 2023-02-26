use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    buildings::buildings::{BuildingMarker, Ground},
    common::{get_raycast_collision, EnemyOwned, Target},
    player::PlayerSelected,
    units::units::{TargetDestination, UnitMarker},
    GameState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(unit_selection)
                .with_system(clear_selected)
                .with_system(unit_movement)
                .with_system(debug_inputs),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay).with_system(display_debug_inputs),
        );
    }
}

// TODO: have one main function that calls other functions based on input | idk if we can do that bc other functions cant query on their own
// we can because we can use events u fucking idiot

// // fn menu_controls

// // fn gameplay controls
// // NOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO???????????????
// fn gameplay_controls(
//     keyboard: Res<Input<KeyCode>>,
//     mut ev_spawntank: EventWriter<SpawnTankEvent>,
//     mut ev_spawnunit: EventWriter<SpawnUnitEvent>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         // ev_spawntank.send(SpawnTankEvent {
//         //     is_player: true,
//         //     position: Vec3 {
//         //         x: 0.0,
//         //         y: 0.0,
//         //         z: 0.0,
//         //     },
//         // });
//     }
// }

// TODO: handle clicks,
fn unit_selection(
    ground: Query<Entity, With<Ground>>,
    rapier_context: Res<RapierContext>,
    window: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut selected: Query<&mut PlayerSelected>,
) {
    // TODO: will have to account for click and hold/drag
    if mouse.just_released(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        let window = window.get_primary().unwrap();
        let ent = get_raycast_collision(
            QueryFilter::new(),
            &rapier_context,
            &camera,
            &camera_transform,
            &window,
        );

        match ent {
            Some(ent) => {
                let mut selected = selected.single_mut();

                info!("Clicked? {:?} | {:?}", ent.0, ent.1);

                if let Ok(_) = ground.get(ent.0) {
                    info!("Ground selected, clearing selection...");
                    selected.0.clear();
                } else {
                    if keyboard.pressed(KeyCode::LShift) {
                        selected.0.insert(ent.0);
                        info!("Inserted entity into selected HashSet...");
                        info!("HashSet is now: {:?}", selected.0);
                    } else {
                        info!("Setting the Selected HashSet to just the clicked entity");
                        selected.0.clear();
                        selected.0.insert(ent.0);
                    }
                }
            }
            None => {
                warn!("Nothing clicked!");
            }
        }
    }
}

// Gives any selected units a target destination
fn unit_movement(
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    window: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    selected: Query<&PlayerSelected>,
    mut commands: Commands,
    units_and_buildings: Query<
        Entity,
        (
            With<EnemyOwned>,
            Or<(With<UnitMarker>, With<BuildingMarker>)>,
        ),
    >,
    ground: Query<Entity, With<Ground>>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.get_primary().unwrap();
    if mouse.just_pressed(MouseButton::Right) {
        info!("Right mouse clicked...");
        let selected = selected.single();

        let collision = get_raycast_collision(
            QueryFilter::new(),
            &rapier_context,
            camera,
            camera_transform,
            &window,
        );

        match collision {
            Some(ent) => {
                if let Ok(_) = ground.get(ent.0) {
                    // Ground selected, set target destination
                    for selected_ent in selected.0.iter() {
                        commands
                            .entity(*selected_ent)
                            .insert(TargetDestination(ent.1));
                    }
                } else if let Ok(_) = units_and_buildings.get(ent.0) {
                    // Unit or building selected, make them a target
                    for selected_ent in selected.0.iter() {
                        commands.entity(*selected_ent).insert(Target(ent.0));
                    }
                } else {
                    warn!("WTF, clicked on neither a unit, building, or ground");
                }
            }
            None => {
                warn!("Nothing clicked!");
            }
        }

        // check the entity type of collision: building, unit, or ground

        // TODO: pathfind
    }
}

fn clear_selected(keyboard: Res<Input<KeyCode>>, mut selected: Query<&mut PlayerSelected>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let mut selected = selected.single_mut();
        if selected.0.is_empty() {
            // TODO: open pause menu
        } else {
            selected.0.clear();
            info!("Cleared Selected HashSet");
        }
    }
}

fn display_debug_inputs() {
    info!("Debug options:");
    info!("Press F1 to display selected entities");
    info!("Press 1 to build a base");
}

fn debug_inputs(keyboard: Res<Input<KeyCode>>, selected: Query<&PlayerSelected>) {
    if keyboard.just_pressed(KeyCode::F1) {
        let selected = selected.single();
        info!("Selected: {:?}", selected.0);
    }
}
