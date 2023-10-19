use bevy::prelude::*;
use bevy_egui::EguiPlugin;

// use crate::{BOARD_SIZE, CELL_COLOR, CLEAR_COLOR, CYCLE_INTERVAL};

use self::{
    resources::UiState,
    systems::{interaction::handle_ui_interaction, layout::ui_panel},
};

// mod events;
mod resources;
mod systems;

// use events::UiStateChangedEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            // .add_event::<UiStateChangedEvent>()
            .insert_resource(UiState {
                simulation_state: true, // board_width: BOARD_SIZE.0 as u32,
                                        // board_height: BOARD_SIZE.1 as u32,
                                        // cycle_interval: CYCLE_INTERVAL.as_millis() as u32,
                                        // cell_color: [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()],
                                        // background_color: [CLEAR_COLOR.r(), CLEAR_COLOR.g(), CLEAR_COLOR.b()],
            })
            .add_systems(Update, (ui_panel, handle_ui_interaction));
    }
}
