use bevy::prelude::*;
use kanirock::text_builder;
use crate::consts::*;

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

fn setup_title (
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
            parent.spawn(
                text_builder(
                    font_handle.clone(), 
                    Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(TITLE_MAIN_POS_Y),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TITLE_MAIN_CONTENTS,
                    TITLE_COLOR,
                    TITLE_MAIN_SIZE)
                );
        })
        .with_children(|parent| {
            parent.spawn(
                text_builder(
                    font_handle.clone(), 
                    Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(TITLE_SUB_POS_Y),
                            ..Default::default()
                        },
                        ..Default::default()    
                    }, 
                    TITLE_SUB_CONTENTS,
                    TITLE_COLOR, TITLE_SUB_SIZE)
            );
        })
        .with(Title);  
}

fn press_start(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if input.pressed(KeyCode::Space) {
        state.set_next(AppState::Game);
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