use bevy::prelude::*;

#[derive(Component)]
pub struct Cell;

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
        .add_systems(Startup, (spawn_camera, draw_cell))
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub const CELL_SIZE: f32 = 10.0;

pub fn draw_cell(mut commands: Commands) {
    let cell_sprite = Sprite {
        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
        ..default()
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: cell_sprite,
            ..default()
        },
        Cell {},
    ));
}
