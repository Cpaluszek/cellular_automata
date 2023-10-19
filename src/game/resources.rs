use bevy::{prelude::*, utils::HashMap};

use crate::WindowSize;

use super::components::CellPosition;

const DEFAULT_BOARD_SIZE: (usize, usize) = (40, 40);
const DEFAULT_BOARD_STATE: [bool; 40 * 40] = [false; 40 * 40];

#[derive(Resource, Default)]
pub struct CellEntityMap(pub HashMap<CellPosition, Entity>);

#[derive(Resource)]
pub struct CycleTimer(pub Timer);

#[derive(Event)]
pub struct BoardCycleEvent {
    pub delta: Vec<(CellPosition, bool)>,
}

#[derive(Resource)]
pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

impl FromWorld for CellSize {
    fn from_world(world: &mut World) -> Self {
        // get the window component from world
        let window = world.get_resource::<WindowSize>().unwrap();

        let board = world.get_resource::<CellBoard>().unwrap();
        Self {
            width: window.resolution.x / board.width as f32,
            height: window.resolution.y / board.height as f32,
        }
    }
}

#[derive(Resource, Clone)]
pub struct CellBoard {
    pub width: usize,
    pub height: usize,
    pub state: Vec<bool>,
}

impl Default for CellBoard {
    fn default() -> Self {
        let (state, (width, height)) = (Vec::from(DEFAULT_BOARD_STATE), DEFAULT_BOARD_SIZE);
        Self {
            width,
            height,
            state,
        }
    }
}

impl CellBoard {
    pub fn new_random(width: usize, height: usize) -> Self {
        let mut state = vec![false; width * height];
        for cell in state.iter_mut() {
            *cell = rand::random::<f32>() > 0.9;
        }
        Self {
            width,
            height,
            state,
        }
    }

    // pub fn clear(&mut self) {
    //     self.state = vec![false; self.width * self.height];
    // }

    // pub fn patch(
    //     &mut self,
    //     pos: CellPosition,
    //     patch: &[bool],
    //     patch_width: usize,
    //     patch_height: usize,
    // ) {
    //     assert!(pos.col < self.width, "Non existent column index");
    //     assert!(pos.row < self.height, "Non existent row index");

    //     assert!(
    //         pos.col + patch_width <= self.width,
    //         "Patch exceed board size"
    //     );
    //     assert!(
    //         pos.row + patch_height <= self.height,
    //         "Patch exceed board size"
    //     );

    //     for row_patch in 0..patch_height {
    //         for col_patch in 0..patch_width {
    //             let pos = CellPosition {
    //                 col: pos.col + col_patch,
    //                 row: pos.row + row_patch,
    //             };
    //             self.set(pos, patch[row_patch * patch_width + col_patch]);
    //         }
    //     }
    // }

    pub fn set(&mut self, pos: CellPosition, state: bool) {
        assert!(pos.col < self.width, "Non existent column index");
        assert!(pos.row < self.height, "Non existent row index");
        self.state[pos.row * self.width + pos.col] = state;
    }

    pub fn alive(&self, pos: CellPosition) -> bool {
        assert!(pos.col < self.width, "Non existent column index");
        assert!(pos.row < self.height, "Non existent row index");
        self.state[pos.row * self.width + pos.col]
    }

    pub fn neighbours(&self, pos: CellPosition) -> Vec<CellPosition> {
        assert!(pos.col < self.width, "Non existent column index");
        assert!(pos.row < self.height, "Non existent row index");

        let mut neighbours = vec![];
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let row_neighbour = pos.row as isize + row_offset;
                let row_neibgour = if row_neighbour < 0 || row_neighbour as usize >= self.height {
                    continue;
                } else {
                    row_neighbour as usize
                };

                let col_neighbour = pos.col as isize + col_offset;
                let col_neighbour = if col_neighbour < 0 || col_neighbour as usize >= self.width {
                    continue;
                } else {
                    col_neighbour as usize
                };

                neighbours.push(CellPosition {
                    col: col_neighbour,
                    row: row_neibgour,
                });
            }
        }
        neighbours
    }
}
