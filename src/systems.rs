use bevy::app::AppExit;
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
