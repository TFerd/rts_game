use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum UnitType {
    Tank,
    Marine,
    Miner,
}
