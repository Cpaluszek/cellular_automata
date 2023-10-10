use std::fmt;

use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
}

pub enum CellState {
    Alive,
    Dead,
}

impl fmt::Debug for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Alive => write!(f, "Alive"),
            CellState::Dead => write!(f, "Dead"),
        }
    }
}
