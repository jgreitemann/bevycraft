use crate::prelude::*;

#[derive(Copy, Clone, Component, Debug, Eq, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct MapBuilder {
    pub map_data: Vec<TileType>,
}

impl MapBuilder {
    pub fn new() -> Self {
        MapBuilder {
            map_data: [TileType::Wall, TileType::Floor, TileType::Floor]
                .iter()
                .cycle()
                .take((MAP_SIZE.0 * CHUNK_SIZE.0) as usize * (MAP_SIZE.1 * CHUNK_SIZE.1) as usize)
                .cloned()
                .collect(),
        }
    }
}
