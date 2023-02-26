use core::fmt;

use std::fs;

use bevy::{app::PluginGroupBuilder, prelude::*, utils::HashMap};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier3d::prelude::{Collider, Restitution, RigidBody};
use serde::Deserialize;

use crate::{
    common::{AttackCooldown, Damage, EnemyOwned, Health, PlayerOwned, Range},
    player::PlayerSelected,
    GameState,
};

use super::events::{SpawnUnitEvent, UnitDeathEvent};

pub struct UnitsPlugin;
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_inspectable::<TargetDestination>()
            .add_event::<SpawnUnitEvent>()
            .add_event::<UnitDeathEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(Self::load_units_config.label("units")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(spawn_unit)
                    .with_system(unit_death)
                    .with_system(move_unit),
            );
    }
}

impl UnitsPlugin {
    /// Loads the units config
    /// Runs upon entering Gameplay state
    fn load_units_config(mut commands: Commands) {
        // Load units config
        let units_desc = fs::read_to_string("src/units/units.ron").unwrap();

        let units_config: UnitsConfig = ron::de::from_str(&units_desc).unwrap_or_else(|e| {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        });
        //info!("Units config: {:?}", units_config);
        commands.insert_resource(units_config);
    }
}

#[derive(Deserialize, Debug, Resource)]
pub struct UnitsConfig(pub HashMap<UnitType, Unit>);

/// Holds info each unit.
/// Data is stored in a HashMap with the UnitType enum as the key.
///
/// * `model` - The name of the mesh's file. You will not use this to spawn the model. Use model_handle instead.
#[derive(Deserialize, Debug)]
pub struct Unit {
    // TODO: maybe change the types to the components again
    pub health: f32,
    pub range: f32,
    pub atk_cd: f32,
    pub damage: f32,
    pub speed: f32,
    pub model: String,
    // TODO: training time
    // TODO: building being trained out of
}

#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum UnitType {
    Tank,
    Marine,
}

impl UnitType {
    // pub fn spawn(self, commands: &mut Commands, config: &UnitsConfig, position: Vec3) -> Entity {
    //     let unit = commands.spawn(bundle)
    // }

    // TODO: what about training unit, how will i know which to train | maybe i dont need to, just remove from queue then spawn the unit u removed
}

/********************
 * START COMPONENTS
 ********************/
#[derive(Component)]
pub struct UnitMarker; // Marker component

#[derive(Component, Reflect, Default, Deserialize)]
#[reflect(Component)]
pub struct Speed(pub u32);

#[derive(Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct TargetDestination(pub Vec3); // Position unit wants to move to. NOT the position of their target

// TODO: this
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TrainingUnit {
    //building: Entity, // building which they will spawn out of
} // ?

/*************************
        Systems
**************************/

// should i make all of these send out events? no? idk lots of extra code
fn spawn_unit() {}

// this should happen before spawn unit
fn train_unit() {}

// Despawns enemies and sends death event for statistics purposes
fn unit_death(
    mut commands: Commands,
    mut ev_death: EventWriter<UnitDeathEvent>,
    query: Query<(Entity, &Health), With<UnitType>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            // Death
            ev_death.send(UnitDeathEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

// TODO: maybe move this under the unittypes
// TODO: fix this shit
fn move_unit(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &GlobalTransform,
        &mut Transform,
        &TargetDestination,
        &Speed,
    )>,
) {
    for (entity, global_transform, mut transform, target, speed) in query.iter_mut() {
        info!(
            "Distance is: {}",
            global_transform.translation().distance(target.0)
        );
        if global_transform.translation().distance(target.0) < 0.5 {
            info!("Entity arrived at destination, removing target");
            commands.entity(entity).remove::<TargetDestination>();
        } else {
            let mut direction = global_transform.translation() - target.0;
            direction.y = 0.0;
            transform.translation -= time.delta_seconds() * speed.0 as f32 * direction.normalize();
        }
    }
}
