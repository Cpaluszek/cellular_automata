use bevy::prelude::*;

use crate::{
    ui::{
        components::*,
        styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
    SimulationState,
};

pub fn interact_with_pause_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseButton>),
    >,
    mut commands: Commands,
    simulation_state: Res<State<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                // Note: refacto with game system?
                match *simulation_state.get() {
                    SimulationState::Running => {
                        commands.insert_resource(NextState(Some(SimulationState::Paused)));
                    }
                    SimulationState::Paused => {
                        commands.insert_resource(NextState(Some(SimulationState::Running)));
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
