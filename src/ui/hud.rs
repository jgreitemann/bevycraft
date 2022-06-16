use super::UiStyles;
use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_hud)
            .add_system(update_fps_hud)
            .add_system(update_health_hud);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PlayerHealthText;

fn setup_hud(mut commands: Commands, styles: Res<UiStyles>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "FPS: ".to_string(),
                                style: styles.text(),
                            },
                            TextSection {
                                value: String::new(),
                                style: styles.text(),
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..default()
                })
                .insert(FpsText);
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Player Health: ".to_string(),
                                style: styles.text(),
                            },
                            TextSection {
                                value: String::new(),
                                style: styles.text(),
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..default()
                })
                .insert(PlayerHealthText);
        });
}

fn update_fps_hud(diagnostics: Res<Diagnostics>, mut text_query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            let mut text = text_query.single_mut();
            text.sections[1].value = format!("{:.1}", average);
        }
    }
}

fn update_health_hud(
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut text_query: Query<&mut Text, With<PlayerHealthText>>,
) {
    for player_health in player_query.iter() {
        let mut text = text_query.single_mut();
        text.sections[1].value = format!("{}", player_health.hitpoints());
    }
}
