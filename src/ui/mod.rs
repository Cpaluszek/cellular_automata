use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::BOARD_SIZE;

use self::{
    resources::{UIBoardState, UiSimulationState},
    systems::{interaction::handle_pause_interaction, layout::ui_panel},
};

mod resources;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            // .add_event::<UiStateChangedEvent>()
            .insert_resource(UiSimulationState {
                simulation_state: true,
                // cycle_interval: CYCLE_INTERVAL.as_millis() as u32,
                // cell_color: [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()],
                // background_color: [CLEAR_COLOR.r(), CLEAR_COLOR.g(), CLEAR_COLOR.b()],
            })
            .insert_resource(UIBoardState {
                board_height: BOARD_SIZE.1 as u32,
                board_width: BOARD_SIZE.0 as u32,
            })
            .add_systems(Update, (ui_panel, handle_pause_interaction));
    }
}
