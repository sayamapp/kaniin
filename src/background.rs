use bevy::prelude::*;
use crate::consts::*;

use crate::Materials;

struct Background;
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup_background.system())
            .on_state_exit(APP_STATE_STAGE, AppState::GameOver, despawn_background.system());
    }
}

fn setup_background(
    commands: &mut Commands,
    materials: Res<Materials>,
) {
    commands
        .spawn(SpriteBundle {
            material: materials.bg_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, BACKGROUND_POSITION_Y, 0.0),
                scale: Vec3::new(WINDOW_WIDTH / BACKGROUND_SCALE, BACKGROUND_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Background);
}

fn despawn_background(
    commands: &mut Commands,
    mut entities: Query<Entity, With<Background>>,
) {
    for entity in entities.iter_mut() {
        commands.despawn(entity);
    }
}