use bevy::{prelude::*, utils::HashMap};

use crate::game::{resources::SimulationBatch, CellMap, SimulationPause, ConwayCellState, Moore2dCell};

fn handle_cell((cell, state): (&Moore2dCell, &ConwayCellState), map: &HashMap<IVec2, ConwayCellState>) -> Option<ConwayCellState>
{
    let neighbours_coords = cell.neighbours_coordinates();
    let neigbours_states = neighbours_coords.iter().filter_map(|c| map.get(c));
    let new_state = state.new_cell_state(neigbours_states);
    if &new_state == state {
        None
    } else {
        Some(new_state)
    }
}

pub fn handle_cells<C, S>(
    mut commands: Commands,
    par_commands: ParallelCommands,
    query: Query<(Entity, &Moore2dCell, &ConwayCellState)>,
    pause: Option<Res<SimulationPause>>,
    batch: Option<Res<SimulationBatch>>,
) {
    // Todo: remove batch and pause
    if pause.is_some() {
        println!("Pause");
        return;
    }
    let map: HashMap<_, _> = query
        .iter()
        .map(|(_entity, cell, state)| (cell.coords().clone(), state.clone()))
        .collect();

    if batch.is_some() {
        query.par_iter().for_each(|(entity, cell, state)| {
            if let Some(new_state) = handle_cell((cell, state), &map) {
                par_commands.command_scope(|mut cmd| {
                    cmd.entity(entity).insert(new_state);
                })
            }
        });
    } else {
        for (entity, cell, state) in query.iter() {
            if let Some(new_state) = handle_cell((cell, state), &map) {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}

pub fn handle_new_cells(query: Query<(Entity, &Moore2dCell), Added<Moore2dCell>>, mut map: ResMut<CellMap>)
{
    for (entity, new_cell) in query.iter() {
        let old_entity = map.insert_cell(new_cell.coords().clone(), entity);
        if let Some(e) = old_entity {
            if e != entity {
                warn!(
                    "{:?} replaced {:?} at {:?} coordinates",
                    entity,
                    e,
                    new_cell.coords()
                );
            }
        }
    }
}

pub fn handle_removed_cells(mut removed_cells: RemovedComponents<Moore2dCell>, mut map: ResMut<CellMap>)
{
    if removed_cells.is_empty() {
        return;
    }
    trace!("Removing {} cells from cell map", removed_cells.len());
    map.remove_entities(removed_cells.iter());
}
