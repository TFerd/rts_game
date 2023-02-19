use bevy::prelude::*;

use super::building_types::BuildingType;

pub struct BuildEvent {
    pub player: bool,
    pub building_type: BuildingType,
    pub position: Vec3,
}
