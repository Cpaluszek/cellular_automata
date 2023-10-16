use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        components::{CellPosition, CellState},
        resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, CycleTimer},
    },
    resources::CellColor,
};

pub fn get_next_generation(
    mut cycle_events: EventWriter<BoardCycleEvent>,
    mut board: ResMut<CellBoard>,
    mut cycle_timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    if !cycle_timer.0.tick(time.delta()).finished() {
        return;
    }
    let delta: Vec<_> = (0..board.height)
        // flat_map is used to flatten the results of the following closure.
        // For each row in the range generated in step 2, it creates another range from 0 to board.width (representing the columns),
        // and for each combination of row and col, it creates a CellPosition struct.
        // flat_map essentially flattens this 2D structure into a single iterator.
        .flat_map(|row| (0..board.width).map(move |col| CellPosition { col, row }))
        .filter_map(|pos| {
            let alive_neighbours_count: usize = board
                .neighbours(pos)
                .into_iter()
                .filter(|pos| board.alive(*pos))
                .count();

            let is_alive = board.alive(pos);
            let can_live = is_alive && (alive_neighbours_count == 2 || alive_neighbours_count == 3);
            let can_revive = !is_alive && alive_neighbours_count == 3;

            if (can_live || can_revive) && !is_alive {
                return Some((pos, CellState::Alive));
            }
            if !(can_live || can_live) && is_alive {
                return Some((pos, CellState::Dead));
            }
            None
        })
        .collect();

    delta
        .iter()
        .for_each(|(pos, state)| board.set(*pos, *state));
    cycle_events.send(BoardCycleEvent { delta });
}

pub fn apply_next_generation(
    mut commands: Commands,
    mut cycle_events: EventReader<BoardCycleEvent>,
    mut cell_entities: ResMut<CellEntityMap>,
    cell_color: Res<CellColor>,
    cell_size: Res<CellSize>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for evt in cycle_events.iter() {
        for (pos, state) in &evt.delta {
            let old_cell = match state {
                CellState::Dead => cell_entities.0.remove(pos),
                CellState::Alive => {
                    let x = -window.single().width() / 2.0
                        + (pos.col as f32 * cell_size.width)
                        + cell_size.width / 2.0;
                    let y = window.single().height() / 2.0
                        - (pos.row as f32 * cell_size.height)
                        - cell_size.height / 2.0;

                    // Cell entity
                    let new_cell = commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: cell_color.0,
                                    custom_size: Some(Vec2::new(cell_size.width, cell_size.height)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(x, y, 0.0),
                                ..default()
                            },
                            CellPosition {
                                col: pos.col,
                                row: pos.row,
                            },
                        ))
                        .id();
                    cell_entities.0.insert(*pos, new_cell)
                }
            };

            if let Some(entt) = old_cell {
                commands.entity(entt).despawn();
            }
        }
    }
}
