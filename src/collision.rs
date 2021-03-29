use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::consts::*;

use crate::bullet::Bullet;
use crate::rock::Rock;
use crate::rock::RockSize;
use crate::ufo::UFO;
use crate::player::Player;
use crate::player::PlayerPosition;
use crate::score::Score;
use crate::rock_spawner;

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
    mat: Res<Materials>,
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

                match rock.size {
                    RockSize::Large => {
                        rock_spawner::rock_spawn(
                            commands, 
                            &mat,
                            RockSize::Midium, 
                            rock_transform.translation, 
                            rock.velocity_x);
                        rock_spawner::rock_spawn(
                            commands, 
                            &mat,
                            RockSize::Midium, 
                            rock_transform.translation, 
                            -rock.velocity_x);
                    }
                    RockSize::Midium => {
                        rock_spawner::rock_spawn(
                            commands, 
                            &mat,
                            RockSize::Small, 
                            rock_transform.translation, 
                            rock.velocity_x);
                        rock_spawner::rock_spawn(
                            commands, 
                            &mat,
                            RockSize::Small, 
                            rock_transform.translation, 
                            -rock.velocity_x);
                    }
                    RockSize::Small => {}
                }
            }
        }
    }
}

fn player_collision(
    pos: Res<PlayerPosition>,
    mut state: ResMut<State<AppState>>,
    query: Query<(&Transform, &Rock)>,
    // mut query: Query<&Transform, With<Rock>>,
) {
    for (transform, rock) in query.iter() {
        let size = match rock.size {
            RockSize::Large => {Vec2::new(128.0, 128.0)}
            RockSize::Midium => {Vec2::new(64.0, 64.0)}
            RockSize::Small => {Vec2::new(32.0, 32.0)}
        };

        let collision = collide(
            transform.translation, 
            size, 
            Vec3::new(pos.0, PLAYER_POSITION_Y, 0.0), 
            Vec2::new(32.0, 20.0));

        if let Some(_) = collision {
            state.set_next(AppState::GameOver).unwrap();
        }
    }
    
}
