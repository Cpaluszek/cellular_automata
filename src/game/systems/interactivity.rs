use crate::{game::{BoardSize, Cell, CellContainer, CellMap, Moore2dCell, ConwayCellState, BoardBackground}, SPRITE_SIZE};
use bevy::prelude::*;

pub fn handle_board_resize<C>(
    board_size: Res<BoardSize>,
    map: Res<CellMap<C>>,
    mut cell_container: Query<(Entity, &mut Transform), With<CellContainer>>,
    mut board_background: Query<&mut Sprite, With<BoardBackground>>,
    mut commands: Commands,
    ) where
C: Cell,
{
    if board_size.is_changed() {
        let entities_count = map.cell_count();
        let prev_board_size = (entities_count as f64).sqrt() as u32;

        // Set board background sprite
        let mut sprite = board_background.get_single_mut().unwrap();
        sprite.custom_size = Some(Vec2::new(
                board_size.size as f32 * SPRITE_SIZE,
                board_size.size as f32 * SPRITE_SIZE,
                ));

        let (parent_entity, mut parent_transform) = cell_container.get_single_mut().unwrap();
        let delta_size = board_size.size as i32 - prev_board_size as i32;
        println!("Delta size: {}", delta_size);
        parent_transform.translation.x -= delta_size as f32 * SPRITE_SIZE * 0.5;
        parent_transform.translation.y -= delta_size as f32 * SPRITE_SIZE * 0.5;

        println!("Board size changed to: {}", board_size.size);
        println!("Previous entities count: {}", entities_count);
        println!("Previous board size: {}", prev_board_size);
        if prev_board_size < board_size.size {
            let mut new_entities = vec![];
            for y in 0..board_size.size {
                for x in 0..board_size.size {
                    if x < prev_board_size && y < prev_board_size {
                        continue ;
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
            println!("Added {} entities", new_entities.len());
        } 
        // else {
        //     for y in (prev_board_size..board_size.size).rev() {
        //         for x in (prev_board_size..board_size.size).rev() {
        //             // Todo: remove cells
        //         }
        //     }
        // }
        // Todo:  resize background
    }
}
