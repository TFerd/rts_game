use bevy::{app::PluginGroupBuilder, prelude::*};

use super::base::BasePlugin;
pub struct BuildingsPluginGroup;

impl PluginGroup for BuildingsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(BasePlugin)
    }
}
