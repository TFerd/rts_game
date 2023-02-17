use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier3d::prelude::{Collider, Restitution, RigidBody};

use crate::{
    player::PlayerSelected,
    units::tank::Tank,
    utils::{AttackCooldown, Damage, EnemyOwned, Health, PlayerOwned, Range},
    GameState,
};

use super::{
    events::{SpawnUnitEvent, UnitDeathEvent},
    tank::TankPlugin,
    unit_types::UnitType,
};

// put plugins from other unit classes here
pub struct UnitsPluginGroup;

impl PluginGroup for UnitsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TankPlugin)
            .add(UnitsPlugin)
    }
}

struct UnitsPlugin;
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_inspectable::<TargetDestination>()
            .add_event::<SpawnUnitEvent>()
            .add_event::<UnitDeathEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(spawn_unit)
                    .with_system(unit_death)
                    .with_system(move_unit),
            );
    }
}

#[derive(Component)]
pub struct Unit; // Marker component

#[derive(Bundle)]
pub struct UnitBundle {
    pub health: Health,
    pub range: Range,
    pub atk_cd: AttackCooldown,
    pub damage: Damage,
    pub unit_type: UnitType,
    // @TODO: unit_flag: Unit,
    pub speed: Speed,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub u32);

#[derive(Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct TargetDestination(pub Vec3); // Position unit wants to move to. NOT the position of their target

// @TODO: this
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TrainingUnit {
    //building: Entity, // building which they will spawn out of
} // ?

/*************************
        Systems
**************************/

// should i make all of these send out events? no? idk lots of extra code
fn spawn_unit(
    mut commands: Commands,
    mut ev_spawnunit: EventReader<SpawnUnitEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // @TODO: can write less code here
    for ev in ev_spawnunit.iter() {
        // let unit = commands.spawn(blah blah);
        // match unit_type {
        //    tank => unit.insert(...)
        //    etc...
        // }

        // add common stuff here, like maybe stuff passed in from event
        // ok turns out like nothing is common
        // maybe make stuff from a config file
        match ev.unit_type {
            UnitType::Tank => {
                info!("Spawning tank.");
                let tank = commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(if ev.is_player {
                            Color::rgb(0.3, 0.3, 0.9).into()
                        } else {
                            Color::rgb(0.9, 0.3, 0.3).into()
                        }),
                        transform: Transform::from_translation(ev.position),
                        ..Default::default()
                    })
                    .insert(UnitBundle {
                        health: Health(5.0),
                        range: Range(8),
                        atk_cd: AttackCooldown(Timer::from_seconds(1.5, TimerMode::Once)),
                        damage: Damage(2),
                        unit_type: UnitType::Tank,
                        speed: Speed(5),
                    })
                    .insert(Tank)
                    .insert(Name::new("Tank".to_string()))
                    .id();

                if ev.is_player {
                    commands.entity(tank).insert(PlayerOwned);
                } else if !ev.is_player {
                    commands.entity(tank).insert(EnemyOwned);
                } else {
                    unreachable!("WTF");
                }
            }
            UnitType::Marine => {
                info!("Spawning marine.");
                todo!();
            }
            UnitType::Miner => {
                info!("Spawning miner.");
                todo!();
            }
        }
    }
}

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

// @TODO: fix this shit
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
            let direction = global_transform.translation() - target.0;
            transform.translation += time.delta_seconds() * speed.0 as f32 * direction.normalize();
        }
    }
}
