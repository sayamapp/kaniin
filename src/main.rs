mod consts;
mod title;
mod kaniin;

use bevy::prelude::*;
use crate::consts::*;
use crate::title::TitlePlugin;
use crate::kaniin::KaniinPlugin;
use bevy::input::system::exit_on_esc_system;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "kaniin!".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Title))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )

        .add_plugins(DefaultPlugins)
        .add_plugin(KaniinPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())

        .add_plugin(TitlePlugin)
        .run();
}

fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}