use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        components::{CellPosition, CellState},
        resources::{CellBoard, CellEntityMap, CellSize},
    },
    resources::{BoardSize, CellColor},
    SimulationState,
};

pub fn change_cell_color(cell_color: Res<CellColor>, mut query: Query<&mut Sprite>) {
    if cell_color.is_changed() {
        for mut sprite in query.iter_mut() {
            sprite.color = cell_color.0;
        }
    }
}

// Todo: refacto function, extract state and board size outside of GamePlugin?
// use resource insteaad
pub fn handle_board_resize(
    board_size: Res<BoardSize>,
    mut cell_entities: ResMut<CellEntityMap>,
    mut board: ResMut<CellBoard>,
    mut commands: Commands,
) {
    if !board_size.is_changed() || board_size.w == 0 || board_size.h == 0 {
        return;
    }
    let new_board_state: Vec<_> = (0..board_size.h as usize)
        .flat_map(|row| (0..board_size.w as usize).map(move |col| CellPosition { col, row }))
        .filter_map(|pos| {
            // Remove if out of bounds
            if pos.col >= board_size.w as usize || pos.row >= board_size.h as usize {
                if let Some(entt) = cell_entities.0.remove(&pos) {
                    commands.entity(entt).despawn();
                }
                return None;
            }
            // if the board gets bigger, spawn new cells
            if pos.col >= board.width || pos.row >= board.height {
                return Some((pos, CellState::Dead));
            }
            let state = if board.alive(pos) {
                CellState::Alive
            } else {
                CellState::Dead
            };
            return Some((pos, state));
        })
        .collect();

    board.height = board_size.h as usize;
    board.width = board_size.w as usize;
    board.state = vec![CellState::Dead; board.width * board.height];
    new_board_state
        .iter()
        .for_each(|(pos, state)| board.set(*pos, *state));
}

pub fn update_cell_sprite_on_resize(
    mut cells: Query<(&mut Sprite, &mut Transform, &CellPosition)>,
    mut cell_size: ResMut<CellSize>,
    window: Query<&Window, With<PrimaryWindow>>,
    board: Res<CellBoard>,
    board_size: Res<BoardSize>,
) {
    if !board_size.is_changed() {
        return;
    }
    cell_size.width = window.single().width() / board.width as f32;
    cell_size.height = window.single().height() / board.height as f32;
    let half_window_width = window.single().width() / 2.0;
    let half_window_height = window.single().height() / 2.0;

    for (mut sprite, mut transform, pos) in cells.iter_mut() {
        sprite.custom_size = Some(Vec2::new(cell_size.width, cell_size.height));

        let x = -half_window_width + (pos.col as f32 * cell_size.width) + cell_size.width / 2.0;
        let y = half_window_height - (pos.row as f32 * cell_size.height) - cell_size.height / 2.0;

        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn toggle_simulation_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *simulation_state.get() {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
            }
        }
    }
}
