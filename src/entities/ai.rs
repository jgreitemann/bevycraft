use crate::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::First,
            chasing.run_in_state(TurnState::MonsterTurn),
        );
    }
}

pub fn chasing(
    movers: Query<(Entity, &Position), With<ChasingPlayer>>,
    player: Query<&Position, With<Player>>,
    map_query: TileMapQuery,
    mut msgs: EventWriter<WantsToMove>,
) {
    let map = MapAdapter::new(map_query);
    let &player_pos = player.single();
    let player_idx = map.point2d_to_index(player_pos.into());

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(MAP_WIDTH, MAP_HEIGHT, &search_targets, &map, 1024.0);

    movers.iter().for_each(|(entity, &pos)| {
        let idx = map.point2d_to_index(pos.into());
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &map) {
            let distance = DistanceAlg::Pythagoras.distance2d(pos.into(), player_pos.into());
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination).into()
            } else {
                player_pos
            };

            msgs.send(WantsToMove {
                entity,
                destination,
            });
        }
    });
}
