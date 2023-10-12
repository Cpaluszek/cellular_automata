use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.8, 1.0, 1.80);

pub const UI_MENU_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.9);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(120.0);
    style.height = Val::Px(40.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub const UI_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    // style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.width = Val::Percent(16.0);
    style.height = Val::Percent(90.0);
    style.left = Val::Percent(2.0);
    style.top = Val::Percent(2.0);
    style.padding = UiRect::all(Val::Px(12.0));
    style
};

pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size,
        color: Color::WHITE,
    }
}
