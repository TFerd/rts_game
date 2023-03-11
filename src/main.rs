use assets::GameAssetsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rapier3d::prelude::*;

pub const WINDOW_WIDTH: f32 = 1370.0;
pub const WINDOW_HEIGHT: f32 = 750.0;

mod assets;
mod buildings;
mod camera;
mod common;
mod gamestates;
mod input_handling;
mod maps;
mod player;
mod units;

use buildings::{building_grid::BuildingGridPlugin, buildings::BuildingsPluginGroup};
pub use camera::*;
use common::UtilsPlugin;
pub use gamestates::*;
use input_handling::InputPlugin;
pub use maps::sandbox_map::*;
use player::PlayerPlugin;
use units::units::UnitsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .add_plugin(GameAssetsPlugin)
        .add_plugin(SandboxMapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InputPlugin)
        .add_plugins(BuildingsPluginGroup)
        .add_plugin(UnitsPlugin)
        .add_plugin(UtilsPlugin)
        .add_plugin(BuildingGridPlugin)
        //.add_plugin(RapierDebugRenderPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
