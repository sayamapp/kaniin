use std::ops::Neg;

use bevy::prelude::*;
use crate::consts::*;

pub struct RockPlugin;
impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_update(APP_STATE_STAGE, AppState::Game, rock_move.system());
    }
}
pub enum RockSize {
    Large,
    Midium,
    Small,
}

pub struct Rock {
    pub size: RockSize,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

fn rock_move(
    mut query: Query<(&mut Transform, &mut Rock)>,
) {
    for (mut transform, mut rock) in query.iter_mut() {

        let rock_size = match rock.size {
            RockSize::Large => {128.0}
            RockSize::Midium => {64.0}
            RockSize::Small => {32.0}
        };

        // rock move x
        transform.translation.x += rock.velocity_x;

        if transform.translation.x >= WINDOW_WIDTH / 2.0 - rock_size / 2.0 && rock.velocity_x.is_sign_positive() {
            rock.velocity_x = rock.velocity_x.neg();
        }
        if transform.translation.x <= -WINDOW_WIDTH / 2.0 + rock_size / 2.0 && rock.velocity_x.is_sign_negative() {
            rock.velocity_x = rock.velocity_x.neg();
        }

        // rock move y
        transform.translation.y += rock.velocity_y;
        rock.velocity_y -= ROCK_GRAVITY;

        if transform.translation.y <= BACKGROUND_POSITION_Y + 35.0 + rock_size / 2.0 && rock.velocity_y.is_sign_negative() {
            rock.velocity_y = ROCK_MAX_SPEED_Y;
        }
    }
}

