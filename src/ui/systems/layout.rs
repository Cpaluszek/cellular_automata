use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::{
    game::{BoardSize, CellColor, SimulationState},
    ui::{
        resources::{UIBoardState, UICellColor, UIPatternFile, UiSimulationState},
        SIDE_PANEL_WIDTH,
    },
    BOARD_MAX_SIZE, BOARD_MIN_SIZE,
};

// [bevy\_egui/examples/ui.rs at main · mvlabat/bevy\_egui](https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs)
// [egui – An immediate mode GUI written in Rust](https://www.egui.rs/#Demo)
pub fn ui_panel(
    mut contexts: EguiContexts,
    mut ui_simulation_state: ResMut<UiSimulationState>,
    mut ui_board_state: ResMut<UIBoardState>,
    mut board_size: ResMut<BoardSize>,
    mut cell_color: ResMut<CellColor>,
    mut ui_cell_color: ResMut<UICellColor>,
    mut ui_pattern_file: ResMut<UIPatternFile>,
    simulation_state: Res<State<SimulationState>>,
) {
    egui::SidePanel::left("Settings")
        .exact_width(SIDE_PANEL_WIDTH)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(0.0, 4.0));
            ui.heading("Settings");
            ui.separator();

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
            ui.label("Board size");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(
                        &mut ui_board_state.board_size,
                        BOARD_MIN_SIZE..=BOARD_MAX_SIZE,
                    ));
                });
                ui.allocate_space(egui::Vec2::new(10.0, 0.0));

                if ui.button("Apply").clicked() {
                    if board_size.size != ui_board_state.board_size {
                        board_size.size = ui_board_state.board_size;
                    }
                }
            });

            // Cell color
            ui.allocate_space(egui::Vec2::new(0.0, 10.0));
            ui.separator();
            ui.label("Colors");
            ui.horizontal(|ui| {
                if ui.color_edit_button_rgb(&mut ui_cell_color.color).changed() {
                    cell_color.color = ui_cell_color.color.into();
                }
                ui.label("Cell color");
            });
            // Pattern files
            ui.allocate_space(egui::Vec2::new(0.0, 10.0));
            ui.separator();
            ui.label("RLE Files");
            ui.horizontal(|ui| {
                ui.label("Lines");
                if ui.button("Blinker").clicked() {
                    ui_pattern_file.path = "assets/rle/blinker.rle".to_string();
                }
                if ui.button("four").clicked() {
                    // ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
                    //             "assets/rle/four.rle".to_string(),
                    //             )));
                }
                if ui.button("Five").clicked() {
                    // ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
                    //             "assets/rle/five.rle".to_string(),
                    //             )));
                }
                if ui.button("ten").clicked() {
                    // ui_event_writer.send(UiStateChangedEvent(UiParameter::LoadPatternFile(
                    //             "assets/rle/ten.rle".to_string(),
                    //             )));
                }
            });
        });
}
// // Pattern files loading
// ui.separator();
// ui.label("RLE files:");
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
