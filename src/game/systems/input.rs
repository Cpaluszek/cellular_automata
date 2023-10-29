use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

const ZOOM_SPEED: f32 = 1.;
const MOVE_SPEED: f32 = 10.0;

pub fn scroll_events(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    // Todo: zoom to mouse pos -> https://github.com/bevyengine/bevy/issues/2580
    for event in scroll_event.iter() {
        for mut camera in camera_query.iter_mut() {
            let mut log_scale = camera.scale.ln();
            log_scale -= match event.unit {
                MouseScrollUnit::Line => event.y * ZOOM_SPEED * time.delta_seconds(),
                MouseScrollUnit::Pixel => event.y * ZOOM_SPEED * 0.1 * time.delta_seconds(),
            };
            camera.scale = log_scale.exp();
        }
    }
}

pub fn mouse_drag_event(
    // input_mouse: Res<Input<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
    // mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    // Todo: try without WSL

    // if input_mouse.pressed(MouseButton::Left) {
    for ev in ev_motion.iter() {
        println!("delta {}", ev.delta);
    }
    // let delta = ev_motion.iter().fold(Vec2::ZERO, |acc, e| acc + e.delta);
    // println!("Drag : {}", delta);
    //
    // if delta.length_squared() > 0.0 {
    //     // Translate the camera
    //     let mut camera = camera_query.get_single_mut().unwrap();
    //     camera.translation.x += delta.x;
    //     camera.translation.y += delta.y;
    // }
    // }
}

pub fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    let mut translation = Vec2::ZERO;

    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        translation.x += MOVE_SPEED;    
    }
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        translation.x -= MOVE_SPEED;    
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        translation.y -= MOVE_SPEED;    
    }
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        translation.y += MOVE_SPEED;    
    }

    if translation.length_squared() > 0.0 {
       camera_transform.translation.x += translation.x; 
       camera_transform.translation.y += translation.y; 
    }

}
