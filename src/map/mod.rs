mod mouse;
mod texture;

use crate::prelude::*;

pub const MAP_SIZE: MapSize = MapSize(2, 2);
pub const CHUNK_SIZE: ChunkSize = ChunkSize(8, 8);
pub const TILE_SIZE: TileSize = TileSize(32.0, 32.0);
pub const TEXTURE_SIZE: TextureSize = TextureSize(512.0, 512.0);
pub const MAP_ID: u16 = 0;
pub const MAP_LAYER_ID: u16 = 0;

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

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, texture::build_texture_atlases)
            .add_startup_system(spawn_map_layer)
            .add_event::<mouse::TileInteraction>()
            .add_system(texture::set_texture_filters_to_nearest)
            .add_system(mouse::mouse_click_tile_interaction)
            .add_system(mouse::hide_tiles_by_click.after(mouse::mouse_click_tile_interaction));
    }
}

fn spawn_map_layer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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

    layer_builder.set_all(TileBundle {
        tile: Tile {
            texture_index: 46,
            ..default()
        },
        ..default()
    });

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
