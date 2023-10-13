use bevy::prelude::*;
use bevy_egui::EguiPlugin;

// use self::systems::interaction::handle_ui_interaction;
use self::systems::{interaction::handle_ui_interaction, layout::ui_panel};

mod events;
mod resources;
mod systems;

use events::UiStateChangedEvent;
use resources::UiState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_event::<UiStateChangedEvent>()
            .init_resource::<UiState>()
            .add_systems(Update, (ui_panel, handle_ui_interaction.after(ui_panel)));
    }
}
