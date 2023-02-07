use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;

pub const WINDOW_WIDTH: f32 = 1370.0;
pub const WINDOW_HEIGHT: f32 = 750.0;

mod buildings;
mod camera;
mod gamestates;
mod input_handling;
mod maps;
mod player;
mod units;
mod utils;

use buildings::buildings::BuildingsPluginGroup;
pub use camera::*;
pub use gamestates::*;
use input_handling::InputPlugin;
pub use maps::sandbox_map::*;
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
        .add_plugin(SandboxMapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InputPlugin)
        .add_plugins(BuildingsPluginGroup)
        .add_plugins(UnitsPluginGroup)
        .add_plugin(UtilsPlugin)
        //.add_plugin(RapierDebugRenderPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .run();
}
