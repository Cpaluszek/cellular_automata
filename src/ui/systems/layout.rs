use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    ui::{
        events::{UiParameter, UiStateChangedEvent},
        resources::{BackgroundColor, BoardHeight, BoardWidth, CellColor, CycleInterval, UiState},
    },
    SimulationState, BOARD_SIZE, CELL_COLOR, CLEAR_COLOR, CYCLE_INTERVAL,
};

pub fn init_ui_values(mut ui_state: ResMut<UiState>) {
    ui_state.board_width = BOARD_SIZE.0 as u32;
    ui_state.board_height = BOARD_SIZE.1 as u32;
    ui_state.cycle_interval = CYCLE_INTERVAL.as_millis() as u32;
    ui_state.cell_color = [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()];
    ui_state.background_color = [CLEAR_COLOR.r(), CLEAR_COLOR.g(), CLEAR_COLOR.b()];
}

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
pub fn ui_panel(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut board_width: ResMut<BoardWidth>,
    mut board_height: ResMut<BoardHeight>,
    mut cycle_interval: ResMut<CycleInterval>,
    mut cell_color: ResMut<CellColor>,
    mut background_color: ResMut<BackgroundColor>,
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
        if ui_state.cycle_interval != cycle_interval.0 {
            cycle_interval.0 = ui_state.cycle_interval;
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
                cell_color.0 = ui_state.cell_color;
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
                background_color.0 = ui_state.background_color;
                ui_event_writer.send(UiStateChangedEvent(UiParameter::BackgroundColor(
                    ui_state.background_color,
                )));
            }
            ui.label("Background color");
        });
        ui.allocate_space(egui::Vec2::new(1.0, 10.0));
    });
}
