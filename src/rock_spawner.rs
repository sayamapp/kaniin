use crate::{consts::*, score};
use bevy::prelude::*;

use crate::ufo::UFO;
use std::time::Duration;

use rand::Rng;

use crate::rock::{Rock, RockSize};

pub struct RockSpawnerPlugin;
impl Plugin for RockSpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::Game, setup_debug.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, rock_spawner.system());
    }
}
pub struct RockSpawnTimer(Timer);
impl Default for RockSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(SPAWN_DURATION), true))
    }
}

pub struct DebugRockCount(usize);

pub fn setup_debug(commands: &mut Commands) {
    commands.insert_resource(DebugRockCount(1));
}

pub fn rock_spawn(
    commands: &mut Commands,
    materials: &Res<Materials>,
    size: RockSize,
    pos: Vec3,
    velocity_x: f32,
) {
    let scale = match size {
        RockSize::Large => 8.0,
        RockSize::Midium => 4.0,
        RockSize::Small => 2.0,
    };

    commands
        .spawn(SpriteBundle {
            material: materials.rock_material.clone(),
            transform: Transform {
                translation: pos,
                scale: Vec3::new(scale, scale, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Rock {
            size: size,
            velocity_x: velocity_x,
            velocity_y: 0.5,
        });
}

fn rock_spawner(
    commands: &mut Commands,
    time: Res<Time>,
    mut timer: Local<RockSpawnTimer>,
    query: Query<&Transform, With<UFO>>,
    materials: Res<Materials>,
    mut count: ResMut<DebugRockCount>,
) {
    // if timer.0.tick(time.delta_seconds()).finished() {
    let c = count.0;
    if time.seconds_since_startup() / 5.0 > c as f64 {
        count.0 = c + 1;
        let mut pos = 0.0;
        for transform in query.iter() {
            pos = transform.translation.x;
        }

        // let random_size = rand::thread_rng().gen_range(1, 100);
        // let random_velocity_x = rand::thread_rng().gen_range(1.0, 4.0);
        let random_size = 10;
        let random_velocity_x = 2.0;

        if c % 2 != 0 {
            rock_spawn(
                commands,
                &materials,
                RockSize::Midium,
                Vec3::new(pos, ROCK_SPAWN_POSITION_Y, 0.0),
                random_velocity_x,
            );
        } else {
            rock_spawn(
                commands,
                &materials,
                RockSize::Large,
                Vec3::new(pos, ROCK_SPAWN_POSITION_Y, 0.0),
                random_velocity_x,
            );
        }
    }
}
