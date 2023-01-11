use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const WINDOW_WIDTH: f32 = 1370.0;
pub const WINDOW_HEIGHT: f32 = 750.0;

mod buildings;
mod camera;
mod gamestates;
mod levels;
mod utils;

pub use camera::*;
pub use gamestates::*;
pub use levels::sandbox_lvl::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,

                title: "RTS Game".to_string(),

                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_state(GameState::Gameplay)
        .add_plugin(SandboxLvlPlugin)
        .add_plugin(CameraPlugin)
        .run();
}
