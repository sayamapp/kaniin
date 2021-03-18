use bevy::prelude::*;
use crate::consts::*;
use crate::player::Player;
use crate::rock::Rock;
use bevy::sprite::collide_aabb::collide;

pub struct BubblePlugin;

pub struct Bubble {
    is_active: bool,
}


impl Plugin for BubblePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, spawn.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, collision.system());
    }
}


fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let handle = asset_server.load("textures/rock.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(handle.into()),
            transform: Transform {
                translation: Vec3::new(0., -900., 0.),
                scale: Vec3::new(4., 4., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Bubble { is_active: false });
}

fn spawn(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Bubble)>,
    player: Query<&Transform, With<Player>>,
) {
    for (mut transform, mut bubble) in query.iter_mut() {
        for player_transform in player.iter() {
            if input.pressed(KeyCode::LShift) {
                if bubble.is_active == false {
                    bubble.is_active = true;
                    transform.translation.x = player_transform.translation.x;
                    transform.translation.y = player_transform.translation.y;
                }
            }
            if bubble.is_active {
                transform.translation.y += 10.0;
            }

            if transform.translation.y > 400.0 {
                bubble.is_active = false;
                transform.translation.y = -500.0;
            }
        }
    }
}

fn collision(
    commands: &mut Commands,
    mut query_bubble: Query<(&mut Bubble, &mut Transform)>,
    mut query_rock: Query<(Entity, &Transform), With<Rock>>,
) {
    for (mut bubble, mut bubble_transform) in query_bubble.iter_mut() {
        for (rock_entity, rock_transform) in query_rock.iter_mut() {
            let collision = collide(
                bubble_transform.translation,
                Vec2::new(64.0, 64.0),
                rock_transform.translation,
                Vec2::new(64.0, 64.0),
            );
            if let Some(_) = collision {
                bubble.is_active = false;
                bubble_transform.translation = Vec3::new(0., -900., 0.);
                commands
                    .despawn(rock_entity);
                setup.system();
            }
        }
    }
}

