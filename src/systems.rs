use bevy::app::AppExit;
use bevy::prelude::*;

pub fn quit_application(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_events_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_events_writer.send(AppExit);
    }
}
