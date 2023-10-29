use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

mod components;
mod resources;
mod systems;

pub use components::*;
pub use resources::*;

use crate::{
    game::systems::{
        cells::{handle_cells, handle_new_cells, handle_removed_cells},
        coloring::color_sprites,
        interactivity::{handle_board_resize, handle_cell_color_change},
        input::{mouse_drag_event, scroll_events, handle_keyboard_input},
    },
    BOARD_SIZE, CELL_COLOR, CYCLE_INTERVAL,
};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)
// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

pub struct GameOfLifePlugin {
    pub tick_time_step: f64,
}

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        // Todo: use resource for duration -> UI
        let duration = Duration::from_secs_f64(self.tick_time_step);

        app.add_state::<SimulationState>()
            // Resources
            .insert_resource(BoardSize { size: BOARD_SIZE })
            .insert_resource(CellMap::<Moore2dCell>::default())
            .insert_resource(CellColor {
                color: [CELL_COLOR.r(), CELL_COLOR.r(), CELL_COLOR.r()].into(),
            })
            // Systems
            .add_systems(
                Update,
                (
                    handle_new_cells::<Moore2dCell>,
                    color_sprites::<ConwayCellState>,
                ),
            )
            .add_systems(
                Update,
                handle_cells::<Moore2dCell, ConwayCellState>
                    .run_if(on_timer(duration))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                PostUpdate,
                (
                    handle_removed_cells::<Moore2dCell>,
                    handle_cell_color_change,
                    handle_board_resize::<Moore2dCell>,
                    mouse_drag_event,
                    scroll_events,
                    handle_keyboard_input,
                ),
            );
        info!("Loaded cellular automata plugin");
    }
}

impl GameOfLifePlugin {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            tick_time_step: CYCLE_INTERVAL,
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
