use bevy::prelude::*;

mod components;
pub mod resources;
mod systems;

use crate::{SimulationState, CYCLE_INTERVAL};

use self::{
    components::{CellPosition, CellState},
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, CycleTimer},
    systems::{
        apply_next_generation, change_cell_color, get_next_generation, life_setup,
        toggle_simulation_state,
    },
};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)

// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

pub struct GamePlugin {
    pub init_state: Option<(Vec<CellState>, (usize, usize))>,
    pub board_width: usize,
    pub board_height: usize,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let board = match &self.init_state {
            None => CellBoard::new_random(self.board_width, self.board_height),
            Some((state, (width, height))) => {
                let mut board = CellBoard::new(
                    self.board_width,
                    self.board_height,
                    vec![CellState::Alive; self.board_width * self.board_height],
                );
                let pos = CellPosition {
                    col: (self.board_width - width) / 2,
                    row: (self.board_height - height) / 2,
                };
                board.patch(pos, state, *width, *height);
                board
            }
        };

        app.add_event::<BoardCycleEvent>()
            .insert_resource(board)
            .insert_resource(CycleTimer(Timer::new(CYCLE_INTERVAL, TimerMode::Repeating)))
            .init_resource::<CellEntityMap>()
            .init_resource::<CellSize>()
            .add_systems(Startup, life_setup)
            .add_systems(
                Update,
                (
                    get_next_generation.run_if(in_state(SimulationState::Running)),
                    apply_next_generation.after(get_next_generation),
                ),
            )
            .add_systems(Update, change_cell_color)
            .add_systems(Update, toggle_simulation_state);
    }
}
