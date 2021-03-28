use crate::consts::*;
use bevy::prelude::*;

use crate::player::Player;
use crate::ufo::UFO;
use crate::rock::Rock;
pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::GameOver, gameover_setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::GameOver, game_retry.system());
    }
}

fn gameover_setup() {
    println!("GAMEOVER");
}

fn game_retry(
    commands: &mut Commands,
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Player>>,
    query2: Query<Entity, With<UFO>>,
    query3: Query<Entity, With<Rock>>,
) {
    if input.pressed(KeyCode::R) {
        for entity in query.iter() {
            commands.despawn(entity);
        }
        for enitiy in query2.iter() {
            commands.despawn(enitiy);
        }
        for entity in query3.iter() {
            commands.despawn(entity);
        }

        state.overwrite_next(AppState::Title);

    }
}
