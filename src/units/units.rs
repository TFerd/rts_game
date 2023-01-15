use bevy::{app::PluginGroupBuilder, prelude::*};

use super::tank::TankPlugin;

pub struct UnitsPluginGroup;

impl PluginGroup for UnitsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(TankPlugin)
    }
}
