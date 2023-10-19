use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

mod components;
mod resources;
mod systems;

pub use components::*;
pub use resources::*;

use crate::{
    game::systems::cells::{handle_cells, handle_new_cells, handle_removed_cells},
    CYCLE_INTERVAL,
};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)
// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

pub struct GameOfLifePlugin {
    pub tick_time_step: Option<f64>,
}

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>();
        app.insert_resource(CellMap::<Moore2dCell>::default())
            .add_systems(Update, handle_new_cells::<Moore2dCell>)
            .add_systems(PostUpdate, handle_removed_cells::<Moore2dCell>);
        if let Some(time_step) = self.tick_time_step {
            let duration = Duration::from_secs_f64(time_step);
            app.add_systems(
                Update,
                handle_cells::<Moore2dCell, ConwayCellState>
                    .run_if(on_timer(duration))
                    .run_if(in_state(SimulationState::Running)),
            );
        } else {
            app.add_systems(
                Update,
                handle_cells::<Moore2dCell, ConwayCellState>
                    .run_if(in_state(SimulationState::Running)),
            );
        }
        app.add_systems(Update, systems::coloring::color_sprites::<ConwayCellState>);
        info!("Loaded cellular automata plugin");
    }
}

impl GameOfLifePlugin {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            tick_time_step: Some(CYCLE_INTERVAL),
        }
    }
}

impl Default for GameOfLifePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
