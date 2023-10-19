use bevy::prelude::*;

mod components;
pub mod resources;
mod systems;

use crate::{SimulationState, CYCLE_INTERVAL};

use self::{
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, CycleTimer},
    systems::simulation::{apply_next_generation, get_next_generation},
};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)

// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

pub struct GamePlugin {
    pub board_width: usize,
    pub board_height: usize,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let board = CellBoard::new_random(self.board_width, self.board_height);

        app.add_event::<BoardCycleEvent>()
            .insert_resource(board)
            .insert_resource(CycleTimer(Timer::new(CYCLE_INTERVAL, TimerMode::Repeating)))
            .init_resource::<CellEntityMap>()
            .init_resource::<CellSize>()
            // Simulation Systems
            .add_systems(
                Update,
                (
                    get_next_generation.run_if(in_state(SimulationState::Running)),
                    apply_next_generation.after(get_next_generation),
                ),
            );
        // Interactivity Systems
        // .add_systems(
        //     Update,
        //     (
        //         change_cell_color,
        //         handle_board_resize,
        //         update_cell_sprite_on_resize,
        //         load_pattern_file,
        //     )
        //         .after(apply_next_generation),
        // )
        // .add_systems(Update, toggle_simulation_state);
    }
}
