use bevy::prelude::*;

use crate::game::ConwayCellState;

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

pub fn color_sprites(
    mut query: Query<(&ConwayCellState, &mut Visibility), Changed<ConwayCellState>>,
) {
    query.par_iter_mut().for_each_mut(|(state, mut visible)| {
        apply_color(state, &mut visible);
    });
}
