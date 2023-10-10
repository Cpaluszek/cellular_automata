use bevy::prelude::*;

use crate::events::{ExitGame, SimulationStart, SimulationStop};

// Todo: update color to palette
const NORMAL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const PRESSED_COLOR: Color = Color::rgb(0.4, 0.8, 0.8);
const HOVERED_COLOR: Color = Color::rgb(0.8, 1.0, 1.80);

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
pub enum ButtonType {
    Start,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(Update, button_interaction);
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((build_classic_button(), ClassicButton(ButtonType::Start)))
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("PLAY", &asset_server));
                        });
                    parent
                        .spawn((build_classic_button(), ClassicButton(ButtonType::Stop)))
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("STOP", &asset_server));
                        });
                    parent
                        .spawn((build_classic_button(), ClassicButton(ButtonType::Exit)))
                        .with_children(|parent| {
                            parent.spawn(build_classic_text("EXIT", &asset_server));
                        });
                });
        });
}

fn build_classic_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_COLOR.into(),
        border_color: BorderColor(Color::BLACK),
        ..default()
    }
}

fn build_classic_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                value,
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ClassicButton),
        With<Button>,
    >,
    mut start_event_writer: EventWriter<SimulationStart>,
    mut stop_event_writer: EventWriter<SimulationStop>,
    mut exit_event_writer: EventWriter<ExitGame>,
) {
    for (interaction, mut color, classic_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_COLOR.into();
                match classic_button.0 {
                    ButtonType::Start => start_event_writer.send(SimulationStart),
                    ButtonType::Stop => stop_event_writer.send(SimulationStop),
                    ButtonType::Exit => exit_event_writer.send(ExitGame),
                }
            }
            Interaction::Hovered => *color = HOVERED_COLOR.into(),
            Interaction::None => *color = NORMAL_COLOR.into(),
        }
    }
}
