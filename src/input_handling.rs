use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(on_click));
    }
}

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
fn on_click(
    rapier_context: Res<RapierContext>,
    window: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_released(MouseButton::Right) {
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
                            info!("Clicked? {:?} | {:?}", ent.0, ent.1);
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
