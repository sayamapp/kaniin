pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const BACKGROUND_TEXTURE: &str = "textures/floor.png";
pub const PLAYER_TEXTURE: &str = "textures/kani.png";

pub const SPRITE_SCALE: f32 = 4.0;

pub const PLAYER_TA_SIZE: f32 = 16.0;
pub const PLAYER_TA_COLUMNS: usize = 24;
pub const PLAYER_TA_ROWS: usize = 1;
pub const PLAYER_POSITION_Y: f32 = -200.0;
pub const PLAYER_ANIMATION_TIMER: f32 = 0.05;
pub const PLAYER_MOVE_SPEED: f32 = 5.0;

pub const FONT_PASS: &str = "fonts/Square.ttf";
pub const TITLE_MAIN_CONTENTS: &str = "Press SPACE KEY";
pub const TITLE_MAIN_POS_Y: f32 = 200.0;
pub const TITLE_MAIN_SIZE: f32 = 100.0;
pub const TITLE_SUB_CONTENTS: &str = "KANIROCK";
pub const TITLE_SUB_POS_Y: f32 = 400.0;
pub const TITLE_SUB_SIZE: f32 = 50.0;

pub const BACKGROUND_POSITION_Y: f32 = -265.0;

pub const FLOOR_POSITION_Y: f32 = -265.0;
pub const ROCK_SIZE: f32 = 16.0;


pub const APP_STATE_STAGE: &str = "app_state_stage";

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    Title,
    Game,
}