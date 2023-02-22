use super::{base::BasePlugin, building_types::BuildingType, events::BuildEvent};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct BuildingsPluginGroup;

impl PluginGroup for BuildingsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BasePlugin)
            .add(BuildingsPlugin)
    }
}

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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingMarker; // Marker component // TODO: add all required stuff here, or create a building template

#[derive(Component)]
pub struct TrainingQueue;

// TODO: implement this: add all required components
pub struct Building {}

// TODO: load config file

// TODO
fn read_build_events(mut ev_build: EventReader<BuildEvent>, mut commands: Commands) {
    for ev in ev_build.iter() {
        // create building at location

        // maybe make these go to a non-system function
        match ev.building_type {
            BuildingType::Base => todo!(),
            BuildingType::Barracks => todo!(),
            BuildingType::CoalMine => todo!(),
        }
    }
}

fn build_building(commands: &mut Commands, building_type: BuildingType) {}
