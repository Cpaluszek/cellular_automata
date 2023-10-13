use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct UiState {
    pub board_width: u32,
    pub board_height: u32,
    pub cycle_interval: u32,
    pub cell_color: [f32; 3],
    pub background_color: [f32; 3],
}

#[derive(Default, Resource)]
pub struct CellColorChanged {
    pub color: Color,
}
