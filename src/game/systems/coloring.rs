use bevy::prelude::*;

use crate::game::{resources::SimulationBatch, CellState, ConwayCellState};

#[inline]
fn apply_color<S>(state: &S, visible: &mut Visibility, sprite: &mut Sprite)
where
    S: CellState,
{
    match state.color() {
        Some(c) => {
            sprite.color = c;
            if *visible != Visibility::Inherited {
                *visible = Visibility::Inherited;
            }
        }
        None => *visible = Visibility::Hidden,
    }
}

// Todo: ???
#[allow(clippy::needless_pass_by_value)]
pub fn color_sprites<S>(
    mut query: Query<(&ConwayCellState, &mut Visibility, &mut Sprite), Changed<ConwayCellState>>,
    batch: Option<Res<SimulationBatch>>,
) {
    if batch.is_some() {
        query
            .par_iter_mut()
            .for_each_mut(|(state, mut visible, mut sprite)| {
                apply_color(state, &mut visible, &mut sprite);
            });
    } else {
        for (state, mut visible, mut sprite) in &mut query {
            apply_color(state, &mut visible, &mut sprite);
        }
    }
}
