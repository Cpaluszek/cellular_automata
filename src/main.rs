use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use startup::{setup_map, spawn_camera};
use systems::{quit_application, toggle_simulation_state};

mod game;
mod startup;
mod systems;

mod ui;
use game::GameOfLifePlugin;
use ui::UIPlugin;

pub const WINDOW_WIDTH: f32 = 1300.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

pub const BOARD_SIZE: u32 = 100;
pub const BOARD_MAX_SIZE: u32 = 600;
pub const BOARD_MIN_SIZE: u32 = 20;

pub const SPRITE_SIZE: f32 = 8.0;

pub const CELL_PROBABILITY: f64 = 0.3;

pub const CYCLE_INTERVAL: f64 = 0.1;

// [catppuccin/catppuccin: 😸 Soothing pastel theme for the high-spirited!](https://github.com/catppuccin/catppuccin)
pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const BACKGROND_COLOR: Color = Color::hsl(237.0, 0.16, 0.23);
// pub const BACKGROND_COLOR: Color = Color::hsl(240.0, 0.21, 0.15);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

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
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(UIPlugin)
        .add_plugins(GameOfLifePlugin::default())
        .add_systems(Startup, (spawn_camera, setup_map))
        .add_systems(PostUpdate, (quit_application, toggle_simulation_state))
        .run();
}
