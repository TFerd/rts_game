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
        app.register_type::<Building>().add_event::<BuildEvent>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Building; // Marker component

#[derive(Component)]
pub struct TrainingQueue;

// TODO
fn build_buildings(mut ev_build: EventReader<BuildEvent>, mut commands: Commands) {
    for ev in ev_build.iter() {
        // create building at location
    }
}
