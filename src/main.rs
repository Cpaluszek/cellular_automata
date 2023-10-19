use bevy::prelude::*;
use systems::{quit_application, setup_map, spawn_camera};

mod game;
mod systems;
use game::GameOfLifePlugin;

pub const WINDOW_WIDTH: f32 = 1600.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub const BOARD_SIZE: (i32, i32) = (320, 180);
pub const SPRITE_SIZE: f32 = 4.0;

pub const CYCLE_INTERVAL: f64 = 0.1;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game Of Life".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(GameOfLifePlugin::default())
        .add_systems(Startup, (spawn_camera, setup_map))
        .add_systems(Update, quit_application)
        .run();
}
