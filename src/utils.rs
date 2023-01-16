use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Range(pub u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackCooldown(Timer);

#[derive(Component)]
pub struct Target {
    target: Entity,
}

// just for registering types for all the utils
// i guess
// @TODO: i'm removing all util files, so remove this and unhook from main
pub struct UtilPluginGroup;

impl PluginGroup for UtilPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(UtilsPlugin)
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Range>()
            .register_type::<AttackCooldown>();
    }
}

// fn tick_timers(mut timers: Query<(&mut AttackCooldown)>) {
//     for mut timer in timers.iter_mut() {
//         timer.0.ti
//     }
// }
