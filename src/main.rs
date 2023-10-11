use std::time::Duration;

use bevy::prelude::*;
use game::GamePlugin;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);

mod game;

#[derive(Resource)]
pub struct WindowSize {
    pub resolution: Vec2,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cellular Automata".into(),
                        resolution: (1024.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(WindowSize {
            resolution: Vec2::new(1024.0, 720.0),
        })
        // Events
        // Custom Plugins
        .add_plugins(GamePlugin {
            cycle_interval: Duration::from_millis(100),
            init_state: None,
            board_width: 100,
            board_height: 100,
        })
        // Systems
        .run();
}
