use crate::{consts::*, UIFont};
use bevy::prelude::*;
pub struct Title;
pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Title, setup_title.system())
            .on_state_update(APP_STATE_STAGE, AppState::Title, press_start.system())
            .on_state_exit(APP_STATE_STAGE, AppState::Title, despawn_title.system());
    }
}
fn setup_title(
    commands: &mut Commands,
    font: Res<UIFont>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(text_helper(
                font.0.clone(), 
                TITLE_MAIN_CONTENTS, 
                Rect {
                    top: Val::Px(TITLE_MAIN_POS_Y),
                    ..Default::default()
                }, 
                TITLE_MAIN_SIZE,
            ));
        })
        .with_children(|parent| {
            parent.spawn(text_helper(
                font.0.clone(),
                TITLE_SUB_CONTENTS,
                Rect {
                    top: Val::Px(TITLE_SUB_POS_Y),
                    ..Default::default()
                },
                TITLE_SUB_SIZE,
            ));
        })
        .with(Title);
}
fn text_helper(font: Handle<Font>, value: &str, pos: Rect<Val>, size: f32) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: pos,
            ..Default::default()
        },
        text: Text {
            value: value.to_string(),
            font: font,
            style: TextStyle {
                font_size: size,
                color: Color::YELLOW,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
fn press_start(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if input.pressed(KeyCode::Space) {
        state.set_next(AppState::Game).expect("Err AppState::Title -> AppState::Game");
    }
}
fn despawn_title(
    commands: &mut Commands,
    query: Query<(Entity, &Title)>,
) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}