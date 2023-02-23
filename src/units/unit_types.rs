use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Clone, Copy, Deserialize)]
pub enum UnitType {
    Tank,
    Marine,
}
