use bevy::prelude::*;

use crate::{game::SimulationState, ui::resources::UiSimulationState};

// Todo: move to interactivity.rs
pub fn handle_pause_interaction(
    ui_state: Res<UiSimulationState>,
    simulation_state: Res<State<SimulationState>>,
    mut commands: Commands,
) {
    if ui_state.is_changed() {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        } else if *simulation_state.get() == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        }
    }
}
