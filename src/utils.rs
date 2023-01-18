/*
    THIS CLASS IS FOR COMPONENTS/SYSTEMS THAT CAN BE SHARED BETWEEN BUILDINGS AND UNITS
    @TODO: rename to globals.rs or global.rs ?
*/

use bevy::{prelude::*, utils::FloatOrd};

use crate::GameState;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Range(pub u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackCooldown(pub Timer);

// Sparse-set storage because this is added/removed frequently
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Target {
    pub target: Entity,
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Range>()
            .register_type::<AttackCooldown>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(tick_timers)
                    .with_system(aquire_target),
            );
    }
}

fn tick_timers(mut timers: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut timer in timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

// find closest target in range
// maybe need to add With<PlayerOwned>, or maybe not so enemies can use this too
// still would need two diff teams components to tell who enemies are
// @TODO: make this have less calculations, (try to capture and reuse values)
fn aquire_target(
    mut commands: Commands,
    shooters: Query<(Entity, &Range, &GlobalTransform), Without<Target>>,
    targets: Query<(Entity, &GlobalTransform), With<Health>>, // @TODO: With<EnemyOwned>
) {
    for (shooter_ent, range, shooter_transform) in shooters.iter() {
        let closest_enemy = targets.iter().min_by_key(|closest_transform| {
            FloatOrd(Vec3::distance(
                shooter_transform.translation(),
                closest_transform.1.translation(),
            ))
        }); //@TODO: dont think i need this?

        if let Some(closest_enemy) = closest_enemy {
            let distance = Vec3::distance(
                closest_enemy.1.translation(),
                shooter_transform.translation(),
            );

            if distance.abs() <= range.0 as f32 {
                commands.entity(shooter_ent).insert(Target {
                    target: closest_enemy.0,
                });

                info!(
                    "Entity: {:?} is targeting entity: {:?}",
                    shooter_ent, closest_enemy.0
                );
            }
        }
    }
}

// if target moves out of range, remove the target
fn remove_target() {}

// for entities with a target, shoot them
fn attack_target() {}
