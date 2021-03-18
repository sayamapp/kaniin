use bevy::prelude::*;
use crate::consts::*;

struct Title;

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(APP_STATE_STAGE, AppState::Title, setup_title.system())
            .on_state_update(APP_STATE_STAGE, AppState::Title, title_input.system())
            .on_state_exit(APP_STATE_STAGE, AppState::Title, despawn_title.system());
    }
}

fn setup_title(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server.load(FONT_PASS);

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
            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(TITLE_SUB_POS_Y),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: TITLE_SUB_CONTENTS.to_string(),
                    font: font_handle.clone(),
                    style: TextStyle {
                        font_size: TITLE_SUB_SIZE,
                        color: Color::YELLOW,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(TITLE_MAIN_POS_Y),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: TITLE_MAIN_CONTENTS.to_string(),
                    font: font_handle.clone(),
                    style: TextStyle {
                        font_size: TITLE_MAIN_SIZE,
                        color: Color::YELLOW,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with(Title);
}

fn despawn_title(
    commands: &mut Commands,
    query: Query<(Entity, &Title)>,
) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}

fn title_input(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if input.pressed(KeyCode::Space) {
        state
            .set_next(AppState::Game);
    }
}