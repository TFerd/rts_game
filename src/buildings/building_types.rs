use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum BuildingType {
    Base,
    Barracks,
    CoalMine,
}
