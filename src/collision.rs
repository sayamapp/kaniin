use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::consts::*;

use crate::bullet::Bullet;
use crate::rock::Rock;
use crate::ufo::UFO;
use crate::player::Player;
use crate::player::PlayerPosition;

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
    mut query_bullet: Query<(&mut Transform, &mut Bullet)>,
    mut query_rock: Query<(Entity, &Transform), With<Rock>>,
) {
    for (mut bullet_transform, mut bullet) in query_bullet.iter_mut() {

        for (rock_entity, rock_transform) in query_rock.iter_mut() {
            let collision = collide(
                bullet_transform.translation, 
                Vec2::new(64.0, 128.0), 
                rock_transform.translation, 
            Vec2::new(64.0, 64.0));

            if let Some(_) = collision {
                bullet.0 = false;
                bullet_transform.translation = Vec3::new(0.0, -900.0, 0.0);
                commands.despawn(rock_entity);
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