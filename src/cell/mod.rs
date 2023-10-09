use bevy::{prelude::*, window::PrimaryWindow};

pub const CELL_SIZE: f32 = 10.0;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
pub const CELL_SPRITE_SIZE: Vec2 = Vec2::new(CELL_SIZE, CELL_SIZE);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

// Todo: create parent Cell container

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
        custom_size: Some(CELL_SPRITE_SIZE),
        color: CELL_COLOR,
        ..default()
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0 + HALF_CELL_SIZE, 0.0 + HALF_CELL_SIZE, 0.0),
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
            // Convert to world space position
            let (camera, camera_transform) = camera_query.single();
            let world_position = camera
                .viewport_to_world(camera_transform, cursor_position)
                .map(|ray| ray.origin.truncate());
            println!("World position: {:?}", world_position);

            // Spawn a new cell
            let mut cell_position = world_position.unwrap();
            cell_position.x = (cell_position.x / 10.0).round() * 10.0;
            cell_position.y = (cell_position.y / 10.0).round() * 10.0;
            println!("Cell position: {:?}", cell_position);
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        cell_position.x + HALF_CELL_SIZE,
                        cell_position.y + HALF_CELL_SIZE,
                        0.0,
                    ),
                    sprite: Sprite {
                        custom_size: Some(CELL_SPRITE_SIZE),
                        color: CELL_COLOR,
                        ..default()
                    },
                    ..default()
                },
                Cell {},
            ));
        }
    }
}
