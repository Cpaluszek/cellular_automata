use bevy::prelude::{Component, IVec2, Reflect};
use std::ops::Deref;

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

#[derive(Component)]
pub struct CellContainer;

#[derive(Component)]
pub struct BoardBackground;

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

impl Moore2dCell {
    // must_use is a compiler hint that the return value of this function should
    // be used. If the return value is not used, the compiler will emit a warning.
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec2) -> Self {
        Self { coords }
    }

    #[inline]
    pub fn coords(&self) -> &IVec2 {
        &self.coords
    }

    #[inline]
    pub fn neighbours_coordinates(&self) -> Vec<IVec2> {
        NEIGHBOURS_COORDINATES.map(|c| c + *self.coords()).to_vec()
    }
}

