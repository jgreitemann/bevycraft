use crate::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_system(update_hud);
    }
}

#[derive(Component)]
struct PlayerHealthText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("PixeloidSans.ttf"),
        font_size: 18.0,
        color: Color::WHITE,
    };
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
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
                                value: "Player Health: ".to_string(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: String::new(),
                                style: style.clone(),
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..default()
                })
                .insert(PlayerHealthText);
        });
}

fn update_hud(
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut text_query: Query<&mut Text, With<PlayerHealthText>>,
) {
    for player_health in player_query.iter() {
        let mut text = text_query.single_mut();
        text.sections[1].value = format!("{}", player_health.hitpoints());
    }
}
