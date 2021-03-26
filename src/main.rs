mod consts;
mod title;
// mod title;
// mod kaniin;
// mod player;
// mod background;
// mod bubble;
// mod rock;
// mod collision;

// use bevy::prelude::*;
// use crate::consts::*;
// use crate::title::TitlePlugin;
// use crate::kaniin::KaniinPlugin;
// use crate::player::PlayerPlugin;
// use crate::background::BackgroundPlugin;
// use crate::bubble::BubblePlugin;
// use crate::rock::RockPlugin;
// use crate::collision::CollisionPlugin;
// use bevy::input::system::exit_on_esc_system;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use crate::consts::*;
use crate::title::TitlePlugin;

// 後で移動
mod background;
mod player;
use crate::background::BackgroundPlugin;
use crate::player::PlayerPlugin;

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

        .run();
}

fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}


// fn main() {
//     App::build()
//         .add_resource(WindowDescriptor {
//             title: "kaniin!".to_string(),
//             width: WINDOW_WIDTH,
//             height: WINDOW_HEIGHT,
//             ..Default::default()
//         })
//         .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
//         .add_resource(State::new(AppState::Title))
//         .add_stage_after(
//             stage::UPDATE,
//             APP_STATE_STAGE,
//             StateStage::<AppState>::default(),
//         )

//         .add_plugins(DefaultPlugins)
//         .add_plugin(KaniinPlugin)
//         .add_startup_system(setup.system())
//         .add_system(exit_on_esc_system.system())

//         .add_plugin(TitlePlugin)
//         .add_plugin(PlayerPlugin)
//         .add_plugin(BackgroundPlugin)
//         .add_plugin(BubblePlugin)
//         .add_plugin(RockPlugin)
//         .add_plugin(CollisionPlugin)
//         .run();
// }

// fn setup(
//     commands: &mut Commands,
// ) {
//     commands
//         .spawn(Camera2dBundle::default())
//         .spawn(CameraUiBundle::default());
// }