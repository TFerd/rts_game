use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    units::tank::Tank,
    utils::{AttackCooldown, Health, Range},
    GameState,
};

use super::tank::TankPlugin;

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
        app.add_event::<SpawnUnitEvent>()
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(spawn_unit));
    }
}

#[derive(Component)]
pub struct Unit;

pub struct SpawnUnitEvent {
    pub is_player: bool,
    pub position: Vec3,
    pub unit_type: UnitType,
}

#[derive(Component)]
pub enum UnitType {
    Tank,
    Marine,
    Miner,
}

#[derive(Bundle)]
pub struct UnitBundle {
    health: Health,
    range: Range,
    atk_cd: AttackCooldown,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub u32);

// should i make all of these send out events? no? idk lots of extra code
fn spawn_unit(
    mut commands: Commands,
    mut ev_spawnunit: EventReader<SpawnUnitEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_spawnunit.iter() {
        match ev.unit_type {
            UnitType::Tank => {
                info!("Spawning tank.");
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(if ev.is_player {
                            Color::rgb(0.9, 0.3, 0.3).into()
                        } else {
                            Color::rgb(0.3, 0.3, 0.9).into()
                        }),
                        transform: Transform::from_translation(ev.position),
                        ..Default::default()
                    })
                    .insert(UnitBundle {
                        health: Health(5.0),
                        range: Range(8),
                        atk_cd: AttackCooldown(Timer::from_seconds(1.5, TimerMode::Repeating)),
                    })
                    .insert(Tank)
                    .insert(Name::new("Tank".to_string()));
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
