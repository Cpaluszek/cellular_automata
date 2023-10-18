use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPosition {
    pub col: usize,
    pub row: usize,
}
