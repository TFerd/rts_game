use crate::{assets::AssetMaps, units::units::UnitType};

use super::{building_types::BuildingType, events::BuildEvent};
use bevy::{app::PluginGroupBuilder, prelude::*, utils::HashMap};
use serde::Deserialize;

// pub struct BuildingsPluginGroup;
// impl PluginGroup for BuildingsPluginGroup {
//     fn build(self) -> bevy::app::PluginGroupBuilder {
//         PluginGroupBuilder::start::<Self>().add(BuildingsPlugin)
//     }
// }

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ground;

pub struct BuildingsPlugin;
impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BuildingMarker>()
            .add_event::<BuildEvent>();
    }
}

#[derive(Deserialize, Debug, Resource)]
pub struct BuildingsConfig(pub HashMap<BuildingType, Building>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingMarker; // Marker component // TODO: add all required stuff here, or create a building template

#[derive(Component)]
pub struct TrainingQueue {
    pub queue_size: u32,
    pub queue: Vec<UnitType>,
}

// TODO: implement this: add all required components
/// This is not a component, this simply holds all of the data for a general building
#[derive(Deserialize, Debug)]
pub struct Building {
    // TODO: queue size, hp, armor?, model
    pub health: f32,
    pub range: f32,
    pub atk_cd: f32,
    pub damage: f32,
    pub queue_size: f32,
    pub model: String,
}

// TODO: load config file

// TODO
fn read_build_events(
    mut ev_build: EventReader<BuildEvent>,
    mut commands: Commands,
    assets: Res<AssetMaps>,
) {
    for ev in ev_build.iter() {
        // create building at location

        // maybe make these go to a non-system function
        // match ev.building_type {
        //     BuildingType::Base => todo!(),
        //     BuildingType::Barracks => todo!(),
        //     BuildingType::CoalMine => todo!(),
        // }

        let scene = assets
            .building_meshes
            .get(&ev.building_type)
            .expect(&format!("No model for object {:?}", ev.building_type))
            .clone();
        let building = commands
            .spawn(SceneBundle {
                scene,
                transform: Transform::from_translation(ev.position),
                ..Default::default()
            })
            .id();
    }
}

fn build_building(commands: &mut Commands, building_type: BuildingType) {}
