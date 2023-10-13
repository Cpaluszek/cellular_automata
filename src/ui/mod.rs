use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::systems::layout::ui_example_system;

mod resources;
mod systems;

use resources::UiState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<UiState>()
            .add_systems(Update, ui_example_system);
    }
}
