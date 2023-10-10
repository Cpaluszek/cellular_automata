use bevy::prelude::*;
use rand::*;

use super::{components::*, CELL_SIZE, GRID_SIZE};
use super::{CELL_COLOR, CELL_SPRITE_SIZE};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)
pub fn spawn_cells(mut commands: Commands) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_x = x - GRID_SIZE / 2;
            let current_y = y - GRID_SIZE / 2;
            let alive = random::<f32>() < 0.2;
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        (current_x as f32) * CELL_SIZE,
                        (current_y as f32) * CELL_SIZE,
                        0.0,
                    ),
                    sprite: Sprite {
                        custom_size: Some(CELL_SPRITE_SIZE),
                        color: if alive {
                            CELL_COLOR
                        } else {
                            Color::NONE.into()
                        },
                        ..default()
                    },
                    ..default()
                },
                Cell {
                    state: if alive {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    },
                },
            ));
        }
    }
}

pub fn get_next_generation(mut cell_query: Query<(&mut Transform, &mut Cell)>) {
    for (mut transform, cell) in cell_query.iter_mut() {
        // Do something
        // println!("{} - {:?}", transform.translation.to_string(), cell.state);
    }
}
