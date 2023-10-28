use crate::{
    game::{
        BoardBackground, BoardSize, Cell, CellContainer, CellMap, ConwayCellState, Moore2dCell,
    },
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
            return ;
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
            coords.iter().for_each(|c| match map.remove_cell(c.coords()) {
                Some(e) => commands.entity(e).despawn(),
                None => println!("Tried to despawn without entity"),
            });
        }
    }
}
