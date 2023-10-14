use crate::ui::events::*;
use crate::SimulationState;
use bevy::prelude::*;

pub fn handle_ui_interaction(
    mut ui_event: EventReader<UiStateChangedEvent>,
    simulation_state: Res<State<SimulationState>>,
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
) {
    for ev in ui_event.iter() {
        match ev.0 {
            UiParameter::ResetSimulation => {
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
                info!("Board width: {}", width);
            }
            UiParameter::BoardHeight(height) => {
                info!("Board height: {}", height);
            }
            UiParameter::CycleInterval(interval) => {
                info!("Cycle interval: {}", interval);
            }
            UiParameter::CellColor(color) => {
                info!("Cell color: {:?}", color);
            }
            UiParameter::BackgroundColor(color) => {
                clear_color.0 = Color::rgb(color[0], color[1], color[2]);
            }
        }
    }
}
