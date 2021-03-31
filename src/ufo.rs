use std::ops::Neg;

use bevy::prelude::*;
use crate::consts::*;

pub struct UfoPlugin;
impl Plugin for UfoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, ufo_setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, ufo_move.system());
    }
}

pub struct UFO {
    velocity_x: f32,
}

fn ufo_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load(UFO_TEXTURE);

    commands
        .spawn(SpriteBundle{
            material: materials.add(texture_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, UFO_POSITION_Y, 0.0),
                scale: Vec3::new(4.0, 4.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(UFO{ velocity_x: UFO_SPEED });
}

fn ufo_move(
    mut query: Query<(&mut Transform, &mut UFO)>,
) {
    for (mut transform, mut ufo) in query.iter_mut() {
        transform.translation.x += ufo.velocity_x;

        if transform.translation.x >= WINDOW_WIDTH / 2.0 - UFO_WIDTH / 2.0 && ufo.velocity_x.is_sign_positive() {
            ufo.velocity_x = ufo.velocity_x.neg();
        }
        if transform.translation.x <= -WINDOW_WIDTH / 2.0 + UFO_WIDTH / 2.0 && ufo.velocity_x.is_sign_negative() {
            ufo.velocity_x = ufo.velocity_x.neg();
        }
    }
}
