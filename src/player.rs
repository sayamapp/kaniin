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

pub struct Player {
    direction: Direction,
}

fn setup_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PLAYER_TEXTURE);
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::splat(PLAYER_TA_SIZE),
        PLAYER_TA_COLUMNS,
        PLAYER_TA_ROWS,
    );
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0., PLAYER_POSITION_Y, 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(PLAYER_ANIMATION_TIMER, true))
        .with(Player { direction: Direction::None });
}

fn player_move(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= PLAYER_MOVE_SPEED;
            player.direction = Direction::Left;
        } else if input.pressed(KeyCode::Right) {
            transform.translation.x += PLAYER_MOVE_SPEED;
            player.direction = Direction::Right;
        } else {
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