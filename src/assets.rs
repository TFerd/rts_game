use std::fs;

use crate::{
    units::units::{UnitType, UnitsConfig},
    GameState,
};

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

//const ASSETS_PATH: &str = "src/assets";

// TODO: add new factions and only load their models
/// Loads
#[derive(AssetCollection, Resource)]
struct GameAssets {
    // #[asset(path = "meshes", collection(typed, mapped))]  // TODO: one day...
    // meshes: HashMap<String, Handle<Scene>>,
    #[asset(path = "meshes/tank.glb#Scene0")]
    pub tank: Handle<Scene>,
}

/// Asset mappings to be used in the game.
#[derive(Resource)]
pub struct AssetMaps {
    pub unit_meshes: HashMap<UnitType, Handle<Scene>>,
    // TODO: materials
    // TODO: building meshes
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Gameplay)
                .with_collection::<GameAssets>(), // TODO: add each factions collection
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Loading)
                .with_system(load_configs)
                .label("assets"),
        );
    }
}

/// Loads configurations and models for units/buildings into Resources.
/// Reads from their respective .ron files.
fn load_configs(mut commands: Commands, assets: Res<GameAssets>) {
    info!("Loading configuration files into maps...");
    // Load unit configs
    let units_desc = fs::read_to_string("src/units/units.ron").unwrap();
    let units_config: UnitsConfig = ron::de::from_str(&units_desc).unwrap_or_else(|e| {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    // // Load unit models (meshes)
    // let mut unit_meshes: HashMap<UnitType, Handle<Scene>> = HashMap::default();
    // for unit in units_config.0.iter() {
    //     let mut key = String::from("meshes\\");
    //     key.push_str(&unit.1.model);
    //     key.push_str(".glb#");
    //     info!("key: {:?}", key);
    //     info!("assets: {:?}", assets.meshes);

    //     let handle = assets.meshes.get(&key).unwrap_or_else(|| {
    //         error!("Failed to load mesh from config: {}", unit.1.model);
    //         std::process::exit(1);
    //     });

    //     // TODO: maybe assert the result of adding?
    //     unit_meshes.insert(*unit.0, handle.clone());
    // }

    commands.insert_resource(units_config);
    commands.insert_resource(AssetMaps { unit_meshes });
}
