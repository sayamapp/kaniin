use bevy::prelude::*;
use kanirock::text_builder;
use crate::consts::*;

#[derive(Default)]
pub struct Score(pub usize);

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Score(0))
            .on_state_enter(APP_STATE_STAGE, AppState::Game, score_setup.system());
    }
}


fn score_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server.load(FONT_PASS);

    commands
        .spawn(text_builder(
            font_handle.clone(), 
            Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            "score", 
            Color::YELLOW,
            30.0));  
}