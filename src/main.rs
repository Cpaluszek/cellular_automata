use bevy::prelude::*;

pub const CLEAR_COLOR: Color = Color::hsl(240.0, 0.23, 0.09);
pub const GRID_SIZE: usize = 100;

mod cell;
mod events;
mod simulation;
mod ui;

use cell::CellPlugin;
use events::*;
use ui::MainMenuPlugin;

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
        // Events
        .add_event::<SimulationStart>()
        .add_event::<SimulationStop>()
        .add_event::<ExitGame>()
        // Custom Plugins
        .add_plugins((CellPlugin, MainMenuPlugin))
        // Systems
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
