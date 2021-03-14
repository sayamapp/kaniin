use bevy::prelude::*;
use crate::consts::*;

struct Kaniin;

pub struct KaniinPlugin;
impl Plugin for KaniinPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, game_main.system());
    }
}

fn setup(
    commands: &mut Commands,
) {
}

fn game_main(
    commands: &mut Commands,
) {
}