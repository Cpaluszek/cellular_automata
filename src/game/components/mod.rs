use bevy::prelude::Component;
use std::{fmt::Debug, hash::Hash};

mod cell;
mod cell_state;
pub use cell::*;
pub use cell_state::*;

pub trait Cell: Clone + Component {
    type Coordinates: Clone + Debug + Send + Sync + Eq + Hash;

    #[must_use]
    fn coords(&self) -> &Self::Coordinates;

    #[must_use]
    fn neighbours_coordinates(&self) -> Vec<Self::Coordinates>;
}

pub trait CellState: Component + Sized + Clone + PartialEq {
    #[must_use]
    fn new_cell_state<'a>(&self, neighbours: impl Iterator<Item = &'a Self>) -> Self;

    #[must_use]
    fn color(&self) -> Option<bevy::render::color::Color>;
}
