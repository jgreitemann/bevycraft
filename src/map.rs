use crate::prelude::*;

pub fn spawn_map_layer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("dungeonfont.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(32.0, 32.0),
            TextureSize(512.0, 512.0),
        ),
        0u16,
        0u16,
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
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-256.0, -256.0, 0.0))
        .insert(GlobalTransform::default());
}
