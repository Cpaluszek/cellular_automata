use std::time::Duration;

use crate::game::resources::CycleTimer;
use crate::resources::{BoardSize, CellColor, CycleInterval};
use crate::ui::events::*;
use crate::SimulationState;
use bevy::prelude::*;

pub fn handle_ui_interaction(
    mut ui_event: EventReader<UiStateChangedEvent>,
    simulation_state: Res<State<SimulationState>>,
    mut commands: Commands,
    mut board_size: ResMut<BoardSize>,
    mut clear_color: ResMut<ClearColor>,
    mut cell_color: ResMut<CellColor>,
    mut cycle_interval: ResMut<CycleInterval>,
    mut cycle_timer: ResMut<CycleTimer>,
) {
    for ev in ui_event.iter() {
        match ev.0 {
            UiParameter::ResetSimulation => {
                // Todo: reset simulation
                info!("Reset simulation");
            }
            UiParameter::PauseSimulation => {
                if let SimulationState::Running = *simulation_state.get() {
                    commands.insert_resource(NextState(Some(SimulationState::Paused)));
                } else {
                    commands.insert_resource(NextState(Some(SimulationState::Running)));
                }
            }
            UiParameter::BoardWidth(width) => {
                board_size.w = width;
                // Todo: update board size
                info!("Board width: {}", width);
            }
            UiParameter::BoardHeight(height) => {
                board_size.h = height;
                info!("Board height: {}", height);
            }
            UiParameter::CycleInterval(interval) => {
                cycle_interval.0 = interval;
                cycle_timer
                    .0
                    .set_duration(Duration::from_millis(interval as u64));
            }
            UiParameter::CellColor(color) => {
                cell_color.0 = Color::rgb(color[0], color[1], color[2]);
            }
            UiParameter::BackgroundColor(color) => {
                clear_color.0 = Color::rgb(color[0], color[1], color[2]);
            }
        }
    }
}
