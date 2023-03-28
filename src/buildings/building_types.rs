use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum BuildingType {
    Base,
    Barracks,
    CoalMine,
}
