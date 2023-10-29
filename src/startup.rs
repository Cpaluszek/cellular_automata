use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{BoardBackground, CellContainer, ConwayCellState, Moore2dCell},
    BACKGROND_COLOR, BOARD_SIZE, CELL_PROBABILITY, SPRITE_SIZE, ui::SIDE_PANEL_WIDTH,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_map(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let size = BOARD_SIZE;

    // Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    size as f32 * SPRITE_SIZE,
                    size as f32 * SPRITE_SIZE,
                )),
                color: BACKGROND_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(SIDE_PANEL_WIDTH * 0.5 -SPRITE_SIZE * 0.5, -SPRITE_SIZE * 0.5, 0.),
            ..default()
        },
        BoardBackground {},
    ));

    // Cells
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                SIDE_PANEL_WIDTH * 0.5 -(size as f32 * SPRITE_SIZE) * 0.5,
                -(size as f32 * SPRITE_SIZE) / 2.,
                0.,
            )),
            CellContainer {},
        ))
        .with_children(|builder| {
            for y in 0..size {
                for x in 0..size {
                    let state = ConwayCellState(rng.gen_bool(CELL_PROBABILITY));
                    builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(SPRITE_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                x as f32 * SPRITE_SIZE,
                                y as f32 * SPRITE_SIZE,
                                0.,
                            ),
                            ..default()
                        },
                        Moore2dCell::new(IVec2::new(x as i32, y as i32)),
                        state,
                    ));
                }
            }
        });
    println!("Map generated");
}
