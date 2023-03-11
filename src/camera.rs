use bevy::{input::mouse::MouseMotion, prelude::*};
// use bevy_mod_picking::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraMovement>()
            .add_startup_system(spawn_camera)
            .add_system(camera_controls);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CameraMovement {
    panning: bool,
    pan_speed: f32,
    mouse_pan_speed: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-10.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraMovement {
            panning: false,
            pan_speed: 15.0,
            mouse_pan_speed: 10.0,
        });
}

// TODO: add keybinds from settings
fn camera_controls(
    mut camera_q: Query<(&mut Transform, &mut CameraMovement), With<Camera3d>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut ev_mouse_motion: EventReader<MouseMotion>,
) {
    let (mut camera, mut cam_movement) = camera_q.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let mut pan = Vec2::ZERO;

    if mouse_input.pressed(MouseButton::Middle) {
        for ev in ev_mouse_motion.iter() {
            pan += ev.delta;
            info!("Pan: {:?}", pan);

            let pan_final = Vec3::new(pan.x, 0.0, pan.y).normalize();
            camera.translation +=
                pan_final * time.delta_seconds() * (cam_movement.mouse_pan_speed / 2.0);
        }
    } else {
        // go forward
        if keyboard_input.pressed(KeyCode::W) {
            camera.translation += forward * time.delta_seconds() * cam_movement.pan_speed;
        }

        // go opposite of forward (backward)
        if keyboard_input.pressed(KeyCode::S) {
            camera.translation -= forward * time.delta_seconds() * cam_movement.pan_speed;
        }

        // go right
        if keyboard_input.pressed(KeyCode::A) {
            camera.translation += left * time.delta_seconds() * cam_movement.pan_speed;
        }

        // go opposite of right (left)
        if keyboard_input.pressed(KeyCode::D) {
            camera.translation -= left * time.delta_seconds() * cam_movement.pan_speed;
        }
    }

    if mouse_input.just_released(MouseButton::Right) {
        cam_movement.panning = false;
    }
}
