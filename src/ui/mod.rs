use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::{
    resources::UiState,
    systems::{
        interaction::handle_ui_interaction,
        layout::{init_ui_values, ui_panel},
    },
};

mod events;
mod resources;
mod systems;

use events::UiStateChangedEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_event::<UiStateChangedEvent>()
            .init_resource::<UiState>()
            .add_systems(Startup, init_ui_values)
            .add_systems(Update, (ui_panel, handle_ui_interaction.after(ui_panel)));
    }
}
