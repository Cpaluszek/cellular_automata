use bevy::prelude::{Component, IVec2, Reflect};
use std::ops::Deref;

use super::Cell;

const NEIGHBOURS_COORDINATES: [IVec2; 8] = [
    IVec2::new(-1, 0),
    IVec2::new(-1, -1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(1, 0),
    IVec2::new(1, -1),
    IVec2::new(0, -1),
    IVec2::new(-1, 1),
];

#[derive(Debug, Clone, Component, Reflect)]
pub struct Moore2dCell {
    pub coords: IVec2,
}

impl Deref for Moore2dCell {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for Moore2dCell {
    type Coordinates = IVec2;

    // Todo: what is inline?
    #[inline]
    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    #[inline]
    fn neighbours_coordinates(&self) -> Vec<Self::Coordinates> {
        NEIGHBOURS_COORDINATES.map(|c| c + *self.coords()).to_vec()
    }
}

impl Moore2dCell {
    // must_use is a compiler hint that the return value of this function should
    // be used. If the return value is not used, the compiler will emit a warning.
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec2) -> Self {
        Self { coords }
    }
}
