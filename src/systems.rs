use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{ConwayCellState, Moore2dCell},
    BOARD_SIZE, CELL_PROBABILITY, SPRITE_SIZE,
};

pub fn quit_application(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_events_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_events_writer.send(AppExit);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_map(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = BOARD_SIZE;

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(size_x as f32 * SPRITE_SIZE) / 2.,
            -(size_y as f32 * SPRITE_SIZE) / 2.,
            0.,
        )))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
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
                        Moore2dCell::new(IVec2::new(x, y)),
                        state,
                    ));
                }
            }
        });
    println!("Map generated");
}
