use bevy::prelude::*;

#[derive(Default, Resource, PartialEq)]
pub struct UiSimulationState {
    pub simulation_state: bool,
}

#[derive(Default, Resource, PartialEq)]
pub struct UIBoardState {
    pub board_size: u32,
}

#[derive(Default, Resource)]
pub struct UICellColor {
    pub color: [f32; 3],
}

#[derive(Default, Resource)]
pub struct UIPatternFile {
    pub path: String,
}

// pub cycle_interval: u32,
// pub cell_color: [f32; 3],
// pub background_color: [f32; 3],
