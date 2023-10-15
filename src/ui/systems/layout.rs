use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    resources::CycleInterval,
    ui::{
        events::{UiParameter, UiStateChangedEvent},
        resources::UiState,
    },
    SimulationState,
};

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
pub fn ui_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    cycle_interval: Res<CycleInterval>,
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
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut ui_state.board_width, 16..=320).text("width"));
                ui.add(egui::Slider::new(&mut ui_state.board_height, 9..=180).text("height"));
            });
            ui.allocate_space(egui::Vec2::new(10.0, 0.0));
            if ui.button("Apply").clicked() {
                ui_event_writer.send(UiStateChangedEvent(UiParameter::BoardSize((
                    ui_state.board_width,
                    ui_state.board_height,
                ))));
            }
        });
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        // Simulation speed
        ui.separator();
        ui.label("Simulation speed:");
        ui.add(egui::Slider::new(&mut ui_state.cycle_interval, 40..=300).text("interval (ms)"));
        if ui_state.cycle_interval != cycle_interval.0 {
            ui_event_writer.send(UiStateChangedEvent(UiParameter::CycleInterval(
                ui_state.cycle_interval,
            )));
        }
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        // Cell color
        ui.separator();
        ui.label("Colors:");
        ui.horizontal(|ui| {
            if ui.color_edit_button_rgb(&mut ui_state.cell_color).changed() {
                ui_event_writer.send(UiStateChangedEvent(UiParameter::CellColor(
                    ui_state.cell_color,
                )));
            }
            ui.label("Cell color");
        });
        ui.horizontal(|ui| {
            if ui
                .color_edit_button_rgb(&mut ui_state.background_color)
                .changed()
            {
                ui_event_writer.send(UiStateChangedEvent(UiParameter::BackgroundColor(
                    ui_state.background_color,
                )));
            }
            ui.label("Background color");
        });
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
    });
}
