use crate::consts::*;
use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FPSPlugin;
impl Plugin for FPSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(fps_setup.system())
            .add_system(fps_update.system());
    }
}
struct FpsText;

fn fps_setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load(FONT_PASS);
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(0.0),
                    bottom: Val::Percent(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "FPS".to_string(),
                font: font_handle.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::YELLOW,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}

fn fps_update(diagonostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagonostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average).into();
            }
        }
    }
}
