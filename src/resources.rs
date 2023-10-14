use bevy::prelude::*;

#[derive(Resource)]
pub struct WindowSize {
    pub resolution: Vec2,
}

#[derive(Default, Resource, PartialEq)]
pub struct BoardSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Default, Resource, PartialEq)]
pub struct CycleInterval(pub u32);

#[derive(Default, Resource, PartialEq)]
pub struct CellColor(pub Color);
