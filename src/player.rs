use bevy::prelude::*;
use crate::consts::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup_player.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, player_move.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, animation.system());
    }
}

enum Direction {
    None,
    Left,
    Right,
}

struct Player {
    direction: Direction,
}
fn setup_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load("textures/kani.png");
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(16., 16.),
        24,
        1,
    );
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_texture_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05, true))
        .with(Transform{
            translation: Vec3::new(0., -200., 0.),
            scale: Vec3::new(4., 4., 1.),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05, true))
        .with(Player {direction: Direction::None });
}

fn player_move(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for ( mut transform , mut player) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 5.0;
            player.direction = Direction::Left;
        }
        else if input.pressed(KeyCode::Right) {
            transform.translation.x  += 5.0;
            player.direction = Direction::Right;
        }
        else {
            player.direction = Direction::None;
        }
    }
}

fn animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Player)>,
) {
    for (mut timer, mut sprite, player) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            match player.direction {
                Direction::None => {
                    sprite.index = if sprite.index >= 15 {
                        0
                    } else {
                        sprite.index + 1
                    };
                }
                Direction::Left => {
                    sprite.index = if sprite.index < 16 || sprite.index >= 19 {
                        16
                    } else {
                        sprite.index + 1
                    };
                }
                Direction::Right => {
                    sprite.index = if sprite.index < 20 || sprite.index >= 23 {
                        20
                    } else {
                        sprite.index + 1
                    };
                }
            };
        }
    }
}