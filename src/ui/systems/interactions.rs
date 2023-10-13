use bevy::prelude::*;

// use crate::SimulationState;

// // use crate::{
// //     ui::{
// //         components::*,
// //         styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
// //     },
// //     SimulationState,
// // };

// // Note: merge with game syst3em?
// pub fn ui_toggle_simulation_state(
//     mut commands: Commands,
//     simulation_state: Res<State<SimulationState>>,
// ) {
//     match *simulation_state.get() {
//         SimulationState::Running => {
//             commands.insert_resource(NextState(Some(SimulationState::Paused)));
//         }
//         SimulationState::Paused => {
//             commands.insert_resource(NextState(Some(SimulationState::Running)));
//         }
//     }
// }
