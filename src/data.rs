use crate::prelude::*;

use bevy::app::PluginGroupBuilder;
use bevy::reflect::TypeUuid;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_asset_ron::RonAssetPlugin;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum EffectData {
    Healing(u32),
}

#[derive(Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "efb7973b-784a-403b-8fd7-6707d6dd412f"]
pub struct ItemData {
    pub name: String,
    pub texture_index: usize,
    pub frequency: f32,
    pub effects: Vec<EffectData>,
}

#[derive(AssetCollection)]
struct DataAssets {
    #[asset(path = "dungeonfont.png")]
    _texture_atlas: Handle<Image>,

    #[asset(path = "PixeloidSans.ttf")]
    _font: Handle<Font>,

    #[asset(path = "data/items", collection)]
    _items: Vec<HandleUntyped>,
}

pub struct DataPlugins;

impl PluginGroup for DataPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(RonAssetPlugin::<ItemData>::new(&["item"]))
            .add(DataLoaderPlugin);
    }
}

struct DataLoaderPlugin;

impl Plugin for DataLoaderPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(TurnState::Loading)
            .continue_to_state(TurnState::NewGame)
            .with_collection::<DataAssets>()
            .build(app);
    }
}
