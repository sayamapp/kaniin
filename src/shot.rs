use bevy::prelude::*;
use crate::consts::*;
use crate::player::PlayerPosition;

use crate::Materials;
pub struct ShotPlugin;
impl Plugin for ShotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup_bullet.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, fire_burret.system())
            .on_state_exit(APP_STATE_STAGE, AppState::GameOver, despawn_bullet.system());
    }
}

pub struct Bullet(pub bool);

fn setup_bullet(
    commands: &mut Commands,
    materials: Res<Materials>,
) {
    
    commands
        .spawn(SpriteBundle {
            material: materials.shot_material.clone(), 
            transform: Transform {
                translation: Vec3::new(0.0, -900.0, 0.0),
                scale: Vec3::new(4.0, 4.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Bullet(false));
}

fn fire_burret(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Bullet)>,
    pos: Res<PlayerPosition>,
) {
    for (mut transform, mut bullet) in query.iter_mut() {
        if input.pressed(KeyCode::Space) {
            if !bullet.0 {
                bullet.0 = true;
                transform.translation.x = pos.0; 
                transform.translation.y = PLAYER_POSITION_Y;
            }
        }
        if bullet.0 {
            transform.translation.y += BULLET_SPEED;
        }

        if transform.translation.y > WINDOW_HEIGHT / 2.0 {
            bullet.0 = false;
            transform.translation.y = -900.0;
        }
    }
}

fn despawn_bullet(
    commands: &mut Commands,
    mut entities: Query<Entity, With<Bullet>>,
) {
    for entity in entities.iter_mut() {
        commands.despawn(entity);
    }
}