use crate::prelude::*;
use std::collections::HashMap;

use bevy::app::PluginGroupBuilder;
use bevy::reflect::TypeUuid;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_asset_ron::RonAssetPlugin;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum EffectData {
    Healing(u32),
    RevealMap,
}

#[derive(Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "efb7973b-784a-403b-8fd7-6707d6dd412f"]
pub struct ItemData {
    pub name: String,
    pub icon: String,
    pub frequency: f32,
    pub effects: Vec<EffectData>,
}

#[derive(Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "0f8f2dfb-6fba-4f34-b258-358b9438e3f9"]
pub struct BiomeData {
    pub name: String,
    pub levels: Vec<usize>,
    pub tile_textures: HashMap<TileType, u16>,
}

#[derive(AssetCollection)]
struct DataAssets {
    #[asset(path = "dungeonfont.png")]
    _texture_atlas: Handle<Image>,

    #[asset(path = "PixeloidSans.ttf")]
    _font: Handle<Font>,

    #[asset(key = "data.items", collection(typed))]
    _item_data: Vec<Handle<ItemData>>,

    #[asset(key = "data.biomes", collection(typed))]
    _biome_data: Vec<Handle<BiomeData>>,

    #[asset(key = "images.items", collection(typed))]
    _item_images: Vec<Handle<Image>>,
}

pub struct DataPlugins;

impl PluginGroup for DataPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(RonAssetPlugin::<ItemData>::new(&["item"]))
            .add(RonAssetPlugin::<BiomeData>::new(&["biome"]))
            .add(DataLoaderPlugin);
    }
}

struct DataLoaderPlugin;

impl Plugin for DataLoaderPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(TurnState::Loading)
            .continue_to_state(TurnState::NewGame)
            .with_dynamic_asset_collection_file("manifest.assets")
            .with_collection::<DataAssets>()
            .build(app);
    }
}
