use bevy::prelude::*;
use kanirock::text_builder;
use crate::consts::*;

#[derive(Default)]
pub struct Score(pub usize);

pub struct ScoreText;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Score(0))
            .on_state_enter(APP_STATE_STAGE, AppState::Game, score_setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, score_update.system())
            .on_state_enter(APP_STATE_STAGE, AppState::Title, score_despawn.system());
    }
}



fn score_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
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
            "score: 0", 
            Color::YELLOW,
            30.0))
            .with(ScoreText);  

}

fn score_update(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    let sc = score.0;
    for mut score_text in query.iter_mut() {
        score_text.value = format!("score: {:>5}", sc);
    }

}

fn score_despawn(
    commands: &mut Commands,
    mut score: ResMut<Score>,
    query: Query<(Entity, &ScoreText)>,
) {
    score.0 = 0;
    for (entity, _) in query.iter() {
        commands
            .despawn(entity);
    }
}