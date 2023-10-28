use bevy::app::AppExit;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::prelude::*;

use crate::game::SimulationState;

pub fn quit_application(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_events_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_events_writer.send(AppExit);
    }
}

const ZOOM_SPEED: f32 = 1.;

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
    input_mouse: Res<Input<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {

    if input_mouse.pressed(MouseButton::Left) {
        let delta = ev_motion.iter().fold(Vec2::ZERO, |acc, e| acc + e.delta);
        println!("Drag : {}", delta);

        if delta.length_squared() > 0.0 {
            // Translate the camera
            let mut camera = camera_query.get_single_mut().unwrap();
            camera.translation.x += delta.x;
            camera.translation.y += delta.y;
        }
    }
}

pub fn toggle_simulation_state(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        } else if *simulation_state.get() == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        }
    }
}
