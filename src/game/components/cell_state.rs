use bevy::prelude::{Color, Component, Reflect};
use std::ops::{Deref, DerefMut};

use crate::CELL_COLOR;

use super::CellState;

// use super::CellState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component, Reflect)]
pub struct ConwayCellState(pub bool);

impl CellState for ConwayCellState {
    fn new_cell_state<'a>(&self, neighbours: impl Iterator<Item = &'a Self>) -> Self {
        let alive_neighbors_count = neighbours.filter(|cell| cell.0).count();
        match (self.0, alive_neighbors_count) {
            (true, 2) | (true, 3) => Self(true),
            (false, 3) => Self(true),
            _ => Self(false),
        }
    }

    fn color(&self) -> Option<Color> {
        if self.0 {
            // Todo: use custom color
            Some(CELL_COLOR)
        } else {
            None
        }
    }
}

// Todo: research
impl Deref for ConwayCellState {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConwayCellState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<bool> for ConwayCellState {
    fn from(b: bool) -> Self {
        Self(b)
    }
}
