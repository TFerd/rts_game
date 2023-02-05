/*
    THIS CLASS IS FOR COMPONENTS/SYSTEMS THAT CAN BE SHARED BETWEEN BUILDINGS AND UNITS
    @TODO: rename to globals.rs or global.rs ?
*/

use bevy::{prelude::*, utils::FloatOrd};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::GameState;

/********************
 * START COMPONENTS
 ********************/

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerOwned;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EnemyOwned;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Range(pub u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackCooldown(pub Timer);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Damage(pub u32);

// Sparse-set storage because this is added/removed frequently
#[derive(Component, Inspectable)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

/********************
 *  END COMPONENTS
 ********************/

pub struct AttackEvent {
    pub target: Entity,
    pub damage: u32,
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Range>()
            .register_type::<AttackCooldown>()
            .register_type::<Damage>()
            .register_inspectable::<Target>()
            .add_event::<AttackEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(tick_timers)
                    .with_system(aquire_target)
                    .with_system(remove_target)
                    .with_system(attack_target)
                    .with_system(apply_damage),
            );
    }
}

/*************************
        Systems
**************************/

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
    shooters: Query<(Entity, &Range, &GlobalTransform), (Without<Target>, With<PlayerOwned>)>, // @TODO: Make this work for both enemy and player
    targets: Query<(Entity, &GlobalTransform), (With<Health>, With<EnemyOwned>)>, // @TODO: With<EnemyOwned>
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
                commands.entity(shooter_ent).insert(Target(closest_enemy.0));

                info!(
                    "Entity: {:?} is targeting entity: {:?}",
                    shooter_ent, closest_enemy.0
                );
            }
        }
    }
}

// if target moves out of range, remove the target
// @TODO: maybe remove the world param and do the same thing as apply_damage for getting entity
fn remove_target(
    world: &World,
    mut commands: Commands,
    query: Query<(Entity, &Target, &GlobalTransform, &Range), With<PlayerOwned>>,
) {
    for (shooter_ent, target, shooter_transform, shooter_range) in query.iter() {
        let target_ent = world.get_entity(target.0);

        match target_ent {
            Some(ent) => {
                let target_transform = ent.get::<GlobalTransform>().unwrap();

                // calculate if in range
                let distance = shooter_transform
                    .translation()
                    .distance(target_transform.translation());

                if distance > shooter_range.0 as f32 {
                    // de-target
                    commands.entity(shooter_ent).remove::<Target>();
                    info!("Removing target from {:?}", shooter_ent);
                }
            }
            None => {
                warn!("Entity not found, removing Target component...");
                commands.entity(shooter_ent).remove::<Target>();
            }
        }
    }
}

// for entities with a target, shoot them
fn attack_target(
    mut ev_attack: EventWriter<AttackEvent>,
    mut query: Query<(&Damage, &Target, &mut AttackCooldown)>,
) {
    for (damage, target, mut atk_cd) in query.iter_mut() {
        debug!("hello");
        if atk_cd.0.finished() {
            info!("Attacking..");
            ev_attack.send(AttackEvent {
                target: target.0,
                damage: damage.0,
            });
            atk_cd.0.reset();
        }
    }
}

// reads attack events and applies damage
// fn apply_damage(world: &World, mut ev_attack: EventReader<AttackEvent>) {
//     for ev in ev_attack.iter() {
//         // dmg
//         let target = world.get_entity_mut(ev.target);

//         match target {
//             Some(mut target) => {
//                 info!("Doing {:?} damage to entity: {:?}", ev.damage, ev.target);
//                 let mut target_hp = target.get_mut::<Health>().unwrap();
//                 target_hp.0 -= ev.damage as f32;
//             }
//             None => warn!("apply_damage: Entity not found!"),
//         }
//     }
// }
fn apply_damage(mut ev_attack: EventReader<AttackEvent>, mut query: Query<&mut Health>) {
    for ev in ev_attack.iter() {
        if let Ok(mut target_hp) = query.get_mut(ev.target) {
            target_hp.0 -= ev.damage as f32;
            info!("{:?} took {:?} damage.", ev.target, ev.damage);
        }
    }
}
