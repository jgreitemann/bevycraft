use super::{UiState, UiStyles};
use crate::prelude::*;

use bevy_ui_navigation::{
    components::FocusableButtonBundle, event_helpers::NavEventQuery, FocusState, Focusable,
    NavRequestSystem,
};

const BACKGROUND: Color = Color::rgba(0.1, 0.1, 0.1, 0.90);
const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.45, 0.75, 0.45);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonAction>()
            .add_enter_system(UiState::Menu, set_up_menu)
            .add_exit_system(UiState::Menu, tear_down_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(UiState::Menu)
                    .after(NavRequestSystem)
                    .with_system(button_system)
                    .with_system(handle_nav_events)
                    .into(),
            );
    }
}

#[derive(Component, Debug)]
struct MenuItem;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
enum ButtonAction {
    ResumeGame,
    RestartGame,
}

fn set_up_menu(
    mut commands: Commands,
    styles: Res<UiStyles>,
    turn_state: Res<CurrentState<TurnState>>,
) {
    use TurnState::*;
    let CurrentState(current_turn_state) = *turn_state;
    let heading_text = match current_turn_state {
        Loading => "Loading game assets...",
        Victory => "Victory!",
        Defeat => "Defeat!",
        Pause => "Game Paused",
        _ => {
            panic!("Menu should not be shown in state {:?}", current_turn_state)
        }
    };
    let restart_text = match current_turn_state {
        Loading => "",
        Victory | Defeat => "Play again",
        Pause => "Restart game",
        _ => {
            panic!("Menu should not be shown in state {:?}", current_turn_state)
        }
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: BACKGROUND.into(),
            ..default()
        })
        .insert(MenuItem)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(heading_text, styles.heading(), Default::default()),
                style: Style {
                    margin: Rect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });

            if current_turn_state == Pause {
                parent
                    .spawn_bundle(FocusableButtonBundle {
                        button_bundle: ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: Rect::all(Val::Px(10.0)),
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ButtonAction::ResumeGame)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section("Resume", styles.text(), Default::default()),
                            ..default()
                        });
                    });
            }

            if current_turn_state != Loading {
                parent
                    .spawn_bundle(FocusableButtonBundle {
                        button_bundle: ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: Rect::all(Val::Px(10.0)),
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ButtonAction::RestartGame)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                restart_text,
                                styles.text(),
                                Default::default(),
                            ),
                            ..default()
                        });
                    });
            }
        });
}

fn tear_down_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuItem>>) {
    for menu_item in menu_query.iter() {
        commands.entity(menu_item).despawn_recursive();
    }
}

fn button_system(mut focusables: Query<(&Focusable, &mut UiColor), Changed<Focusable>>) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = match focus.state() {
            FocusState::Dormant | FocusState::Active => PRESSED_BUTTON,
            FocusState::Focused => HOVERED_BUTTON,
            FocusState::Inert => NORMAL_BUTTON,
        };
        *color = new_color.into();
    }
}

fn handle_nav_events(mut commands: Commands, mut buttons: NavEventQuery<&mut ButtonAction>) {
    match buttons
        .single_activated_mut()
        .deref_mut()
        .ignore_remaining()
    {
        Some(ButtonAction::ResumeGame) => {
            commands.insert_resource(NextState(TurnState::AwaitingInput));
        }
        Some(ButtonAction::RestartGame) => {
            commands.insert_resource(NextState(TurnState::NewGame));
        }
        None => {}
    }
}
