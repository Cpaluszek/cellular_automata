use bevy::prelude::*;

mod components;
mod systems;
use systems::*;

pub const CELL_SIZE: f32 = 10.0;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
pub const CELL_SPRITE_SIZE: Vec2 = Vec2::new(CELL_SIZE, CELL_SIZE);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub const ALIVE_CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_systems(Startup, spawn_cells);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
