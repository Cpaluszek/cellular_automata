use bevy::prelude::*;

mod cell;

use cell::CellPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cellular Automata".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        // Custom Plugins
        .add_plugins(CellPlugin)
        // Systems
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
