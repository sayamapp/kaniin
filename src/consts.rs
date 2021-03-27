use bevy::prelude::*;
// Window Settings
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// Game State
#[derive(PartialEq, Clone, Copy)]
pub enum AppState {
    Title,
    Game,
    GameOver,
}

pub struct Materials {
    pub rock_material: Handle<ColorMaterial>,
    pub bullet_material: Handle<ColorMaterial>,
}

pub const APP_STATE_STAGE: &str = "app_state_stage";

// assets path
pub const FONT_PASS: &str = "fonts/Square.ttf";
pub const BACKGROUND_TEXTURE: &str = "textures/floor.png";
pub const PLAYER_TEXTURE: &str = "textures/kani.png";
pub const ROCK_TEXTURE: &str = "textures/rock2.png";
pub const UFO_TEXTURE: &str = "textures/ufo.png";
pub const TURBO_FISH_TEXTURE: &str = "textures/turbofish.png";


// title settings
pub const TITLE_COLOR: Color = Color::YELLOW; 
pub const TITLE_MAIN_CONTENTS: &str = "KANIROCK";
pub const TITLE_MAIN_POS_Y: f32 = 100.0;
pub const TITLE_MAIN_SIZE: f32 = 100.0;
pub const TITLE_SUB_CONTENTS: &str = "Press Space key";
pub const TITLE_SUB_POS_Y: f32 = 300.0;
pub const TITLE_SUB_SIZE: f32 = 50.0;

// background settings
pub const BACKGROUND_POSITION_Y: f32 = -265.0;
pub const BACKGROUND_SCALE: f32 = 4.0;

// player settings
pub const PLAYER_TEXTURE_SIZE: f32 = 16.0;
pub const PLAYER_TEXTURE_COLUMNS: usize = 24;
pub const PLAYER_TEXTURE_ROWS: usize = 1;
pub const PLAYER_POSITION_Y: f32 = -200.0;
pub const PLAYER_SCALE: f32 = 4.0;
pub const PLAYER_ANIMATION_TIMER: f32 = 0.05;
pub const PLAYER_MOVE_SPEED: f32 = 5.0;

// rock settings
pub const ROCK_GRAVITY: f32 = 0.05;
pub const ROCK_SPAWN_POSITION_Y: f32 = 200.0;
pub const ROCK_MAX_SPEED_Y: f32 = 5.0;
// settings for debug
pub const DEBUG_ROCK_SIZE: f32 = 64.0;


// ufo settings
pub const UFO_SPEED: f32 = 2.0;
pub const UFO_POSITION_Y: f32 = 250.0;
pub const UFO_WIDTH: f32 = 128.0;

// spawner settings
pub const SPAWN_DURATION: u64 = 3330;
// bullet settings
pub const BULLET_SPEED: f32 = 10.0;