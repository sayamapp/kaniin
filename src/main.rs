mod consts;
mod title;
mod score;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use score::ScorePlugin;
use crate::consts::*;
use crate::title::TitlePlugin;

// 後で移動
mod background;
mod player;
mod rock;
mod ufo;
mod rock_spawner;
mod bullet;
mod collision;
mod gameover;
use crate::background::BackgroundPlugin;
use crate::player::PlayerPlugin;
use crate::rock::RockPlugin;
use crate::ufo::UfoPlugin;
use crate::rock_spawner::RockSpawnerPlugin;
use crate::bullet::BulletPlugin;
use crate::collision::CollisionPlugin;
use crate::gameover::GameOverPlugin;


fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "kanirock".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(State::new(AppState::Title))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )

        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system()) // Cameras 
        .add_system(exit_on_esc_system.system())  

        // add my plugins
        .add_plugin(TitlePlugin)
        .add_plugin(BackgroundPlugin)

        .add_plugin(PlayerPlugin)
        .add_plugin(RockPlugin)
        .add_plugin(UfoPlugin)
        .add_plugin(RockSpawnerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(ScorePlugin)

        .run();
}

fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

