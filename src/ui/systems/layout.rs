use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{ui::resources::UiState, SimulationState};

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    simulation_state: Res<State<SimulationState>>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        let button_text = match *simulation_state.get() {
            SimulationState::Running => "Pause",
            SimulationState::Paused => "Resume",
        };
        ui.horizontal(|ui| {
            if ui.button(button_text).clicked() {
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
            if ui.button("Reset").clicked() {
                // Todo: use event
                info!("Reset simulation");
            }
        });
        // board size
        ui.label("Board size:");
        ui.add(egui::Slider::new(&mut ui_state.board_width, 40..=200).text("width"));
        ui.add(egui::Slider::new(&mut ui_state.board_heigth, 40..=200).text("height"));
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        // Simulation speed
        ui.separator();
        ui.label("Simulation speed:");
        ui.add(egui::Slider::new(&mut ui_state.cycle_interval, 40..=500).text("interval (ms)"));
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        // Cell color
        ui.separator();
        ui.label("Colors:");
        ui.horizontal(|ui| {
            ui.color_edit_button_rgb(&mut ui_state.cell_color);
            ui.label("Cell color");
        });
        ui.horizontal(|ui| {
            ui.color_edit_button_rgb(&mut ui_state.background_color);
            ui.label("Background color");
        });
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
    });
}
