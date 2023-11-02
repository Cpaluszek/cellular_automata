use std::{fs::File, io::Read};

use crate::{
    game::{
        BoardBackground, BoardSize, CellColor, CellContainer, CellMap, ConwayCellState,
        Moore2dCell, SimulationState,
    },
    ui::resources::{UIPatternFile, UiSimulationState},
    SPRITE_SIZE,
};
use bevy::prelude::*;

pub fn handle_pause_interaction(
    ui_state: Res<UiSimulationState>,
    simulation_state: Res<State<SimulationState>>,
    mut commands: Commands,
) {
    if ui_state.is_changed() {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        } else if *simulation_state.get() == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        }
    }
}

pub fn handle_board_resize(
    board_size: Res<BoardSize>,
    mut map: ResMut<CellMap>,
    cell_entities: Query<&Moore2dCell>,
    mut cell_container: Query<(Entity, &mut Transform), With<CellContainer>>,
    mut board_background: Query<&mut Sprite, With<BoardBackground>>,
    mut commands: Commands,
) {
    if !board_size.is_changed() {
        return;
    }

    let prev_board_size = (map.cell_count() as f64).sqrt() as u32;
    let delta_size = board_size.size as i32 - prev_board_size as i32;

    if delta_size == 0 {
        return;
    }

    // Set board background sprite
    if let Some(mut sprite) = board_background.get_single_mut().ok() {
        sprite.custom_size = Some(Vec2::new(
            board_size.size as f32 * SPRITE_SIZE,
            board_size.size as f32 * SPRITE_SIZE,
        ));
    }

    // Offset cell_container position
    let (parent_entity, mut parent_transform) = cell_container.get_single_mut().unwrap();
    let translation_offset = delta_size as f32 * SPRITE_SIZE * 0.5;
    parent_transform.translation.x -= translation_offset;
    parent_transform.translation.y -= translation_offset;

    if prev_board_size < board_size.size {
        let cell_sprite_size = Some(Vec2::splat(SPRITE_SIZE));

        let mut new_entities = vec![];
        for y in 0..board_size.size {
            for x in 0..board_size.size {
                if x < prev_board_size && y < prev_board_size {
                    continue;
                }
                let cell_pos = IVec2::new(x as i32, y as i32);
                let entity = commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: cell_sprite_size,
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            x as f32 * SPRITE_SIZE,
                            y as f32 * SPRITE_SIZE,
                            0.,
                        ),
                        ..default()
                    },
                    Moore2dCell::new(cell_pos),
                    ConwayCellState(false),
                ));
                new_entities.push(entity.id());
            }
        }
        commands.entity(parent_entity).push_children(&new_entities);
    } else {
        cell_entities
            .iter()
            .filter(|c| {
                let coords = c.coords();
                coords.x >= board_size.size as i32 || coords.y >= board_size.size as i32
            })
            .for_each(|c| match map.remove_cell(c.coords()) {
                Some(e) => commands.entity(e).despawn(),
                None => println!("Tried to despawn without entity"),
            });
    }
}

pub fn handle_cell_color_change(
    cell_color: Res<CellColor>,
    mut sprites: Query<&mut Sprite, With<ConwayCellState>>,
) {
    if cell_color.is_changed() {
        sprites
            .par_iter_mut()
            .for_each_mut(|mut sprite| sprite.color = cell_color.color);
    }
}

pub fn load_pattern_file(
    pattern_file: Res<UIPatternFile>,
    board_size: Res<BoardSize>,
    cell_query: Query<Entity, With<Moore2dCell>>,
    mut commands: Commands,
    map: Res<CellMap>,
) {
    if !pattern_file.is_changed() || pattern_file.path.is_empty() {
        return;
    }

    let content = match read_file_content(&pattern_file.path) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let (pattern_width, pattern_height, state) =
        parse_rle_content(&content, board_size.size as usize);

    // Clear previous board
    cell_query.iter().for_each(|entity| {
        commands.entity(entity).insert(ConwayCellState(false));
    });

    let pattern_start_x = (board_size.size - pattern_width as u32) / 2;
    let pattern_start_y = (board_size.size - pattern_height as u32) / 2;
    let pattern_end_x = pattern_start_x + pattern_width as u32;
    let pattern_end_y = pattern_start_y + pattern_height as u32;

    for y in pattern_start_y..pattern_end_y {
        let local_y = y - pattern_start_y;
        for x in pattern_start_x..pattern_end_x {
            let pos = IVec2::new(x as i32, y as i32);
            let local_x = x - pattern_start_x;
            let pattern_state = state[local_x as usize + local_y as usize * pattern_width];
            match map.get_cell(&pos) {
                Some(ent) => {
                    commands.entity(ent).insert(ConwayCellState(pattern_state));
                }
                None => {
                    println!("No cell at position {:?}", pos);
                }
            };
        }
    }
}

fn parse_rle_content(content: &str, board_size: usize) -> (usize, usize, Vec<bool>) {
    // Read file content - see http://www.conwaylife.com/wiki/RLE

    let mut state: Vec<bool> = vec![];
    let mut pattern_height = 0;
    let mut pattern_width = 0;
    let mut col = 0;
    let mut row = 0;
    let mut count = 0;

    // Parse a rle file
    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        } else if line.starts_with('x') {
            if let Some((width, height)) = parse_pattern_size(line) {
                if width > board_size || height > board_size {
                    println!("Pattern size exceed board size");
                    return (0, 0, vec![]);
                }
                pattern_width = width;
                pattern_height = height;
                state = vec![false; pattern_width * pattern_height];
            } else {
                println!("Invalid pattern size");
                return (0, 0, vec![]);
            }
        } else {
            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        count = count * 10 + c.to_digit(10).unwrap() as usize;
                    }
                    'o' => {
                        if count == 0 {
                            state[(pattern_height - row - 1) * pattern_width + col] = true;
                            col += 1;
                        } else {
                            for _ in 0..count {
                                state[(pattern_height - row - 1) * pattern_width + col] = true;
                                col += 1;
                            }
                            count = 0;
                        }
                    }
                    'b' => {
                        if count == 0 {
                            col += 1;
                        } else {
                            col += count as usize;
                            count = 0;
                        }
                    }
                    _ => {
                        row += 1;
                        col = 0;
                    }
                }
            }
        }
    }

    (pattern_width, pattern_height, state)
}

fn parse_pattern_size(line: &str) -> Option<(usize, usize)> {
    let mut split = line.split(',');
    let width_str = split.nth(0)?.split('=').last()?.trim();
    let height_str = split.nth(0)?.split('=').last()?.trim();
    let width = width_str.parse().ok()?;
    let height = height_str.parse().ok()?;
    Some((width, height))
}

fn read_file_content(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
