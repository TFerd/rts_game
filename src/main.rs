use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;

pub const WINDOW_WIDTH: f32 = 1370.0;
pub const WINDOW_HEIGHT: f32 = 750.0;

mod buildings;
mod camera;
mod gamestates;
mod input_handling;
mod levels;
mod player;
mod units;
mod utils;

use buildings::buildings::BuildingsPluginGroup;
pub use camera::*;
pub use gamestates::*;
// use input_handling::InputPlugin;
pub use levels::sandbox_lvl::*;
use player::PlayerPlugin;
use units::units::UnitsPluginGroup;
use utils::UtilsPlugin;

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
        .add_plugin(PlayerPlugin)
        //.add_plugin(InputPlugin)
        .add_plugins(BuildingsPluginGroup)
        .add_plugins(UnitsPluginGroup)
        .add_plugin(UtilsPlugin)
        .run();
}
