// TODO: class
use bevy::prelude::*;

use super::unit_types::UnitType;

pub struct SpawnUnitEvent {
    pub is_player: bool,
    pub position: Vec3,
    pub unit_type: UnitType,
}

pub struct TrainUnitEvent {}

pub struct UnitDeathEvent;
