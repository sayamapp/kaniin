use bevy::prelude::*;
use crate::consts::*;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup.system());
    }
}

fn setup(
    commands: &mut Commands,

) {
    for i in -10..=10 {

    }
}