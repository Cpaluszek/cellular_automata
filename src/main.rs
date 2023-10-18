use bevy::prelude::*;
use std::time::Duration;
use systems::quit_application;
use ui::UIPlugin;

mod resources;
mod systems;
use game::GamePlugin;
use resources::{BoardSize, CellColor, CycleInterval, PatternFile, WindowSize};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub const CYCLE_INTERVAL: Duration = Duration::from_millis(100);
pub const BOARD_SIZE: (usize, usize) = (160, 90);

mod game;
mod ui;

fn main() {
    App::new()
        .add_state::<SimulationState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cellular Automata".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(WindowSize {
            resolution: Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        })
        .init_resource::<BoardSize>()
        .init_resource::<CycleInterval>()
        .init_resource::<PatternFile>()
        .insert_resource(CellColor(CELL_COLOR))
        .insert_resource(ClearColor(CLEAR_COLOR))
        // Events
        // Custom Plugins
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin {
            board_width: BOARD_SIZE.0,
            board_height: BOARD_SIZE.1,
        })
        // Systems
        .add_systems(Update, quit_application)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
