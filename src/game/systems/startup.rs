use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        components::CellPosition,
        resources::{CellBoard, CellEntityMap, CellSize},
    },
    resources::CellColor,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn life_setup(
    mut commands: Commands,
    mut cell_entities: ResMut<CellEntityMap>,
    board: Res<CellBoard>,
    cell_size: Res<CellSize>,
    cell_color: Res<CellColor>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let half_window_width = window.single().width() / 2.0;
    let half_window_height = window.single().height() / 2.0;

    for row in 0..board.height {
        for col in 0..board.width {
            let pos = CellPosition { col, row };
            if board.alive(pos) {
                let x = -half_window_width + (col as f32 * cell_size.width) + cell_size.width / 2.0;
                let y =
                    half_window_height - (row as f32 * cell_size.height) - cell_size.height / 2.0;

                // Cell Entity
                let new_cell = commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: cell_color.0,
                                custom_size: Some(Vec2::new(cell_size.width, cell_size.height)),
                                ..default()
                            },
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..default()
                        },
                        CellPosition { col, row },
                    ))
                    .id();
                cell_entities.0.insert(pos, new_cell);
            }
        }
    }
}
