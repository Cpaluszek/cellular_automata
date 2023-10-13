use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::SimulationState;

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    simulation_state: Res<State<SimulationState>>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        if ui.button("Pause").clicked() {
            // Todo: use event
            match *simulation_state.get() {
                SimulationState::Running => {
                    commands.insert_resource(NextState(Some(SimulationState::Paused)));
                }
                SimulationState::Paused => {
                    commands.insert_resource(NextState(Some(SimulationState::Running)));
                }
            }
        }
    });
}

