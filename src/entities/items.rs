use crate::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_picks_up_items);
    }
}

#[derive(Component, Debug)]
pub struct Item;

#[derive(Component, Debug, PartialEq)]
pub struct CarriedBy(pub Entity);

#[derive(Component, Debug)]
pub struct AmuletOfYala;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    name: Name,
    position: Position,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl ItemBundle {
    pub fn new(name: &str, position: Position, texture: Handle<Image>) -> Self {
        let world_pos = tile_center(&position);
        ItemBundle {
            item: Item,
            name: Name::new(name.to_string()),
            position,
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(world_pos),
                texture,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct AmuletBundle {
    amulet: AmuletOfYala,
    item: Item,
    name: Name,
    position: Position,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl AmuletBundle {
    pub fn new(position: Position, texture_atlas: &DefaultTextureAtlas) -> Self {
        let DefaultTextureAtlas(atlas_handle) = texture_atlas;
        let world_pos = tile_center(&position);
        Self {
            amulet: AmuletOfYala,
            item: Item,
            name: Name::new("Amulet of Yala"),
            position,
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(124),
                texture_atlas: (*atlas_handle).clone(),
                transform: Transform::from_translation(world_pos),
                ..default()
            },
        }
    }
}

fn player_picks_up_items(
    mut commands: Commands,
    player_query: Query<(Entity, &Position), (With<Player>, Changed<Position>)>,
    item_query: Query<(Entity, &Position), With<Item>>,
) {
    if let Some((player_entity, player_pos)) = player_query.iter().next() {
        for (item_entity, _) in item_query
            .iter()
            .filter(|(_, item_pos)| *item_pos == player_pos)
        {
            commands
                .entity(item_entity)
                .remove::<Position>()
                .insert(Visibility { is_visible: false })
                .insert(CarriedBy(player_entity));
        }
    }
}
