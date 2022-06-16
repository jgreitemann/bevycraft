mod automata;
mod rooms;

use crate::prelude::*;
use rand::rngs::ThreadRng;

trait MapArchitect {
    fn architect(&mut self) -> MapBuilder;
}

pub struct MapBuilder {
    pub map_data: Vec<TileType>,
    pub player_start: Position,
    pub amulet_start: Position,
    pub spawn_locations: Vec<Position>,
}

impl MapBuilder {
    pub fn new() -> Self {
        // rooms::RoomsArchitect::default().architect()
        automata::CellularAutomataArchitect::default().architect()
    }

    fn can_enter_tile(&self, p: Point) -> bool {
        self.in_bounds(p) && !self.is_opaque(self.point2d_to_index(p))
    }

    fn try_idx(&self, p: Point) -> Option<usize> {
        if self.in_bounds(p) {
            Some(self.point2d_to_index(p))
        } else {
            None
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            MAP_WIDTH,
            MAP_HEIGHT,
            &[self.point2d_to_index(self.player_start.into())],
            self,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        self.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|(_, lhs_dist), (_, rhs_dist)| lhs_dist.partial_cmp(rhs_dist).unwrap())
                .unwrap()
                .0,
        )
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.try_idx(Point::new(x, y)) {
                self.map_data[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.try_idx(Point::new(x, y)) {
                self.map_data[idx] = TileType::Floor;
            }
        }
    }

    fn spawn_monsters(&self, rng: &mut ThreadRng) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        const MIN_DISTANCE: f32 = 10f32;

        let spawnable_tiles: Vec<_> = self
            .map_data
            .iter()
            .enumerate()
            .map(|(idx, tile_type)| (self.index_to_point2d(idx), tile_type))
            .filter(|&(_, &tile_type)| tile_type == TileType::Floor)
            .map(|(p, _)| p)
            .filter(|&p| {
                DistanceAlg::Pythagoras.distance2d(p, self.player_start.into()) > MIN_DISTANCE
            })
            .collect();

        use rand::prelude::*;
        spawnable_tiles
            .choose_multiple(rng, NUM_MONSTERS)
            .cloned()
            .collect()
    }
}

impl Default for MapBuilder {
    fn default() -> Self {
        MapBuilder {
            map_data: vec![TileType::Wall; MAP_WIDTH * MAP_HEIGHT],
            player_start: Default::default(),
            amulet_start: Default::default(),
            spawn_locations: Vec::new(),
        }
    }
}

impl BaseMap for MapBuilder {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map_data[idx] != TileType::Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let location = self.index_to_point2d(idx);
        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ]
        .into_iter()
        .filter_map(|delta| self.valid_exit(location, delta))
        .map(|idx| (idx, 1.0))
        .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for MapBuilder {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        (0..MAP_WIDTH as i32).contains(&pos.x) && (0..MAP_HEIGHT as i32).contains(&pos.y)
    }
}
