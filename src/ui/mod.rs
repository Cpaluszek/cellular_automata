use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{BOARD_SIZE, CELL_COLOR};

use self::{
    resources::{UIBoardState, UiSimulationState, UICellColor},
    systems::{interaction::handle_pause_interaction, layout::ui_panel},
};

mod resources;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(UiSimulationState {
                simulation_state: true,
                // cycle_interval: CYCLE_INTERVAL.as_millis() as u32,
                // cell_color: [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()],
                // background_color: [CLEAR_COLOR.r(), CLEAR_COLOR.g(), CLEAR_COLOR.b()],
            })
            .insert_resource(UIBoardState {
                board_size: BOARD_SIZE,
            })
            .insert_resource(UICellColor {
                color: [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()]
            })
            .add_systems(Update, ui_panel)
            .add_systems(PostUpdate, handle_pause_interaction);
    }
}
