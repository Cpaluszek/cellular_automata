use std::{fs::File, io::Read};

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        components::{CellPosition, CellState},
        resources::{CellBoard, CellEntityMap, CellSize},
    },
    resources::{BoardSize, CellColor, PatternFile},
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

pub fn load_pattern_file(
    mut commands: Commands,
    pattern_file: Res<PatternFile>,
    mut board: ResMut<CellBoard>,
    mut cell_entities: ResMut<CellEntityMap>,
    cell_color: Res<CellColor>,
    cell_size: Res<CellSize>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if !pattern_file.is_changed() || pattern_file.0.is_empty() {
        return;
    }
    info!("Load pattern file: {}", pattern_file.0);

    // Clear previous board
    board.clear();
    for cell_entt in cell_entities.0.values() {
        commands.entity(*cell_entt).despawn();
    }
    cell_entities.0.clear();

    // Read file content - see http://www.conwaylife.com/wiki/RLE
    let file_content = read_file_content(&pattern_file.0);
    let mut state = Vec::new();
    match file_content {
        Ok(content) => {
            let mut pattern_height = 0;
            let mut pattern_width = 0;
            let mut current_width = 0;
            let mut count = 0;
            // read file line by line
            for line in content.lines() {
                // if 1st char is '#' or 'x' skip
                if line.starts_with('#') || line.starts_with('x') {
                    // Todo: get real board size from file
                    continue;
                }
                info!("Line: {}", line);
                // iterate over line chars
                for c in line.chars() {
                    match c {
                        '0'..='9' => {
                            count = c.to_digit(10).unwrap();
                        }
                        'o' => {
                            for _ in 0..count {
                                state.push(CellState::Alive);
                                current_width += 1;
                            }
                            if count == 0 {
                                state.push(CellState::Alive);
                                current_width += 1;
                            }
                            count = 0;
                        }
                        'b' => {
                            for _ in 0..count {
                                state.push(CellState::Dead);
                                current_width += 1;
                            }
                            if count == 0 {
                                state.push(CellState::Dead);
                                current_width += 1;
                            }
                            count = 0;
                        }
                        '$' => {
                            pattern_height += 1;
                            if current_width > pattern_width {
                                pattern_width = current_width;
                            }
                            current_width = 0;
                        }
                        _ => {
                            pattern_height += 1;
                            if current_width > pattern_width {
                                pattern_width = current_width;
                            }
                            info!("Current width: {}", current_width);
                        }
                    }
                }
            }
            // Set the new state to the board
            let pos = CellPosition {
                col: (board.width - pattern_width) / 2,
                row: (board.height - pattern_height) / 2,
            };
            info!("New state from file: {:?}", state);
            info!("New state position: {:?}", pos);
            info!(
                "New state width: {} - height: {}",
                pattern_width, pattern_height
            );

            board.patch(pos, &state, pattern_width, pattern_height);

            // Spawn entities
            let half_window_height = window.single().height() / 2.0;
            let half_window_width = window.single().width() / 2.0;
            for row in 0..board.height {
                for col in 0..board.width {
                    let pos = CellPosition { col, row };
                    if board.alive(pos) {
                        let x = -half_window_width
                            + (col as f32 * cell_size.width)
                            + cell_size.width / 2.0;
                        let y = half_window_height
                            - (row as f32 * cell_size.height)
                            - cell_size.height / 2.0;

                        // Cell Entity
                        let new_cell = commands
                            .spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: cell_color.0,
                                        custom_size: Some(Vec2::new(
                                            cell_size.width,
                                            cell_size.height,
                                        )),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(x, y, 0.0),
                                    ..default()
                                },
                                CellPosition { col, row },
                            ))
                            .id();
                        cell_entities.0.insert(pos, new_cell);
                    }
                }
            }
        }
        Err(err) => {
            error!("Failed to read file: {}", err);
        }
    }
}

fn read_file_content(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
