use bevy::{prelude::*, reflect};
use bevy_inspector_egui::egui::Grid;

use crate::GameState;

pub struct BuildingGridPlugin;

// TODO: need to do stuff like 'after loading the level, apply the grid'
// TODO: system label
impl Plugin for BuildingGridPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(init_grid_system))
        //     .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(handle_grid));
        app.add_system(init_grid_system.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(handle_grid.in_set(OnUpdate(GameState::Gameplay)));
    }
}

#[derive(Component)]
//#[reflect(Component)]
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
    // if keyboard.just_pressed(KeyCode::F1) {
    //     let grid = grid.get_single();

    //     match grid {
    //         Ok(grid) => print_grid(grid),
    //         Err(e) => {
    //             //error!("BuildingGrid not found!");

    //             error!("{:?}", e);
    //         }
    //     }
    // }
}

// TODO: get ground size and pass to the function helper
// TODO: rename this to be like post-init or something
fn init_grid_system(mut commands: Commands) {
    info!("initting grid..");
    init_grid(&mut commands, 5);
}

// maybe not init system? maybe just a function?
// but i need to query for the grid?
// unless i pass as a parameter duh
fn init_grid(commands: &mut Commands, size: usize) {
    let grid = commands
        .spawn((BuildingGrid::new(size), Name::new("Grid".to_string())))
        .id();

    // create entities
    // or draw lines
}

// TODO: rewrite how we print this
fn print_grid(grid: &BuildingGrid) {
    let mut output = String::new();

    grid.grid.iter().for_each(|x| {
        output.push_str("\n");
        x.iter().for_each(|y| {
            if y.available {
                output.push_str(" O ");
            } else {
                output.push_str(" X ");
            }
        });
    });

    info!("{}", output);
}

//on change: jhide
