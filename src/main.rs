mod consts;
mod title;
mod fps;
mod background;
mod player;
use bevy::prelude::*;
use crate::consts::*;

use crate::title::TitlePlugin;
use crate::fps::FPSPlugin;
use crate::background::BackgroundPlugin;
use crate::player::PlayerPlugin;
pub struct Materials {
    pub rock_material: Handle<ColorMaterial>,
    pub shot_material: Handle<ColorMaterial>,
    pub bg_material: Handle<ColorMaterial>,
}
#[derive(Clone)]
pub struct UIFont(Handle<Font>);

fn main() {
    let mut app = App::build();
    app.add_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
        .add_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_resource(ClearColor(Color::BLACK))
        .add_resource(State::new(AppState::Title))
        .add_stage_after(stage::UPDATE, APP_STATE_STAGE, StateStage::<AppState>::default());


    app.add_startup_system(setup.system())
        .add_plugin(TitlePlugin)
        .add_plugin(FPSPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
    
    let rock_texture_handle = asset_server.load(ROCK_TEXTURE);
    let shot_texture_handle = asset_server.load(TURBO_FISH_TEXTURE);
    let bg_texture_handle = asset_server.load(BACKGROUND_TEXTURE);
    let font_handle = asset_server.load(FONT_PASS);

    commands.insert_resource(Materials {
        rock_material: rock_texture_handle.into(),
        shot_material: shot_texture_handle.into(),
        bg_material: bg_texture_handle.into(),
    });

    commands.insert_resource(UIFont(font_handle.into()));
}
