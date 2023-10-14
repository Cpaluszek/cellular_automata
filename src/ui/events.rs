use bevy::prelude::*;

#[derive(Event)]
pub struct UiStateChangedEvent(pub UiParameter);

pub enum UiParameter {
    ResetSimulation,
    PauseSimulation,
    BoardWidth(u32),
    BoardHeight(u32),
    CycleInterval(u32),
    CellColor([f32; 3]),
    BackgroundColor([f32; 3]),
}
