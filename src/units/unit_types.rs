use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::Deserialize;

use super::common::UnitsConfig;

#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum UnitType {
    Tank,
    Marine,
}

impl UnitType {
    pub fn spawn(self, commands: &mut Commands, config: &UnitsConfig, position: Vec3) {}

    // TODO: what about training unit, how will i know which to train | maybe i dont need to, just remove from queue then spawn the unit u removed
}
