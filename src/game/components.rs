use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPosition {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}
