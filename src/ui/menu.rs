use super::UiStyles;
use crate::prelude::*;

const BACKGROUND: Color = Color::rgba(0.1, 0.1, 0.1, 0.95);
const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.45, 0.75, 0.45);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu)
            .add_system(handle_button_interaction);
    }
}

#[derive(Component, Debug)]
enum ButtonAction {
    PlayAgain,
}

fn setup_menu(mut commands: Commands, styles: Res<UiStyles>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                margin: Rect::all(Val::Px(20.0)),
                ..default()
            },
            color: BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section("Victory!", styles.heading(), Default::default()),
                ..default()
            });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Px(20.0)),
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonAction::PlayAgain)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Play Again", styles.text(), Default::default()),
                        ..default()
                    });
                });
        });
}

fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match *action {
                    ButtonAction::PlayAgain => info!("Resetting game..."),
                }
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
