use bevy::prelude::*;
use crate::consts::*;
use crate::player::Player;
use crate::rock::Rock;
use bevy::sprite::collide_aabb::collide;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_update(APP_STATE_STAGE, AppState::Game, test.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, collision.system());
    }
}

fn test (
    query: Query<(&Transform, &Sprite), With<Player>>,
) {
    for (transform, sprite) in query.iter() {
        println!("{} {}", transform.translation.x, sprite.size);
    }
}

fn collision(
    query: Query<(&Transform, &Player)>,
    query2: Query<(&Transform, &Rock)>,
) {
    for (transform1,  _player) in query.iter() {
        for (transform2, _rock) in query2.iter() {
            let collision = collide(
                transform1.translation,
               Vec2::new(64., 64.),
                transform2.translation,
                Vec2::new(100., 100.,)
            );
            if let Some(x) = collision {
                println!("COLLISION");
            }
        }
    }
}