use std::{fs::File, io::Read};

use crate::{
    game::{
        BoardBackground, BoardSize, Cell, CellColor, CellContainer, CellMap, ConwayCellState,
        Moore2dCell,
    },
    ui::resources::UIPatternFile,
    SPRITE_SIZE,
};
use bevy::prelude::*;

pub fn handle_board_resize<C>(
    board_size: Res<BoardSize>,
    mut map: ResMut<CellMap<C>>,
    cell_entities: Query<&C>,
    mut cell_container: Query<(Entity, &mut Transform), With<CellContainer>>,
    mut board_background: Query<&mut Sprite, With<BoardBackground>>,
    mut commands: Commands,
) where
    C: Cell,
{
    if board_size.is_changed() {
        let prev_board_size = (map.cell_count() as f64).sqrt() as u32;
        let delta_size = board_size.size as i32 - prev_board_size as i32;

        if delta_size == 0 {
            return;
        }

        // Set board background sprite
        let mut sprite = board_background.get_single_mut().unwrap();
        sprite.custom_size = Some(Vec2::new(
            board_size.size as f32 * SPRITE_SIZE,
            board_size.size as f32 * SPRITE_SIZE,
        ));

        // Offset cell_container position
        let (parent_entity, mut parent_transform) = cell_container.get_single_mut().unwrap();
        let translation_offset = delta_size as f32 * SPRITE_SIZE * 0.5;
        parent_transform.translation.x -= translation_offset;
        parent_transform.translation.y -= translation_offset;

        if prev_board_size < board_size.size {
            // Increase board size
            let mut new_entities = vec![];
            for y in 0..board_size.size {
                for x in 0..board_size.size {
                    if x < prev_board_size && y < prev_board_size {
                        continue;
                    }
                    let entity = commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(SPRITE_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                x as f32 * SPRITE_SIZE,
                                y as f32 * SPRITE_SIZE,
                                0.,
                            ),
                            ..default()
                        },
                        Moore2dCell::new(IVec2::new(x as i32, y as i32)),
                        ConwayCellState(false),
                    ));
                    new_entities.push(entity.id());
                }
            }
            commands.entity(parent_entity).push_children(&new_entities);
        } else {
            let coords: Vec<_> = cell_entities
                .iter()
                .filter(|c| {
                    c.get_x() >= board_size.size as i32 || c.get_y() >= board_size.size as i32
                })
                .collect();
            coords
                .iter()
                .for_each(|c| match map.remove_cell(c.coords()) {
                    Some(e) => commands.entity(e).despawn(),
                    None => println!("Tried to despawn without entity"),
                });
        }
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

pub fn load_pattern_file(pattern_file: Res<UIPatternFile>,
    board_size: Res<BoardSize>,
                         ) {
    if !pattern_file.is_changed() || pattern_file.path.is_empty() {
        return;
    }
    println!("Pattern: {}", pattern_file.path);

    // Read file content - see http://www.conwaylife.com/wiki/RLE
    let content = read_file_content(&pattern_file.path).unwrap();
    let mut state: Vec<bool> = vec![];

    let mut pattern_height = 0;
    let mut pattern_width = 0;
    let mut col = 0;
    let mut row = 0;
    let mut count = 0;

    // Todo: clean parsing

    // Parse a rle file
    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        } else if line.starts_with('x') {
            let mut split = line.split(',');
            pattern_width = split
                .next()
                .unwrap()
                .split('=')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            pattern_height = split
                .next()
                .unwrap()
                .split('=')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            if pattern_width <= 0 || pattern_height <= 0 {
                println!("Invalid pattern size");
                return ;
            }

            if pattern_width as u32 > board_size.size || pattern_height as u32 > board_size.size  {
                println!("Pattern size exceed board size");
                return ;
            }

            // Todo: use 2D container - vec of vec?
            state = vec![false; pattern_width * pattern_height];
        } else {
            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        count = count * 10 + c.to_digit(10).unwrap();
                    }
                    'o' => {
                        if count == 0 {
                            state[row * pattern_width + col] = true;
                            col += 1;
                        } else {
                            for _ in 0..count {
                                state[row * pattern_width + col] = true;
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

    // Todo: Clear previous board

    let pattern_x = (board_size.size - pattern_width as u32) / 2;
    let pattern_y = (board_size.size - pattern_height as u32) / 2;
    for x in 0..board_size.size {
        for y in 0..board_size.size {
            if x > pattern_x && x < pattern_x + pattern_width as u32 &&
                y > pattern_y && y < pattern_y + pattern_height as u32 {
                    // look for pattern
                }
            else {
                // Set cell to dead
            }
        }
    }


    // query: Query<(Entity, &C, &S)>,

    // board.clear();
    // for cell_entt in cell_entities.0.values() {
    //     commands.entity(*cell_entt).despawn();
    // }
    // cell_entities.0.clear();

    // Set the new state to the board
    // let pos = CellPosition {
    //     col: (board.width - pattern_width) / 2,
    //     row: (board.height - pattern_height) / 2,
    // };
    //
    // board.patch(pos, &state, pattern_width, pattern_height);

    // Spawn entities
    // let half_window_height = window.single().height() / 2.0;
    // let half_window_width = window.single().width() / 2.0;
    // for row in 0..board.height {
    //     for col in 0..board.width {
    //         let pos = CellPosition { col, row };
    //         if board.alive(pos) {
    //             let x = -half_window_width + (col as f32 * cell_size.width) + cell_size.width / 2.0;
    //             let y =
    //                 half_window_height - (row as f32 * cell_size.height) - cell_size.height / 2.0;
    //
    //             // Cell Entity
    //             let new_cell = commands
    //                 .spawn((
    //                     SpriteBundle {
    //                         sprite: Sprite {
    //                             color: cell_color.0,
    //                             custom_size: Some(Vec2::new(cell_size.width, cell_size.height)),
    //                             ..default()
    //                         },
    //                         transform: Transform::from_xyz(x, y, 0.0),
    //                         ..default()
    //                     },
    //                     CellPosition { col, row },
    //                 ))
    //                 .id();
    //             cell_entities.0.insert(pos, new_cell);
    //         }
    //     }
    // }
}

fn read_file_content(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

