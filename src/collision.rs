use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::consts::*;

use crate::bullet::Bullet;
use crate::rock::Rock;
use crate::rock::RockSize;
use crate::ufo::UFO;
use crate::player::Player;
use crate::player::PlayerPosition;
use crate::score::Score;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_update(APP_STATE_STAGE, AppState::Game, bullet_collision.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, player_collision.system());
    }
}

fn bullet_collision(
    commands: &mut Commands,
    mut score: ResMut<Score>,
    mut query_bullet: Query<(&mut Transform, &mut Bullet)>,
    mut query_rock: Query<(Entity, &Transform, &Rock)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut bullet_transform, mut bullet) in query_bullet.iter_mut() {

        for (rock_entity, rock_transform, rock) in query_rock.iter_mut() {
            let collision = collide(
                bullet_transform.translation, 
                Vec2::new(64.0, 128.0), 
                rock_transform.translation, 
            Vec2::new(64.0, 64.0));

            if let Some(_) = collision {
                bullet.0 = false;
                bullet_transform.translation = Vec3::new(0.0, -900.0, 0.0);
                commands.despawn(rock_entity);
                score.0 += 100;

                let texture_handle = asset_server.load(ROCK_TEXTURE);
                match rock.size {
                    RockSize::Large => {
                        commands
                            .spawn(SpriteBundle {
                                material: materials.add(texture_handle.clone().into()),
                                transform: Transform {
                                    translation: Vec3::new(rock_transform.translation.x, rock_transform.translation.y, 0.0),
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

                            commands
                            .spawn(SpriteBundle {
                                material: materials.add(texture_handle.clone().into()),
                                transform: Transform {
                                    translation: Vec3::new(rock_transform.translation.x, rock_transform.translation.y, 0.0),
                                    scale: Vec3::new(4.0, 4.0, 1.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with(Rock {
                                size: RockSize::Midium,
                                velocity_x: -3.0,
                                velocity_y: 0.5,
                            });
                    }
                    RockSize::Midium => {
                        commands
                        .spawn(SpriteBundle {
                            material: materials.add(texture_handle.clone().into()),
                            transform: Transform {
                                translation: Vec3::new(rock_transform.translation.x, rock_transform.translation.y, 0.0),
                                scale: Vec3::new(2.0, 2.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(Rock {
                            size: RockSize::Small,
                            velocity_x: 3.0,
                            velocity_y: 0.5,
                        }); 

                        commands
                        .spawn(SpriteBundle {
                            material: materials.add(texture_handle.clone().into()),
                            transform: Transform {
                                translation: Vec3::new(rock_transform.translation.x, rock_transform.translation.y, 0.0),
                                scale: Vec3::new(2.0, 2.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(Rock {
                            size: RockSize::Small,
                            velocity_x: -3.0,
                            velocity_y: 0.5,
                        }); 
                    }
                    RockSize::Small => {}
                }


            }
        }
    }
}

fn player_collision(
    commands: &mut Commands,
    pos: Res<PlayerPosition>,
    mut state: ResMut<State<AppState>>,
    mut query: Query<&Transform, With<Rock>>,
) {
    for rock_transform in query.iter() {
        let collision = collide(
            rock_transform.translation, 
            Vec2::new(64.0, 64.0), 
            Vec3::new(pos.0, PLAYER_POSITION_Y, 0.0),
            Vec2::new(32.0, 20.0));

       if let Some(_) = collision {
            state.set_next(AppState::GameOver);
       } 
    }
}

// use bevy::prelude::*;
// use crate::consts::*;
// use crate::player::Player;
// use crate::rock::Rock;
// use bevy::sprite::collide_aabb::collide;

// pub struct CollisionPlugin;
// impl Plugin for CollisionPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app
//             .on_state_update(APP_STATE_STAGE, AppState::Game, test.system())
//             .on_state_update(APP_STATE_STAGE, AppState::Game, collision.system());
//     }
// }

// fn test (
//     query: Query<(&Transform, &Sprite), With<Player>>,
// ) {
//     for (transform, sprite) in query.iter() {
//         println!("{} {}", transform.translation.x, sprite.size);
//     }
// }

// fn collision(
//     query: Query<(&Transform, &Player)>,
//     query2: Query<(&Transform, &Rock)>,
// ) {
//     for (transform1,  _player) in query.iter() {
//         for (transform2, _rock) in query2.iter() {
//             let collision = collide(
//                 transform1.translation,
//                Vec2::new(64., 64.),
//                 transform2.translation,
//                 Vec2::new(100., 100.,)
//             );
//             if let Some(x) = collision {
//                 println!("COLLISION");
//             }
//         }
//     }
// }