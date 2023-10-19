use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource, Default)]
pub struct SimulationBatch;

#[derive(Debug, Resource)]
pub struct SimulationPause;
