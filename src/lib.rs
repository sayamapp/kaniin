use bevy::prelude::*;

pub fn text_builder(font_handle: Handle<Font>, style: Style, text: &str, color: Color, size: f32) -> TextBundle {
    TextBundle {
        style: style,
        text: Text {
            value: text.to_string(),
            font: font_handle,
            style: TextStyle {
                font_size: size,
                color: color,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}