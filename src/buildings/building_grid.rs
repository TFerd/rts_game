use bevy::prelude::*;
use bevy_inspector_egui::egui::Grid;

use crate::{player::Player, GameState};

pub struct BuildingGridPlugin;

// @TODO: need to do stuff like 'after loading the level, apply the grid'
impl Plugin for BuildingGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay)
                .with_system(init_grid_system)
                .label("init_grid"),
        )
        .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(handle_grid));
    }
}

#[derive(Component, Debug)]
pub struct BuildingGrid {
    grid: Vec<Vec<GridSquare>>,
}

impl BuildingGrid {
    pub fn new(n: usize) -> Self {
        let grid: Vec<Vec<GridSquare>> = (0..n)
            .into_iter()
            .map(|y| {
                (0..n)
                    .into_iter()
                    .map(|x| GridSquare {
                        available: true,
                        x,
                        y,
                    })
                    .collect()
            })
            .collect();

        Self { grid }
    }
}

#[derive(Component, Debug)]
pub struct GridSquare {
    pub available: bool,
    pub x: usize,
    pub y: usize,
}

fn handle_grid(grid: Query<&BuildingGrid>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F1) {
        let grid = grid.get_single();

        match grid {
            Ok(grid) => print_grid(grid),
            Err(_) => {
                error!("wt....f...");
            }
        }
    }
}

fn init_grid_system(mut commands: Commands) {
    info!("initting grid..");
    init_grid(&mut commands, 5);
    info!("Hit F1 to print the grid!");
}

// maybe not init system? maybe just a function?
// but i need to query for the grid?
// unless i pass as a parameter duh
fn init_grid(commands: &mut Commands, size: usize) {
    commands.spawn(BuildingGrid::new(size));
}

// @TODO: rewrite how we print this
fn print_grid(grid: &BuildingGrid) {
    grid.grid.iter().for_each(|x| {
        println!();
        x.iter().for_each(|y| {
            if y.available {
                print!(" O ");
            } else {
                print!(" X ");
            }
        });
    });
}
