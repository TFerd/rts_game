use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{player::PlayerSelected, GameState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(unit_selection)
                .with_system(clear_selected),
        );
    }
}

// @TODO: have one main function that calls other functions based on input | idk if we can do that bc other functions cant query on their own

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

// @TODO: handle clicks,
fn unit_selection(
    rapier_context: Res<RapierContext>,
    window: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut selected: Query<&mut PlayerSelected>,
) {
    // @TODO: will have to account for click and hold/drag
    if mouse.just_released(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        let window = window.get_primary().unwrap();
        if let Some(cursor_position) = window.cursor_position() {
            let ray = camera.viewport_to_world(camera_transform, cursor_position);
            match ray {
                Some(ray) => {
                    info!("Ray created: {:?}", ray);
                    let collision = rapier_context.cast_ray(
                        ray.origin,
                        ray.direction,
                        100.0,
                        false,
                        QueryFilter::new(),
                    );
                    match collision {
                        Some(ent) => {
                            let mut selected = selected.single_mut();
                            if keyboard.pressed(KeyCode::LShift) {
                                info!("Clicked? {:?} | {:?}", ent.0, ent.1);

                                selected.0.insert(ent.0);
                                info!("Inserted entity into selected HashSet...");
                                info!("HashSet is now: {:?}", selected.0);
                            } else {
                                info!("Setting the Selected HashSet to just the clicked entity");
                                selected.0.clear();
                                selected.0.insert(ent.0);
                            }
                        }
                        None => {
                            info!("Nothing clicked");
                        }
                    }
                }
                None => {
                    warn!("Ray somehow not created?");
                }
            }
        }
    }
}

fn unit_movement(mouse: Res<Input<MouseButton>>, selected: Query<&PlayerSelected>) {
    if mouse.just_pressed(MouseButton::Right) {
        let selected = selected.single();
        // @TODO: pathfind
    }
}

fn clear_selected(keyboard: Res<Input<KeyCode>>, mut selected: Query<&mut PlayerSelected>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let mut selected = selected.single_mut();
        if selected.0.is_empty() {
            // @TODO: open pause menu
        } else {
            selected.0.clear();
            info!("Cleared Selected HashSet");
        }
    }
}
