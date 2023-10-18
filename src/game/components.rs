use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPosition {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    // Todo: use bool instead of enum
    Alive,
    Dead,
}
