
pub const APP_STATE_STAGE: &str = "app_state_stage";

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    Title,
    Game,
}