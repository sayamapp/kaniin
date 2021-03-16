use bevy::prelude::*;
use crate::consts::*;

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

pub struct RockPlugin;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Game, setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, rock_move.system());
    }
}

fn setup (
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let handle = asset_server.load("textures/rock2.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(handle.into()),
            transform: Transform {
                translation: Vec3::new(0., 100., 0.),
                scale: Vec3::new(8., 8., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Rock {size: RockSize::Midium, velocity_x: 1.0, velocity_y: 0.0});
}

fn rock_move(
    mut query: Query<(&mut Transform, &mut Rock)>,
) {
    for (mut pos, mut rock) in query.iter_mut() {
        pos.translation.x += rock.velocity_x;
        if pos.translation.x >= 350.0 && rock.velocity_x > 0. {
            rock.velocity_x = -rock.velocity_x;
        }
        if pos.translation.x <= -350.0 && rock.velocity_x < 0. {
            rock.velocity_x = -rock.velocity_x;
        }

        pos.translation.y += rock.velocity_y;
        rock.velocity_y += -0.1;

        if pos.translation.y <= -200.0 && rock.velocity_y < 0. {
            rock.velocity_y = -rock.velocity_y;
        }
    }
}