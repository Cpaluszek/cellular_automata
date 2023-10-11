use bevy::prelude::*;
use std::time::Duration;
use systems::quit_application;

mod resources;
mod systems;
use game::GamePlugin;
use resources::WindowSize;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

mod game;

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
        .add_plugins(GamePlugin {
            cycle_interval: Duration::from_millis(50),
            init_state: None,
            board_width: 100,
            board_height: 100,
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
