use crate::prelude::*;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;

#[derive(SystemParam)]
pub struct TileMapQuery<'w, 's> {
    pub map_query: MapQuery<'w, 's>,
    pub tile_query: Query<'w, 's, (&'static TilePos, &'static TileType)>,
}

impl TileMapQuery<'_, '_> {
    fn dimensions(&mut self) -> MapSize {
        self.map_query
            .get_layer(MAP_ID, MAP_LAYER_ID)
            .map(|(_, layer)| layer.get_layer_size_in_tiles())
            .unwrap()
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

pub struct MapAdapter<'w, 's> {
    cell: RefCell<TileMapQuery<'w, 's>>,
}

impl<'w, 's> MapAdapter<'w, 's> {
    pub fn new(tile_map_query: TileMapQuery<'w, 's>) -> Self {
        Self {
            cell: RefCell::new(tile_map_query),
        }
    }
}

impl BaseMap for MapAdapter<'_, '_> {
    fn is_opaque(&self, idx: usize) -> bool {
        let tile_position: Position = self.index_to_point2d(idx).into();
        let mut tile_map_query = self.cell.borrow_mut();
        let tile_entity = tile_map_query
            .map_query
            .get_tile_entity(tile_position.try_into().unwrap(), MAP_ID, MAP_LAYER_ID)
            .unwrap();
        *tile_map_query
            .tile_query
            .get_component::<TileType>(tile_entity)
            .unwrap()
            == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let tile_position: Position = self.index_to_point2d(idx).into();
        let neighbor_positions: SmallVec<[Position; 10]> = {
            let mut tile_map_query = self.cell.borrow_mut();
            tile_map_query
                .map_query
                .get_tile_neighbors(tile_position.try_into().unwrap(), MAP_ID, MAP_LAYER_ID)
                .into_iter()
                .take(4)
                .filter_map(|result| result.ok())
                .filter(|entity| {
                    *tile_map_query
                        .tile_query
                        .get_component::<TileType>(*entity)
                        .unwrap()
                        == TileType::Floor
                })
                .map(|entity| {
                    tile_map_query
                        .tile_query
                        .get_component::<TilePos>(entity)
                        .unwrap()
                        .clone()
                        .into()
                })
                .collect()
        };
        neighbor_positions
            .into_iter()
            .map(|pos| (self.point2d_to_index(pos.into()), 1.0))
            .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for MapAdapter<'_, '_> {
    fn dimensions(&self) -> Point {
        let map_size = self.cell.borrow_mut().dimensions();
        Point::new(map_size.0, map_size.1)
    }
}
