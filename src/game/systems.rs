use bevy::prelude::*;

use super::components::*;
use super::{CELL_COLOR, CELL_SIZE, CELL_SPRITE_SIZE, HALF_CELL_SIZE};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)
pub fn spawn_cells(mut commands: Commands) {
    let cell_sprite = Sprite {
        custom_size: Some(CELL_SPRITE_SIZE),
        color: CELL_COLOR,
        ..default()
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0 + HALF_CELL_SIZE, 0.0 + HALF_CELL_SIZE, 0.0),
            sprite: cell_sprite,
            ..default()
        },
        Cell {
            state: CellState::Alive,
        },
    ));
}
