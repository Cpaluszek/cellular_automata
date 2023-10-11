use std::time::Duration;

use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use self::{
    components::{CellPosition, CellState},
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, CycleTimer},
    systems::{apply_next_generation, get_next_generation, life_setup},
};

// Game of life patterns: [LifeWiki](https://conwaylife.com/wiki)

// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

// === Rules ===
// n for neighbours
// Liging cells:
//      n < 2 || n > 3 - DIES
//      n == 2 || n == 3 - LIVES
// Dead cells:
//      n == 3 - LIVES

// pub const GRID_SIZE: i32 = 100;

// pub const CELL_SIZE: f32 = 6.0;
// pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
// pub const CELL_SPRITE_SIZE: Vec2 = Vec2::new(CELL_SIZE, CELL_SIZE);
pub const CELL_COLOR: Color = Color::hsl(10.0, 0.56, 0.91);

pub struct GamePlugin {
    pub cycle_interval: Duration,
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
            .insert_resource(CycleTimer(Timer::new(
                self.cycle_interval,
                TimerMode::Repeating,
            )))
            .init_resource::<CellEntityMap>()
            .init_resource::<CellSize>()
            .add_systems(Startup, life_setup)
            .add_systems(
                Update,
                (
                    get_next_generation,
                    apply_next_generation.after(get_next_generation),
                ),
            );
    }
}
