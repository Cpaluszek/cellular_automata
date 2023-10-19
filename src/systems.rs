use bevy::app::AppExit;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
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

const ZOOM_SPEED: f32 = 1.;

pub fn scroll_events(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    // Todo: zoom to mouse pos -> https://github.com/bevyengine/bevy/issues/2580
    for event in scroll_event.iter() {
        for mut camera in camera_query.iter_mut() {
            let mut log_scale = camera.scale.ln();
            log_scale -= match event.unit {
                MouseScrollUnit::Line => event.y * ZOOM_SPEED * time.delta_seconds(),
                MouseScrollUnit::Pixel => event.y * ZOOM_SPEED * 0.1 * time.delta_seconds(),
            };
            camera.scale = log_scale.exp();
        }
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
