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

// pub fn build_ui_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn(NodeBundle {
//             style: UI_MENU_STYLE,
//             background_color: UI_MENU_COLOR.into(),
//             ..default()
//         })
//         .with_children(|parent| {
//             // Pause button
//             parent
//                 .spawn((
//                     ButtonBundle {
//                         style: BUTTON_STYLE,
//                         background_color: NORMAL_BUTTON_COLOR.into(),
//                         ..default()
//                     },
//                     PauseButton {},
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle {
//                         text: Text {
//                             sections: vec![TextSection::new(
//                                 "Pause",
//                                 get_text_style(&asset_server, 24.0),
//                             )],
//                             alignment: TextAlignment::Center,
//                             ..default()
//                         },
//                         ..default()
//                     });
//                 });
//         });
// }
