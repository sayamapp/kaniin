use std::ops::Neg;

use bevy::prelude::*;
use crate::consts::*;

pub struct RockPlugin;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup_debug.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, rock_move.system());
    }
}
pub enum RockSize {
    Large,
    Midium,
    Small,
}

pub struct Rock {
    size: RockSize,
    velocity_x: f32,
    velocity_y: f32,
}

fn setup_debug(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load(ROCK_TEXTURE);

    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                scale: Vec3::new(4.0, 4.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Rock{ size: RockSize::Midium, velocity_x: 3.0, velocity_y: 1.0});
}

fn rock_move(
    mut query: Query<(&mut Transform, &mut Rock)>,
) {
    for (mut transform, mut rock) in query.iter_mut() {


        // rock move x
        transform.translation.x += rock.velocity_x;

        if transform.translation.x >= WINDOW_WIDTH / 2.0 - DEBUG_ROCK_SIZE / 2.0 && rock.velocity_x.is_sign_positive() {
            rock.velocity_x = rock.velocity_x.neg();
        }
        if transform.translation.x <= -WINDOW_WIDTH / 2.0 + DEBUG_ROCK_SIZE / 2.0 && rock.velocity_x.is_sign_negative() {
            rock.velocity_x = rock.velocity_x.neg();
        }

        // rock move y
        transform.translation.y += rock.velocity_y;
        rock.velocity_y -= ROCK_GRAVITY;

        if transform.translation.y <= BACKGROUND_POSITION_Y + DEBUG_ROCK_SIZE && rock.velocity_y.is_sign_negative() {
            rock.velocity_y = rock.velocity_y.neg();
        }
    }
}

// use bevy::prelude::*;
// use crate::consts::*;
// use std::time::Duration;

// pub struct RockSpawnTimer(Timer);
// impl Default for RockSpawnTimer {
//     fn default() -> Self {
//         Self(Timer::new(Duration::from_millis(5000), true))
//     }
// }

// pub enum RockSize {
//     Large,
//     Midium,
//     Small,
// }

// pub struct Rock {
//     size: RockSize,
//     velocity_x: f32,
//     velocity_y: f32,
// }

// pub struct RockPlugin;
// impl Plugin for RockPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app
//             .on_state_enter(APP_STATE_STAGE, AppState::Game, setup.system())
//             .on_state_update(APP_STATE_STAGE, AppState::Game, rock_spawner.system())
//             .on_state_update(APP_STATE_STAGE, AppState::Game, rock_move.system());
//     }
// }

// fn setup (
//     commands: &mut Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let handle = asset_server.load("textures/rock2.png");
//     commands
//         .spawn(SpriteBundle {
//             material: materials.add(handle.into()),
//             transform: Transform {
//                 translation: Vec3::new(0., 200., 0.),
//                 scale: Vec3::new(4.0, 4.0, 1.),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Rock {size: RockSize::Midium, velocity_x: 1.0, velocity_y: 0.0});

// }

// fn rock_spawner(
//     commands: &mut Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     time: Res<Time>,
//     mut timer: Local<RockSpawnTimer>,
// ) {
//     if timer.0.tick(time.delta_seconds()).finished() {
//         let texture_handle = asset_server.load("textures/rock2.png");
//         commands
//             .spawn(SpriteBundle {
//                 material: materials.add(texture_handle.clone().into()),
//                 transform: Transform {
//                     translation: Vec3::new(0., 100., 0.),
//                     scale: Vec3::new(4., 4., 1.),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             })
//             .with(Rock{size: RockSize::Midium, velocity_x: 1.0, velocity_y: 0.0});
//     }
// }

// fn rock_move(
//     mut query: Query<(&mut Transform, &mut Rock)>,
// ) {
//     for (mut pos, mut rock) in query.iter_mut() {
//         let rock_size = match rock.size {
//             RockSize::Large => {ROCK_SIZE * 2.0}
//             RockSize::Midium => {ROCK_SIZE}
//             RockSize::Small => {ROCK_SIZE / 2.0}
//         };

//         pos.translation.x += rock.velocity_x;

//         if pos.translation.x >= WINDOW_WIDTH / 2.0 - rock_size * 2.0 && rock.velocity_x > 0. {
//             rock.velocity_x = -rock.velocity_x;
//         }
//         // if pos.translation.x >= 350.0 && rock.velocity_x > 0. {
//         //     rock.velocity_x = -rock.velocity_x;
//         // }
//         if pos.translation.x <= -WINDOW_WIDTH / 2.0 + rock_size * 2.0 && rock.velocity_x < 0. {
//             rock.velocity_x = -rock.velocity_x;
//         }
//         // if pos.translation.x <= -350.0 && rock.velocity_x < 0. {
//         //     rock.velocity_x = -rock.velocity_x;
//         // }

//         pos.translation.y += rock.velocity_y;
//         rock.velocity_y += -0.1;

//         if pos.translation.y <= FLOOR_POSITION_Y + rock_size * 2.0 && rock.velocity_y < 0. {
//             rock.velocity_y = -rock.velocity_y;
//         }
//     }
// }