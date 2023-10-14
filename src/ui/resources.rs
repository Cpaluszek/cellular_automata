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
pub struct BoardSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Default, Resource, PartialEq)]
pub struct CycleInterval(pub u32);

#[derive(Default, Resource, PartialEq)]
pub struct CellColor(pub [f32; 3]);

#[derive(Default, Resource, PartialEq)]
pub struct BackgroundColor(pub [f32; 3]);
