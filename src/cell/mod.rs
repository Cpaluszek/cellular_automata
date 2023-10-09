use bevy::{prelude::*, window::PrimaryWindow};

pub const CELL_SIZE: f32 = 10.0;

#[derive(Component)]
pub struct Cell;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, place_initial_cell)
            .add_systems(Update, spawn_cell_with_mouse);
    }
}

pub fn place_initial_cell(mut commands: Commands) {
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

pub fn spawn_cell_with_mouse(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window_query.single().cursor_position() {
            println!("Cursor position: {:?}", cursor_position);
            // Convert to world space position

            let (camera, camera_transform) = camera_query.single();
            let world_position = camera
                .viewport_to_world(camera_transform, cursor_position)
                .map(|ray| ray.origin.truncate());
            println!("World position: {:?}", world_position);
        }
    }
}
