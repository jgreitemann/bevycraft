mod map_builder;
mod mouse;
mod query_adapter;

use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

use map_builder::*;
pub use query_adapter::*;

pub const MAP_SIZE: MapSize = MapSize(10, 6);
pub const CHUNK_SIZE: ChunkSize = ChunkSize(8, 8);
pub const TILE_SIZE: TileSize = TileSize(32.0, 32.0);
pub const TEXTURE_SIZE: TextureSize = TextureSize(512.0, 512.0);
pub const MAP_ID: u16 = 0;
pub const MAP_LAYER_ID: u16 = 0;
pub const MAP_WIDTH: usize = MAP_SIZE.0 as usize * CHUNK_SIZE.0 as usize;
pub const MAP_HEIGHT: usize = MAP_SIZE.1 as usize * CHUNK_SIZE.1 as usize;

pub fn world_to_tile(world_pos: &Vec2) -> TilePos {
    TilePos(
        ((world_pos.x + 0.5 * TEXTURE_SIZE.0) / TILE_SIZE.0) as u32,
        ((world_pos.y + 0.5 * TEXTURE_SIZE.1) / TILE_SIZE.1) as u32,
    )
}

pub fn tile_center(Position(vec): &Position) -> Vec3 {
    Vec3::new(
        (vec.x as f32 + 0.5) * TILE_SIZE.0 - 0.5 * TEXTURE_SIZE.0,
        (vec.y as f32 + 0.5) * TILE_SIZE.1 - 0.5 * TEXTURE_SIZE.1,
        1.,
    )
}

#[derive(Copy, Clone, Component, Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
}

struct CurrentBiome(Option<BiomeData>);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(CurrentBiome(None))
            .add_enter_system(TurnState::NewGame, spawn_map_layer)
            .add_event::<mouse::TileInteraction>()
            .add_system(
                mouse::mouse_click_tile_interaction
                    .run_in_state(TurnState::AwaitingInput)
                    .label("mouse_click_tile_interaction"),
            )
            .add_system(
                mouse::hide_tiles_by_click
                    .run_in_state(TurnState::AwaitingInput)
                    .after("mouse_click_tile_interaction"),
            )
            .add_system_to_stage(CoreStage::PostUpdate, sync_tiles);
    }
}

#[derive(Bundle, Clone)]
struct BevycraftTileBundle {
    tile_type: TileType,
    #[bundle]
    tilemap_bundle: TileBundle,
}

impl TileBundleTrait for BevycraftTileBundle {
    fn get_tile_pos_mut(&mut self) -> &mut TilePos {
        self.tilemap_bundle.get_tile_pos_mut()
    }

    fn get_tile_parent(&mut self) -> &mut TileParent {
        self.tilemap_bundle.get_tile_parent()
    }
}

fn spawn_map_layer(
    mut commands: Commands,
    mut map_query: MapQuery,
    asset_server: Res<AssetServer>,
    biomes: Res<Assets<BiomeData>>,
) {
    // Despawn the map in case of game reset
    map_query.despawn(&mut commands, MAP_ID);

    // Pick a random biome for now
    let mut rng = thread_rng();
    let biome_data: Vec<_> = biomes.iter().map(|(_, data)| data).collect();
    let biome = biome_data.choose(&mut rng).cloned();
    commands.insert_resource(CurrentBiome(biome.cloned()));

    let texture_handle = asset_server.load("dungeonfont.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(MAP_SIZE, CHUNK_SIZE, TILE_SIZE, TEXTURE_SIZE),
        MAP_ID,
        MAP_LAYER_ID,
    );

    let map_builder = MapBuilder::new();
    let mut map_iter = map_builder.map_data.iter();

    layer_builder.for_each_tiles_mut(|_, bundle| {
        *bundle = map_iter.next().map(|&tile_type| BevycraftTileBundle {
            tile_type,
            tilemap_bundle: TileBundle {
                tile: Tile {
                    texture_index: biome
                        .and_then(|biome| biome.tile_textures.get(&tile_type).cloned())
                        .unwrap_or(176),
                    visible: false,
                    ..default()
                },
                ..default()
            },
        });
    });
    assert!(
        map_iter.next().is_none(),
        "The map builder was not exhaustively consumed by the map."
    );

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, MAP_LAYER_ID, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            -0.5 * TEXTURE_SIZE.0,
            -0.5 * TEXTURE_SIZE.1,
            0.0,
        ))
        .insert(GlobalTransform::default());
}

fn sync_tiles(
    mut tile_query: Query<(&TileType, &TilePos, &mut Tile), Or<(Changed<TileType>, Changed<Tile>)>>,
    mut map_query: MapQuery,
    current_biome: Res<CurrentBiome>,
) {
    if let CurrentBiome(Some(biome)) = current_biome.as_ref() {
        for (tile_type, tile_pos, mut tile) in tile_query.iter_mut() {
            tile.texture_index = biome.tile_textures.get(tile_type).cloned().unwrap_or(176);
            map_query.notify_chunk_for_tile(*tile_pos, MAP_ID, MAP_LAYER_ID);
        }
    }
}
