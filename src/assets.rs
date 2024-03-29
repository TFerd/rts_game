use std::fs;

use crate::{
    buildings::{building_types::BuildingType, buildings::BuildingsConfig},
    units::units::{UnitType, UnitsConfig},
    GameState,
};

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

//const ASSETS_PATH: &str = "src/assets";

// TODO: add new factions and only load their models
// TODO: will have to split these up maybe, because if we have animations, materials, etc...
/// Loads
#[derive(AssetCollection, Resource)]
struct GameAssets {
    // #[asset(path = "meshes", collection(typed, mapped))]  // TODO: one day...
    // meshes: HashMap<String, Handle<Scene>>,
    #[asset(path = "meshes/tank.glb#Scene0")]
    pub tank: Handle<Scene>,

    #[asset(path = "meshes/marine.glb#Scene0")]
    pub marine: Handle<Scene>,

    #[asset(path = "meshes/base.gltf#Scene0")]
    pub base: Handle<Scene>,
}

/// Asset mappings to be used in the game.
#[derive(Resource)]
pub struct AssetMaps {
    pub unit_meshes: HashMap<UnitType, Handle<Scene>>,
    pub building_meshes: HashMap<BuildingType, Handle<Scene>>,
    // TODO: materials
    // TODO: building meshes
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay), // TODO: add each factions collection
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading);
        app.add_system(load_configs.in_schedule(OnExit(GameState::Loading)));
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

    // Load building configs
    let buildings_desc = fs::read_to_string("src/buildings/buildings.ron").unwrap();
    let buildings_config: BuildingsConfig =
        ron::de::from_str(&buildings_desc).unwrap_or_else(|e| {
            error!("Failed to load config: {}", e);
            std::process::exit(1);
        });
    info!("Done!");
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

    let mut unit_meshes: HashMap<UnitType, Handle<Scene>> = HashMap::default();
    let mut building_meshes: HashMap<BuildingType, Handle<Scene>> = HashMap::default();

    //**********    Assign UnitTypes their models here:    ******************
    unit_meshes.insert(UnitType::Tank, assets.tank.clone());
    unit_meshes.insert(UnitType::Marine, assets.marine.clone());

    //**********    Assign BuildingTypes their models here:    ******************
    building_meshes.insert(BuildingType::Base, assets.base.clone());

    // Add maps and configs to Resources
    commands.insert_resource(units_config);
    commands.insert_resource(buildings_config);
    commands.insert_resource(AssetMaps {
        unit_meshes,
        building_meshes,
    });
}
