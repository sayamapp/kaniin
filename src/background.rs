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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bg_handle = asset_server.load(BACKGROUND_TEXTURE);

    commands
        .spawn(SpriteBundle {
            material: materials.add(bg_handle.into()),
            ..Default::default()
        })
        .with(Transform {
            translation: Vec3::new(0., BACKGROUND_POSITION_Y, 0.),
            scale: Vec3::new(100., SPRITE_SCALE, 1.),
            ..Default::default()
        });
}