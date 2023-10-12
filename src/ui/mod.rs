use bevy::prelude::*;

use self::systems::{interactions::interact_with_pause_button, layout::build_ui_menu};

mod components;
mod styles;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_ui_menu)
            .add_systems(Update, interact_with_pause_button);
    }
}
