use super::{UiState, UiStyles};
use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(UiState::Hud, set_up_hud)
            .add_exit_system(UiState::Hud, tear_down_hud)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(UiState::Hud)
                    .with_system(update_fps_hud)
                    .with_system(update_health_hud)
                    .into(),
            );
    }
}

#[derive(Component, Debug)]
struct HudItem;

#[derive(Component, Debug)]
struct FpsText;

#[derive(Component, Debug)]
struct PlayerHealthText;

fn set_up_hud(mut commands: Commands, styles: Res<UiStyles>) {
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
        .insert(HudItem)
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

fn tear_down_hud(mut commands: Commands, hud_query: Query<Entity, With<HudItem>>) {
    for hud_item in hud_query.iter() {
        commands.entity(hud_item).despawn_recursive();
    }
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
    player_query: Query<(&Health, ChangeTrackers<Health>), With<Player>>,
    mut text_query: Query<(&mut Text, ChangeTrackers<PlayerHealthText>), With<PlayerHealthText>>,
) {
    for (player_health, health_tracker) in player_query.iter() {
        for (mut text, text_tracker) in text_query.iter_mut() {
            if health_tracker.is_changed() || text_tracker.is_changed() {
                text.sections[1].value = format!("{}", player_health.hitpoints());
            }
        }
    }
}
