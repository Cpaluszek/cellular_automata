use bevy::prelude::*;

use crate::{game::SimulationState, ui::resources::UiSimulationState};

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

// pub fn handle_ui_interaction(
//     mut ui_event: EventReader<UiStateChangedEvent>,
//     simulation_state: Res<State<SimulationState>>,
//     mut commands: Commands,
//     mut board_size: ResMut<BoardSize>,
//     mut clear_color: ResMut<ClearColor>,
//     mut cell_color: ResMut<CellColor>,
//     mut cycle_interval: ResMut<CycleInterval>,
//     mut cycle_timer: ResMut<CycleTimer>,
//     mut pattern_file: ResMut<PatternFile>,
// ) {
//     for ev in ui_event.iter() {
//         match &ev.0 {
//             UiParameter::PauseSimulation => {
//                 if let SimulationState::Running = *simulation_state.get() {
//                     commands.insert_resource(NextState(Some(SimulationState::Paused)));
//                 } else {
//                     commands.insert_resource(NextState(Some(SimulationState::Running)));
//                 }
//             }
//             UiParameter::BoardSize(size) => {
//                 board_size.w = size.0;
//                 board_size.h = size.1;
//             }
//             UiParameter::CycleInterval(interval) => {
//                 cycle_interval.0 = *interval;
//                 cycle_timer
//                     .0
//                     .set_duration(Duration::from_millis(*interval as u64));
//             }
//             UiParameter::CellColor(color) => {
//                 cell_color.0 = Color::rgb(color[0], color[1], color[2]);
//             }
//             UiParameter::BackgroundColor(color) => {
//                 clear_color.0 = Color::rgb(color[0], color[1], color[2]);
//             }
//             UiParameter::LoadPatternFile(file) => {
//                 pattern_file.0 = file.clone();
//             }
//         }
//     }
// }
