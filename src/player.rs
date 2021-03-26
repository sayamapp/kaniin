use crate::consts::*;
use bevy::{app::startup_stage::POST_STARTUP, prelude::*};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, player_setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, player_move.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, player_animation.system());
    }
}

pub enum Direction {
    None,
    Left,
    Right,
}
pub struct Player {
    direction: Direction,
}

#[derive(Default)]
pub struct PlayerPosition(pub f32);

fn player_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_TEXTURE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(PLAYER_TEXTURE_SIZE),
        PLAYER_TEXTURE_COLUMNS,
        PLAYER_TEXTURE_ROWS,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, PLAYER_POSITION_Y, 0.0),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(PLAYER_ANIMATION_TIMER, true))
        .with(Player {
            direction: Direction::None,
        });

    commands
        .insert_resource(PlayerPosition(0.0));
    
}

fn player_move(
    input: Res<Input<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut Player)>,
    mut pos: ResMut<PlayerPosition>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        if (input.pressed(KeyCode::Left) || input.pressed(KeyCode::A))
            && transform.translation.x > -WINDOW_WIDTH / 2.0 + (PLAYER_TEXTURE_SIZE * PLAYER_SCALE / 2.0)
        {
            // println!("{}", transform.translation.x);
            transform.translation.x -= PLAYER_MOVE_SPEED;
            player.direction = Direction::Left;
        } else if (input.pressed(KeyCode::Right) || input.pressed(KeyCode::D))
            && transform.translation.x < WINDOW_WIDTH / 2.0 - (PLAYER_TEXTURE_SIZE * PLAYER_SCALE / 2.0)
        {
            // println!("{}", transform.translation.x);
            transform.translation.x += PLAYER_MOVE_SPEED;
            player.direction = Direction::Right;
        } else {
            player.direction = Direction::None;
        }
        pos.0 = transform.translation.x;
    }
}

fn player_animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Player)>,
) {
    for (mut timer, mut sprite, player) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
           
            sprite.index = 
                match (&player.direction, sprite.index) {
                    (Direction::None, 0..=14) => {sprite.index + 1},
                    (Direction::None, _) => { 0 },
                    (Direction::Left, 16..=18) => {sprite.index + 1},
                    (Direction::Left, _) => { 16 }
                    (Direction::Right, 19..=22) => {sprite.index + 1},
                    (Direction::Right, _) => { 20 },
                };
        }
    }
}
