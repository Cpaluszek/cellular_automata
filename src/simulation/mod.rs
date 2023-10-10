use bevy::prelude::*;

use crate::GRID_SIZE;

const SPRITE_SIZE: f32 = 32.0;

const ALIVE_COLOR: Color = Color::RED;
const DEAD_COLOR: Color = Color::GRAY;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation);
    }
}

#[derive(Component)]
struct Cell {
    state: CellState,
}

#[derive(Debug)]
pub enum CellState {
    Alive,
    Dead,
}

fn setup_simulation(mut commands: Commands) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        (x as f32) * SPRITE_SIZE,
                        (y as f32) * SPRITE_SIZE,
                        0.0,
                    ),
                    sprite: Sprite::default(),
                    ..default()
                },
                Cell {
                    state: CellState::Dead,
                },
            ));
        }
    }
}
