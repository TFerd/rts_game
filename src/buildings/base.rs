use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Base {
    pub is_player: bool,
    pub health: f32,
}

pub struct BaseDeathEvent; // gameover?

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Base>().add_system(base_death);
    }
}

fn base_death(bases: Query<&Base>, mut base_death_ew: EventWriter<BaseDeathEvent>) {
    for base in bases.iter() {
        if base.health <= 0.0 {
            // base destroyed
            // gameover?
            base_death_ew.send(BaseDeathEvent);
        }
    }
}

fn build_base() {}
