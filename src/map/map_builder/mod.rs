mod automata;
mod rooms;

use crate::prelude::*;

trait MapArchitect {
    fn architect(&mut self) -> MapBuilder;
}

pub struct MapBuilder {
    pub map_data: Vec<TileType>,
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
}

impl Default for MapBuilder {
    fn default() -> Self {
        MapBuilder {
            map_data: vec![TileType::Wall; MAP_WIDTH * MAP_HEIGHT],
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
