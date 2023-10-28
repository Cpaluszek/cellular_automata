use bevy::prelude::*;

use crate::game::{resources::SimulationBatch, ConwayCellState};

#[inline]
fn apply_color(state: &ConwayCellState, visible: &mut Visibility) {
    if state.0 == true {
        if *visible != Visibility::Inherited {
            *visible = Visibility::Inherited;
        }
    } else {
        *visible = Visibility::Hidden;
    }
}

pub fn color_sprites<S>(
    mut query: Query<(&ConwayCellState, &mut Visibility), Changed<ConwayCellState>>,
    batch: Option<Res<SimulationBatch>>,
) {
    if batch.is_some() {
        query.par_iter_mut().for_each_mut(|(state, mut visible)| {
            apply_color(state, &mut visible);
        });
    } else {
        for (state, mut visible) in &mut query {
            apply_color(state, &mut visible);
        }
    }
}
