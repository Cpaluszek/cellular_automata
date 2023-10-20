use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::{
    game::{BoardSize, SimulationState},
    ui::resources::{UIBoardState, UiSimulationState},
    BOARD_MAX_SIZE, BOARD_MIN_SIZE,
};

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
// [egui – An immediate mode GUI written in Rust](https://www.egui.rs/#Demo)
pub fn ui_panel(
    mut contexts: EguiContexts,
    mut ui_simulation_state: ResMut<UiSimulationState>,
    mut ui_board_state: ResMut<UIBoardState>,
    mut board_size: ResMut<BoardSize>,
    simulation_state: Res<State<SimulationState>>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        // Pause - Resume button
        let button_text = match *simulation_state.get() {
            SimulationState::Running => "Pause",
            SimulationState::Paused => "Resume",
        };
        ui.allocate_space(egui::Vec2::new(10.0, 0.0));
        if ui.button(button_text).clicked() {
            ui_simulation_state.simulation_state = !ui_simulation_state.simulation_state;
        }
        ui.allocate_space(egui::Vec2::new(0.0, 10.0));
        ui.separator();

        // Board size
        ui.label("Board size:");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(
                    &mut ui_board_state.board_size,
                    BOARD_MIN_SIZE..=BOARD_MAX_SIZE,
                ));
            });
            ui.allocate_space(egui::Vec2::new(10.0, 0.0));

            if ui.button("Apply").clicked() {
                board_size.size = ui_board_state.board_size;
            }
        });
    });
}
// ui.allocate_space(egui::Vec2::new(1.0, 10.0));
// // Simulation speed
// ui.label("Simulation speed:");
// ui.add(egui::Slider::new(&mut ui_state.cycle_interval, 20..=300).text("interval (ms)"));
// if ui_state.cycle_interval != cycle_interval.0 {
//     ui_event_writer.send(UiStateChangedEvent(UiParameter::CycleInterval(
//         ui_state.cycle_interval,
//     )));
// }
// ui.allocate_space(egui::Vec2::new(1.0, 10.0));
// // Cell color
// ui.separator();
// ui.label("Colors:");
// ui.horizontal(|ui| {
//     if ui.color_edit_button_rgb(&mut ui_state.cell_color).changed() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::CellColor(
//             ui_state.cell_color,
//         )));
//     }
//     ui.label("Cell color");
//     ui.allocate_space(egui::Vec2::new(10.0, 0.0));
//     if ui
//         .color_edit_button_rgb(&mut ui_state.background_color)
//         .changed()
//     {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::BackgroundColor(
//             ui_state.background_color,
//         )));
//     }
//     ui.label("Background color");
// });
// ui.allocate_space(egui::Vec2::new(1.0, 10.0));

// // Pattern files loading
// ui.separator();
// ui.label("RLE files:");
// ui.horizontal(|ui| {
//     ui.label("Lines");
//     if ui.button("Blinker").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/blinker.rle".to_string(),
//         )));
//     }
//     if ui.button("four").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/four.rle".to_string(),
//         )));
//     }
//     if ui.button("Five").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/five.rle".to_string(),
//         )));
//     }
//     if ui.button("ten").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/ten.rle".to_string(),
//         )));
//     }
// });
// ui.horizontal(|ui| {
//     ui.label("Stables");
//     if ui.button("cthulhu").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/cthulhu.rle".to_string(),
//         )));
//     }
//     if ui.button("beehiveplus").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/beehiveplus.rle".to_string(),
//         )));
//     }
//     if ui.button("stairs6").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/stairs6.rle".to_string(),
//         )));
//     }
// });
// ui.horizontal(|ui| {
//     ui.label("Infinites");
//     if ui.button("Max").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/max.rle".to_string(),
//         )));
//     }
//     if ui.button("10-cell inf.").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/10cellinfinitegrowth.rle".to_string(),
//         )));
//     }
// });
// ui.horizontal(|ui| {
//     ui.label("Gliders");
//     if ui.button("glider gun").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/gosperglidergun.rle".to_string(),
//         )));
//     }
//     if ui.button("60p5h2v0").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/60p5h2v0.rle".to_string(),
//         )));
//     }
//     if ui.button("puffer1").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/puffer1.rle".to_string(),
//         )));
//     }
//     if ui.button("canada goose").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/canadagoose.rle".to_string(),
//         )));
//     }
//     if ui.button("linepuffer").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/linepuffer.rle".to_string(),
//         )));
//     }
// });
// ui.horizontal(|ui| {
//     ui.label("Oscillators");
//     if ui.button("oscillos").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/oscillos.rle".to_string(),
//         )));
//     }
//     if ui.button("exploder").clicked() {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/exploder.rle".to_string(),
//         )));
//     }
// });
// ui.horizontal(|ui| {
//     ui.label("Big");
//     if ui
//         .button(RichText::new("breeder1").color(Color32::RED))
//         .clicked()
//     {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/breeder1.rle".to_string(),
//         )));
//     }
//     if ui
//         .button(RichText::new("otcametapixel").color(Color32::RED))
//         .clicked()
//     {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/otcametapixel.rle".to_string(),
//         )));
//     }
//     if ui
//         .button(RichText::new("otcametapixeloff").color(Color32::RED))
//         .clicked()
//     {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/otcametapixeloff.rle".to_string(),
//         )));
//     }
//     if ui
//         .button(RichText::new("p41660p5h2v0gun").color(Color32::RED))
//         .clicked()
//     {
//         ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
//             "assets/rle/p41660p5h2v0gun.rle".to_string(),
//         )));
//     }
// });
// ui.separator();
