use crate::prelude::*;

pub struct EntitySpawningPlugin;

impl Plugin for EntitySpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(TurnState::NewGame, spawn_entities);
    }
}

fn find_start_positions(
    spawnable_locations: &[Position],
    tile_map_query: TileMapQuery,
) -> (Position, Position) {
    let map_adapter = MapAdapter::new(tile_map_query);

    let center = map_adapter.dimensions() / 2;

    let player_start = spawnable_locations
        .into_iter()
        .min_by_key(|&&p| DistanceAlg::PythagorasSquared.distance2d(center, p.into()) as i64)
        .unwrap()
        .clone();

    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &[map_adapter.point2d_to_index(player_start.into())],
        &map_adapter,
        1024.0,
    );

    const UNREACHABLE: &f32 = &f32::MAX;
    let amulet_start = map_adapter
        .index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|(_, lhs_dist), (_, rhs_dist)| lhs_dist.partial_cmp(rhs_dist).unwrap())
                .unwrap()
                .0,
        )
        .into();

    (player_start, amulet_start)
}

fn spawn_entities(
    mut commands: Commands,
    tile_map_query: TileMapQuery,
    entity_query: Query<Entity, Or<(With<Mob>, With<Item>)>>,
    texture_atlas: Res<DefaultTextureAtlas>,
    item_data: Res<Assets<ItemData>>,
    asset_server: Res<AssetServer>,
) {
    const NUM_MONSTERS: usize = 50;
    const MIN_DISTANCE: f32 = 10f32;

    // Despawn all entities in case of game reset
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }

    // Determine spawnable locations
    let spawnable_locations: Vec<Position> = tile_map_query
        .tile_query
        .iter()
        .filter(|(_, tile_type)| **tile_type == TileType::Floor)
        .map(|(&tile_pos, _)| tile_pos.into())
        .collect();

    // Spawn player
    let (player_start, amulet_start) = find_start_positions(&spawnable_locations, tile_map_query);
    commands.spawn_bundle(PlayerBundle::new(player_start, texture_atlas.as_ref()));

    // Spawn the amulet
    commands.spawn_bundle(AmuletBundle::new(amulet_start, texture_atlas.as_ref()));

    // Exclude the vicinity of the player from the spawnable set
    let mut spawnable_locations: Vec<_> = spawnable_locations
        .into_iter()
        .filter(|&p: &Position| {
            DistanceAlg::Pythagoras.distance2d(p.into(), player_start.into()) > MIN_DISTANCE
        })
        .collect();

    use rand::prelude::*;
    let mut rng = thread_rng();
    spawnable_locations.shuffle(&mut rng);

    let location_count = spawnable_locations.len();
    let mut spawnable_locations_iter = spawnable_locations.into_iter();

    // Spawn items
    for (item, spawn_location) in item_data.iter().flat_map(|(_, item)| {
        spawnable_locations_iter
            .by_ref()
            .take((item.frequency * location_count as f32) as usize)
            .map(|loc| (item, loc))
            .collect::<Vec<_>>()
    }) {
        commands
            .spawn_bundle(ItemBundle::new(
                &item.name,
                spawn_location,
                asset_server.load(item.icon.as_str()),
            ))
            .insert(Effects(item.effects.clone()));
    }

    // Spawn monsters
    for spawn_location in spawnable_locations_iter.take(NUM_MONSTERS) {
        commands.spawn_bundle(HostileMobBundle::new(
            spawn_location,
            texture_atlas.as_ref(),
        ));
    }
}
