use bevy::prelude::*;
use crate::consts::*;

use crate::player::Player;
use crate::player::PlayerPosition;
pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_bullet.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, fire_burret.system());
    }
}

pub struct Bullet(pub bool);

fn setup_bullet(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load(TURBO_FISH_TEXTURE);
    
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
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
    mut pos: Res<PlayerPosition>,
    player: Query<&Transform, With<Player>>,
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