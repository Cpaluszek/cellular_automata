use bevy::prelude::*;

#[derive(Default, Resource, PartialEq)]
pub struct UiState {
    pub board_width: u32,
    pub board_height: u32,
    pub cycle_interval: u32,
    pub cell_color: [f32; 3],
    pub background_color: [f32; 3],
}

#[derive(Default, Resource, PartialEq)]
pub struct BoardWidth(pub u32);

#[derive(Default, Resource, PartialEq)]
pub struct BoardHeight(pub u32);
