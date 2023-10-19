use bevy::prelude::Component;
use std::{fmt::Debug, hash::Hash};

mod cell;
mod cell_state;
pub use cell::*;
pub use cell_state::*;

pub trait Cell: Clone + Component {
    // Associated coordinates type
    type Coordinates: Clone + Debug + Send + Sync + Eq + Hash;

    // Retrieves the cell coordinates
    #[must_use]
    fn coords(&self) -> &Self::Coordinates;

    // Retrieves the coordinates of the neighbor cells
    #[must_use]
    fn neighbours_coordinates(&self) -> Vec<Self::Coordinates>;
}

/// This trait defines the state of any given `Cell`. The trait implementation
/// will define the cellular automaton rules which will be automatically
/// applied.
///
/// Every type defining a `Cell` state and rules must implement this trait.
pub trait CellState: Component + Sized + Clone + PartialEq {
    // Todo: check lifetime info
    #[must_use]
    fn new_cell_state<'a>(&self, neighbours: impl Iterator<Item = &'a Self>) -> Self;

    #[must_use]
    fn color(&self) -> Option<bevy::render::color::Color>;
}
