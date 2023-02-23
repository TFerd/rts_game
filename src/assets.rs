use std::fs;

use bevy::{asset::AssetLoader, prelude::*, utils::HashMap};

use crate::{
    units::common::{UnitType, UnitsConfig},
    GameState,
};

//const ASSETS_PATH: &str = "src/assets";

// LOAD IMAGES, THEN ASSIGN THEM TO THEIR HASHMAPS?

#[derive(Resource)]
pub struct GameAssets {
    pub unit_meshes: HashMap<UnitType, Handle<Mesh>>,
    // TODO: materials
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(GameState::Loading)
                .with_system(load_assets)
                .label("assets"),
        );
    }
}

/// Loads units and building configurations and models.
/// Reads from their respective .ron files.
fn load_assets(mut commands: Commands) {
    // Load units
    let units_desc = fs::read_to_string("src/units/units.ron").unwrap();

    let units_config: UnitsConfig = ron::de::from_str(&units_desc).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    // Load unit models (meshes)
    let mut unit_meshes: HashMap<UnitType, Handle<Mesh>> = HashMap::default();
    for unit in units_config.0.iter() {
        // TODO: maybe assert the result of adding?
        unit_meshes.insert(unit.0, unit)
    }

    commands.insert_resource(units_config);
    commands.insert_resource(GameAssets { unit_meshes });
}
