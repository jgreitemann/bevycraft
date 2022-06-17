use super::{UiState, UiStyles};
use crate::prelude::*;

const BACKGROUND: Color = Color::rgba(0.1, 0.1, 0.1, 0.95);
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
                    .with_system(handle_button_interaction)
                    .with_system(resume_game)
                    .with_system(restart_game)
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
        AwaitingInput | PlayerTurn | MonsterTurn => {
            panic!("Menu should not be shown in state {:?}", current_turn_state)
        }
        Victory => "Victory!",
        Defeat => "Defeat!",
        Pause => "Game Paused",
    };
    let restart_text = match current_turn_state {
        AwaitingInput | PlayerTurn | MonsterTurn => {
            panic!("Menu should not be shown in state {:?}", current_turn_state)
        }
        Victory | Defeat => "Play again",
        Pause => "Restart game",
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
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect::all(Val::Px(10.0)),
                            ..default()
                        },
                        color: NORMAL_BUTTON.into(),
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
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Px(10.0)),
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonAction::RestartGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(restart_text, styles.text(), Default::default()),
                        ..default()
                    });
                });
        });
}

fn tear_down_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuItem>>) {
    for menu_item in menu_query.iter() {
        commands.entity(menu_item).despawn_recursive();
    }
}

fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut actions: EventWriter<ButtonAction>,
) {
    for (interaction, mut color, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                actions.send(*action);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn resume_game(mut commands: Commands, mut actions: EventReader<ButtonAction>) {
    for _ in actions
        .iter()
        .filter(|&action| *action == ButtonAction::ResumeGame)
    {
        commands.insert_resource(NextState(TurnState::AwaitingInput));
    }
}

fn restart_game(
    mut commands: Commands,
    mut actions: EventReader<ButtonAction>,
    mut reset_evt: EventWriter<ResetGame>,
) {
    for _ in actions
        .iter()
        .filter(|&action| *action == ButtonAction::RestartGame)
    {
        commands.insert_resource(NextState(TurnState::AwaitingInput));
        reset_evt.send(ResetGame);
    }
}
