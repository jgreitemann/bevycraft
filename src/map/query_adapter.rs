use crate::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct TileMapQuery<'w, 's> {
    pub map_query: MapQuery<'w, 's>,
    pub tile_query: Query<'w, 's, &'static TileType>,
}

impl TileMapQuery<'_, '_> {
    fn in_bounds(&mut self, Position(p): &Position) -> bool {
        self.map_query
            .get_layer(MAP_ID, MAP_LAYER_ID)
            .map(|(_, layer)| {
                let size = layer.get_layer_size_in_tiles();
                (0..size.0 as i32).contains(&p.x) && (0..size.1 as i32).contains(&p.y)
            })
            .unwrap_or(false)
    }

    pub fn can_enter_tile(&mut self, &p: &Position) -> bool {
        if let Ok(tile_pos) = p.try_into() {
            self.map_query
                .get_tile_entity(tile_pos, MAP_ID, MAP_LAYER_ID)
                .map(|tile_entity| {
                    self.tile_query
                        .get_component::<TileType>(tile_entity)
                        .map(|tile_type| *tile_type == TileType::Floor)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        } else {
            false
        }
    }
}
