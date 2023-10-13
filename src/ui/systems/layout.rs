use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    ui::{
        events::{UiParameter, UiStateChangedEvent},
        resources::{BoardHeight, BoardWidth, UiState},
    },
    SimulationState,
};

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
pub fn ui_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut board_width: ResMut<BoardWidth>,
    mut board_height: ResMut<BoardHeight>,
    simulation_state: Res<State<SimulationState>>,
    mut ui_event_writer: EventWriter<UiStateChangedEvent>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        let button_text = match *simulation_state.get() {
            SimulationState::Running => "Pause",
            SimulationState::Paused => "Resume",
        };
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                ui_event_writer.send(UiStateChangedEvent(UiParameter::ResetSimulation));
            }
            ui.allocate_space(egui::Vec2::new(10.0, 0.0));
            if ui.button(button_text).clicked() {
                ui_event_writer.send(UiStateChangedEvent(UiParameter::PauseSimulation));
            }
        });
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        // board size
        ui.separator();
        ui.label("Board size:");
        ui.add(egui::Slider::new(&mut ui_state.board_width, 40..=200).text("width"));
        if ui_state.board_width != board_width.0 {
            board_width.0 = ui_state.board_width;
            ui_event_writer.send(UiStateChangedEvent(UiParameter::BoardWidth(
                ui_state.board_width,
            )));
        }
        ui.add(egui::Slider::new(&mut ui_state.board_height, 40..=200).text("height"));
        if ui_state.board_height != board_height.0 {
            board_height.0 = ui_state.board_height;
            ui_event_writer.send(UiStateChangedEvent(UiParameter::BoardHeight(
                ui_state.board_height,
            )));
        }
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
