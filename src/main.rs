use bevy::prelude::*;
use std::time::Duration;
use systems::quit_application;
use ui::UIPlugin;

mod resources;
mod systems;
use game::GamePlugin;
use resources::WindowSize;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub const CYCLE_INTERVAL: Duration = Duration::from_millis(50);
pub const BOARD_SIZE: (usize, usize) = (160, 90);

mod game;
mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
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
        // Events
        // Custom Plugins
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin {
            init_state: None,
            board_width: 160,
            board_height: 90,
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
