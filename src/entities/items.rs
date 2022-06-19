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
    position: Position,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl ItemBundle {
    pub fn new(
        position: Position,
        texture_index: usize,
        texture_atlas: &DefaultTextureAtlas,
    ) -> Self {
        let DefaultTextureAtlas(atlas_handle) = texture_atlas;
        let world_pos = tile_center(&position);
        ItemBundle {
            item: Item,
            position,
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(world_pos),
                texture_atlas: atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(texture_index),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct AmuletBundle {
    amulet: AmuletOfYala,
    #[bundle]
    item_bundle: ItemBundle,
}

impl AmuletBundle {
    pub fn new(position: Position, texture_atlas: &DefaultTextureAtlas) -> Self {
        Self {
            amulet: AmuletOfYala,
            item_bundle: ItemBundle::new(position, 124, texture_atlas),
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
