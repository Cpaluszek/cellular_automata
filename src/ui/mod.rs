use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{BOARD_SIZE, CELL_COLOR};

use self::{
    resources::{UIBoardState, UICellColor, UIPatternFile, UiSimulationState},
    systems::layout::ui_panel,
};

pub mod resources;
mod systems;

pub const SIDE_PANEL_WIDTH: f32 = 240.0;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(UiSimulationState {
                simulation_state: true,
                // cycle_interval: CYCLE_INTERVAL.as_millis() as u32,
            })
            .insert_resource(UIBoardState {
                board_size: BOARD_SIZE,
            })
            .insert_resource(UICellColor {
                color: [CELL_COLOR.r(), CELL_COLOR.g(), CELL_COLOR.b()],
            })
            .insert_resource(UIPatternFile {
                path: String::new(),
            })
            .add_systems(Update, ui_panel);
    }
}
