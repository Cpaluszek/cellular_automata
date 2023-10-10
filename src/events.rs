use bevy::prelude::*;

#[derive(Event)]
pub struct SimulationStart;

#[derive(Event)]
pub struct SimulationStop;

#[derive(Event)]
pub struct ExitGame;
