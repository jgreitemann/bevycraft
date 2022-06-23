use super::{UiState, UiStyles};
use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::ui::FocusPolicy;

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
                    .with_system(add_newly_carried_item_to_inventory)
                    .with_system(inventory_item_interaction)
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

#[derive(Component, Debug)]
struct InventoryBar;

#[derive(Component, Debug)]
struct RepresentsItem(Entity);

fn set_up_hud(
    mut commands: Commands,
    styles: Res<UiStyles>,
    player_query: Query<Entity, With<Player>>,
    inventory_query: Query<(Entity, &Name, &Handle<Image>, &CarriedBy), With<Item>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(HudItem)
        .with_children(|parent| {
            parent
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

            // Inventory bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    color: Color::rgba_linear(0.05, 0.05, 0.05, 0.3).into(),
                    ..default()
                })
                .insert(InventoryBar)
                .with_children(|parent| {
                    populate_inventory(parent, player_query.iter().next(), inventory_query.iter());
                });
        });
}

fn populate_inventory<'w>(
    parent: &mut ChildBuilder,
    player: Option<Entity>,
    new_items: impl Iterator<Item = (Entity, &'w Name, &'w Handle<Image>, &'w CarriedBy)>,
) {
    if let Some(player) = player {
        for (inventory_item, name, image, _) in
            new_items.filter(|(_, _, _, CarriedBy(carrier))| *carrier == player)
        {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(RepresentsItem(inventory_item))
                .insert(name.clone())
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        image: UiImage((*image).clone()),
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    });
                });
        }
    }
}

fn add_newly_carried_item_to_inventory(
    mut commands: Commands,
    inventory_bar_query: Query<Entity, With<InventoryBar>>,
    player_query: Query<Entity, With<Player>>,
    new_items_query: Query<
        (Entity, &Name, &Handle<Image>, &CarriedBy),
        (With<Item>, Added<CarriedBy>),
    >,
) {
    commands
        .entity(inventory_bar_query.single())
        .with_children(|parent| {
            populate_inventory(parent, player_query.iter().next(), new_items_query.iter());
        });
}

fn inventory_item_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction, &RepresentsItem),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (inventory_item, &interaction, &RepresentsItem(item)) in interaction_query.iter() {
        if interaction == Interaction::Clicked {
            commands.entity(item).insert(Used);
            commands.entity(inventory_item).despawn_recursive();
        }
    }
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
