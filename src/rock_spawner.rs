use crate::consts::*;
use bevy::prelude::*;

use crate::ufo::UFO;
use std::time::Duration;

use rand::Rng;

use crate::rock::{Rock, RockSize};

pub struct RockSpawnerPlugin;
impl Plugin for RockSpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(APP_STATE_STAGE, AppState::Game, rock_spawner.system());
    }
}
pub struct RockSpawnTimer(Timer);
impl Default for RockSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(SPAWN_DURATION), true))
    }
}

fn rock_spawner(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: Local<RockSpawnTimer>,

    query: Query<&Transform, With<UFO>>,
) {
    if timer.0.tick(time.delta_seconds()).finished() {
        let texture_handle = asset_server.load(ROCK_TEXTURE);

        let mut pos = 0.0;
        for transform in query.iter() {
            pos = transform.translation.x;
        }

        let random = rand::thread_rng().gen_range(1, 100);

        if random < 50 {
            commands
                .spawn(SpriteBundle {
                    material: materials.add(texture_handle.clone().into()),
                    transform: Transform {
                        translation: Vec3::new(pos, ROCK_SPAWN_POSITION_Y, 0.0),
                        scale: Vec3::new(4.0, 4.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Rock {
                    size: RockSize::Midium,
                    velocity_x: 3.0,
                    velocity_y: 0.5,
                });
        } else {
            commands
                .spawn(SpriteBundle {
                    material: materials.add(texture_handle.clone().into()),
                    transform: Transform {
                        translation: Vec3::new(pos, ROCK_SPAWN_POSITION_Y, 0.0),
                        scale: Vec3::new(8.0, 8.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Rock {
                    size: RockSize::Large,
                    velocity_x: 3.0,
                    velocity_y: 0.5,
                });
        }
    }
}
